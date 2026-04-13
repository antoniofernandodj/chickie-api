use std::sync::Arc;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::models::{Usuario, ClasseUsuario};
use crate::ports::UsuarioRepositoryPort;

pub struct UsuarioService {
    repo: Arc<dyn UsuarioRepositoryPort>,
}

#[allow(dead_code)]
impl UsuarioService {
    pub fn new(repo: Arc<dyn UsuarioRepositoryPort>) -> Self { Self { repo } }
    pub async fn registrar(
        &self,
        nome: String,
        username: String,
        senha: String,
        email: String,
        celular: String,
        auth_method: String,
        classe: Option<String>,
    ) -> Result<Usuario, String> {

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
}