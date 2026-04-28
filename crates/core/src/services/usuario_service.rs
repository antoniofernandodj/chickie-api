use std::sync::Arc;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::models::{Usuario, ClasseUsuario};
use crate::ports::{EmailServicePort, PreCadastroPort, UsuarioRepositoryPort};

fn validar_cpf(cpf: &str) -> bool {
    // Deve ter exatamente 11 dígitos numéricos
    if cpf.len() != 11 || !cpf.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    // Rejeita sequências repetidas (000...0, 111...1, ...)
    if cpf.chars().all(|c| c == cpf.chars().next().unwrap()) {
        return false;
    }
    let digits: Vec<u32> = cpf.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // Primeiro dígito verificador
    let soma: u32 = digits[..9].iter().enumerate().map(|(i, &d)| d * (10 - i as u32)).sum();
    let resto = soma % 11;
    let d1 = if resto < 2 { 0 } else { 11 - resto };
    if digits[9] != d1 {
        return false;
    }
    // Segundo dígito verificador
    let soma: u32 = digits[..10].iter().enumerate().map(|(i, &d)| d * (11 - i as u32)).sum();
    let resto = soma % 11;
    let d2 = if resto < 2 { 0 } else { 11 - resto };
    digits[10] == d2
}

pub struct UsuarioService {
    repo: Arc<dyn UsuarioRepositoryPort>,
    pre_cadastro: Arc<dyn PreCadastroPort>,
    email: Arc<dyn EmailServicePort>,
}

#[allow(dead_code)]
impl UsuarioService {
    pub fn new(
        repo: Arc<dyn UsuarioRepositoryPort>,
        pre_cadastro: Arc<dyn PreCadastroPort>,
        email: Arc<dyn EmailServicePort>,
    ) -> Self {
        Self { repo, pre_cadastro, email }
    }
    pub async fn registrar(
        &self,
        nome: String,
        username: String,
        senha: String,
        email: String,
        celular: String,
        cpf: String,
        auth_method: String,
        classe: Option<String>,
    ) -> Result<Usuario, String> {

        // Valida CPF (obrigatório para o fluxo de pagamento PIX)
        if !validar_cpf(&cpf) {
            return Err("CPF inválido. Informe os 11 dígitos sem pontuação ou verifique os dígitos verificadores.".to_string());
        }

        // Hash the password using argon2id
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let senha_hash = argon2
            .hash_password(senha.as_bytes(), &salt)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?
            .to_string();

        // Parse classe: default = "cliente"
        let classe_str = classe.as_deref().unwrap_or("cliente");
        let classe = ClasseUsuario::from_str(classe_str)
            .map_err(|e| format!("Classe de usuário inválida: {}", e))?;

        let usuario = Usuario::new(
            nome,
            username,
            email,
            senha_hash,
            celular,
            cpf,
            auth_method,
            classe
        );

        self.repo.criar(&usuario).await?;

        // Exemplo de verificação pós-criação
        if let Some(u) = self
            .repo
            .buscar_por_email(&usuario.email)
            .await? {
                tracing::info!("Usuário confirmado no banco: {:?} (classe: {})", u.nome, u.classe);
            }

        Ok(usuario)
    }

    pub async fn autenticar(
        &self,
        identifier: String,
        senha_plana: String,
    ) -> Result<Usuario, String> {

        // 1. Busca o usuário por email, username ou celular
        let mut usuario = if let Some(u) = self.repo.buscar_por_email(&identifier).await.map_err(|e| e.to_string())? {
            u
        } else if let Some(u) = self.repo.buscar_por_username(&identifier).await.map_err(|e| e.to_string())? {
            u
        } else if let Some(u) = self.repo.buscar_por_celular(&identifier).await.map_err(|e| e.to_string())? {
            u
        } else {
            return Err("Usuário não encontrado".to_string());
        };

        // 2. Verifica soft delete e status ativo
        if usuario.esta_deletado() {
            return Err("Usuário deletado. Não é possível fazer login.".to_string());
        }

        if usuario.esta_marcado_para_remocao() {
            return Err("Usuário marcado para remoção. Login bloqueado.".to_string());
        }

        if !usuario.ativo {
            return Err("Usuário desativado. Contate o suporte.".to_string());
        }

        if usuario.esta_bloqueado() {
            return Err("Usuário bloqueado. Contate o suporte.".to_string());
        }

        // 3. Verifica se a senha enviada condiz com o hash do banco
        let argon2 = Argon2::default();
        let parsed_hash = argon2::password_hash::PasswordHash::new(&usuario.senha_hash)
            .map_err(|e| format!("Erro ao processar senha: {}", e))?;
        let senha_valida = argon2
            .verify_password(senha_plana.as_bytes(), &parsed_hash)
            .is_ok();

        if !senha_valida {
            return Err("Senha incorreta".to_string());
        }

        // 4. Se é o primeiro acesso, marcar como true
        if !usuario.passou_pelo_primeiro_acesso {
            self.repo.marcar_primeiro_acesso(usuario.uuid).await.map_err(|e| e.to_string())?;
            usuario.passou_pelo_primeiro_acesso = true;
        }

        // 5. Retorna o usuário se tudo estiver correto
        Ok(usuario)
    }

    pub async fn listar(&self) -> Result<Vec<Usuario>, String> {
        self.repo.listar_todos().await.map_err(|e| e.to_string())
    }

    pub async fn verificar_email_disponivel(&self, email: &str) -> Result<bool, String> {
        let existente = self.repo.buscar_por_email(email).await?;
        Ok(existente.is_none())
    }

    pub async fn verificar_username_disponivel(&self, username: &str) -> Result<bool, String> {
        let existente = self.repo.buscar_por_username(username).await?;
        Ok(existente.is_none())
    }

    pub async fn verificar_celular_disponivel(&self, celular: &str) -> Result<bool, String> {
        let existente = self.repo.buscar_por_celular(celular).await?;
        Ok(existente.is_none())
    }

    // ===========================================================================
    // Soft Delete
    // ===========================================================================

    /// Marca o usuário para remoção. Após 30 dias, o scheduler marcará como deletado=true.
    pub async fn marcar_para_remocao(&self, uuid: uuid::Uuid) -> Result<(), String> {
        // Verifica se o usuário existe e não está deletado
        let usuario = self.repo.buscar_por_uuid(uuid).await
            .map_err(|e| e.to_string())?
            .ok_or("Usuário não encontrado")?;

        if usuario.esta_deletado() {
            return Err("Usuário já está permanentemente deletado".to_string());
        }

        if usuario.esta_marcado_para_remocao() {
            return Err("Usuário já está marcado para remoção".to_string());
        }

        self.repo.marcar_para_remocao(uuid).await.map_err(|e| e.to_string())
    }

    /// Desmarca a remoção pendente
    pub async fn desmarcar_remocao(&self, uuid: uuid::Uuid) -> Result<(), String> {
        self.repo.desmarcar_remocao(uuid).await.map_err(|e| e.to_string())
    }

    /// Alterna o status ativo do usuário (bloqueio/desbloqueio)
    pub async fn alternar_ativo(&self, uuid: uuid::Uuid, ativo: bool) -> Result<(), String> {
        let usuario = self.repo.buscar_por_uuid(uuid).await
            .map_err(|e| e.to_string())?
            .ok_or("Usuário não encontrado")?;

        if usuario.esta_deletado() {
            return Err("Não é possível alterar status de usuário deletado".to_string());
        }

        self.repo.alterar_ativo(uuid, ativo).await.map_err(|e| e.to_string())
    }

    /// Alterna o status bloqueado do usuário (toggle)
    /// Retorna o novo status de bloqueio
    pub async fn toggle_bloqueado(&self, uuid: uuid::Uuid) -> Result<bool, String> {
        let usuario = self.repo.buscar_por_uuid(uuid).await
            .map_err(|e| e.to_string())?
            .ok_or("Usuário não encontrado")?;

        if usuario.esta_deletado() {
            return Err("Não é possível bloquear usuário deletado".to_string());
        }

        self.repo.toggle_bloqueado(uuid).await.map_err(|e| e.to_string())
    }

    /// Deleta permanentemente todos os usuários marcados para remoção há mais de 30 dias.
    /// Retorna o número de usuários deletados.
    pub async fn deletar_pendentes_antigos(&self) -> Result<u64, String> {
        let thirty_days_ago = chrono::Utc::now() - chrono::Duration::days(30);
        self.repo.deletar_pendentes_antigos(thirty_days_ago).await.map_err(|e| e.to_string())
    }

    // ===========================================================================
    // Fluxo de verificação de email no cadastro
    // ===========================================================================

    /// Valida os dados do usuário, armazena um pré-cadastro em cache e envia o email
    /// de verificação. O usuário só é criado em banco após confirmar o link.
    pub async fn iniciar_cadastro(
        &self,
        nome: String,
        username: String,
        senha: String,
        email: String,
        celular: String,
        cpf: String,
        auth_method: String,
        classe: Option<String>,
    ) -> Result<(), String> {
        if !validar_cpf(&cpf) {
            return Err("CPF inválido. Informe os 11 dígitos sem pontuação ou verifique os dígitos verificadores.".into());
        }

        if self.repo.buscar_por_email(&email).await.map_err(|e| e.to_string())?.is_some() {
            return Err("Email já cadastrado.".into());
        }
        if self.repo.buscar_por_username(&username).await.map_err(|e| e.to_string())?.is_some() {
            return Err("Username já cadastrado.".into());
        }
        if self.repo.buscar_por_celular(&celular).await.map_err(|e| e.to_string())?.is_some() {
            return Err("Celular já cadastrado.".into());
        }

        let salt = SaltString::generate(&mut rand::thread_rng());
        let senha_hash = Argon2::default()
            .hash_password(senha.as_bytes(), &salt)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?
            .to_string();

        let classe_str = classe.as_deref().unwrap_or("cliente");
        ClasseUsuario::from_str(classe_str)
            .map_err(|e| format!("Classe de usuário inválida: {}", e))?;

        let token = uuid::Uuid::new_v4().to_string().replace('-', "");

        let dados = serde_json::json!({
            "nome": nome,
            "username": username,
            "senha_hash": senha_hash,
            "email": email,
            "celular": celular,
            "cpf": cpf,
            "auth_method": auth_method,
            "classe": classe_str,
        });

        let expira_em = chrono::Utc::now() + chrono::Duration::hours(1);

        self.pre_cadastro
            .salvar(&token, dados, expira_em)
            .await
            .map_err(|e| e.to_string())?;

        self.email
            .enviar_verificacao_cadastro(&email, &nome, &token)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Valida o token de verificação, cria o usuário em banco e remove o pré-cadastro do cache.
    pub async fn confirmar_cadastro(&self, token: &str) -> Result<Usuario, String> {
        let dados = self.pre_cadastro
            .buscar(token)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Token inválido ou expirado. Faça o cadastro novamente.".to_string())?;

        let get_str = |key: &str| -> String {
            dados[key].as_str().unwrap_or_default().to_string()
        };

        let classe = ClasseUsuario::from_str(&get_str("classe"))
            .map_err(|e| format!("Classe inválida no pré-cadastro: {}", e))?;

        let usuario = Usuario::new(
            get_str("nome"),
            get_str("username"),
            get_str("email"),
            get_str("senha_hash"),
            get_str("celular"),
            get_str("cpf"),
            get_str("auth_method"),
            classe,
        );

        self.repo.criar(&usuario).await.map_err(|e| e.to_string())?;
        self.pre_cadastro.remover(token).await.map_err(|e| e.to_string())?;

        tracing::info!("Cadastro confirmado: {} ({})", usuario.nome, usuario.email);
        Ok(usuario)
    }
}