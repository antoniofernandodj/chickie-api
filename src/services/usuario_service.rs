use std::sync::Arc;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::entities::usuario::Model;
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
        telefone: String,
        auth_method: String,
        classe: Option<String>,
    ) -> Result<Model, String> {

        // Dentro do registrar...
        let senha_hash = hash(senha, DEFAULT_COST)
            .map_err(|e| e.to_string())?;

        // Parse classe: default = "cliente"
        let classe_str = classe.as_deref().unwrap_or("cliente");
        let classe_enum = crate::models::ClasseUsuario::from_str(classe_str)
            .map_err(|e| format!("Classe de usuário inválida: {}", e))?;

        let usuario = Model {
            uuid: uuid::Uuid::new_v4(),
            nome,
            username,
            email,
            senha_hash,
            celular: telefone,
            telefone: None,
            classe: format!("{:?}", classe_enum),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
            modo_de_cadastro: auth_method,
            criado_em: chrono::Utc::now(),
            atualizado_em: chrono::Utc::now(),
        };

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
        email: String,
        senha_plana: String,
    ) -> Result<Model, String> {

        // 1. Busca o usuário pelo email
        let usuario: Model = self.repo
            .buscar_por_email(&email)
            .await?
            .ok_or_else(|| "Usuário não encontrado".to_string())?;

        // 2. Verifica se a senha enviada condiz com o hash do banco
        // O campo 'password_hash' deve existir no seu model Usuario
        let senha_valida = verify(senha_plana, &usuario.senha_hash)
            .map_err(|e| format!("Erro ao processar senha: {}", e))?;

        if !senha_valida {
            return Err("Senha incorreta".to_string());
        }

        // 3. Retorna o usuário se tudo estiver correto
        Ok(usuario)
    }

    pub async fn listar(&self) -> Result<Vec<Model>, String> {
        self.repo.listar_todos().await
    }
}