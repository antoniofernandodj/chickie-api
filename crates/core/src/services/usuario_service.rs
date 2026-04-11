use std::sync::Arc;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::models::{Usuario, ClasseUsuario};
use crate::repositories::{UsuarioRepository, Repository as _};

pub struct UsuarioService {
    repo: Arc<UsuarioRepository>,
}

#[allow(dead_code)]
impl UsuarioService {
    pub fn new(repo: Arc<UsuarioRepository>) -> Self { Self { repo } }
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
        let mut usuario = if let Some(u) = self.repo.buscar_por_email(&identifier).await? {
            u
        } else if let Some(u) = self.repo.buscar_por_username(&identifier).await? {
            u
        } else if let Some(u) = self.repo.buscar_por_celular(&identifier).await? {
            u
        } else {
            return Err("Usuário não encontrado".to_string());
        };

        // 2. Verifica se a senha enviada condiz com o hash do banco
        let argon2 = Argon2::default();
        let parsed_hash = argon2::password_hash::PasswordHash::new(&usuario.senha_hash)
            .map_err(|e| format!("Erro ao processar senha: {}", e))?;
        let senha_valida = argon2
            .verify_password(senha_plana.as_bytes(), &parsed_hash)
            .is_ok();

        if !senha_valida {
            return Err("Senha incorreta".to_string());
        }

        // 3. Se é o primeiro acesso, marcar como true
        if !usuario.passou_pelo_primeiro_acesso {
            self.repo.marcar_primeiro_acesso(usuario.uuid).await?;
            usuario.passou_pelo_primeiro_acesso = true;
        }

        // 4. Retorna o usuário se tudo estiver correto
        Ok(usuario)
    }

    pub async fn listar(&self) -> Result<Vec<Usuario>, String> {
        self.repo.listar_todos().await
    }

    pub async fn verificar_email_disponivel(&self, email: &str) -> Result<bool, String> {
        let existente = self.repo.buscar_por_email(email).await?;
        Ok(existente.is_none())
    }

    pub async fn verificar_username_disponivel(&self, username: &str) -> Result<bool, String> {
        let existente = self.repo.buscar_por_username(username).await?;
        Ok(existente.is_none())
    }
}