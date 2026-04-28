use sha2::{Sha256, Digest};
use std::collections::HashMap;
use tracing::info;

/// Armazena usuários e suas senhas (hash SHA-256)
/// Em produção: usar PostgreSQL ou outro banco de dados
pub struct UserStore {
    /// username -> SHA-256 hash da senha
    users: HashMap<String, String>,
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    /// Cria um store com usuários padrão para desenvolvimento
    pub fn new_with_defaults() -> Self {
        let mut store = Self::new();

        // Lê usuários do ambiente se disponíveis
        if let Ok(user) = std::env::var("SMTP_USERNAME") {
            if let Ok(pass) = std::env::var("SMTP_PASSWORD") {
                info!("Adicionando usuário do ambiente: {}", user);
                store.add_user(&user, &pass);
            }
        }

        // Usuários padrão para desenvolvimento
        store.add_user("admin@localhost", "admin123");
        store.add_user("test@localhost", "test123");

        info!("UserStore inicializado com {} usuário(s)", store.users.len());
        store
    }

    /// Adiciona um usuário com senha em texto puro (armazenada como hash)
    pub fn add_user(&mut self, username: &str, password: &str) {
        let hash = hash_password(password);
        self.users.insert(username.to_lowercase(), hash);
    }

    /// Adiciona um usuário já com o hash da senha
    pub fn add_user_hash(&mut self, username: &str, password_hash: &str) {
        self.users.insert(username.to_lowercase(), password_hash.to_string());
    }

    /// Verifica as credenciais de um usuário
    pub fn verify(&self, username: &str, password: &str) -> bool {
        let hash = hash_password(password);
        self.users
            .get(&username.to_lowercase())
            .map(|stored_hash| stored_hash == &hash)
            .unwrap_or(false)
    }

    /// Verifica se um usuário existe
    pub fn user_exists(&self, username: &str) -> bool {
        self.users.contains_key(&username.to_lowercase())
    }

    /// Remove um usuário
    pub fn remove_user(&mut self, username: &str) {
        self.users.remove(&username.to_lowercase());
    }

    /// Lista todos os usuários (sem senhas)
    pub fn list_users(&self) -> Vec<String> {
        self.users.keys().cloned().collect()
    }
}

/// Gera o hash SHA-256 da senha
fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_verify_user() {
        let mut store = UserStore::new();
        store.add_user("user@example.com", "senha123");

        assert!(store.verify("user@example.com", "senha123"));
        assert!(!store.verify("user@example.com", "errada"));
        assert!(!store.verify("outro@example.com", "senha123"));
    }

    #[test]
    fn test_case_insensitive_username() {
        let mut store = UserStore::new();
        store.add_user("User@Example.COM", "senha");

        assert!(store.verify("user@example.com", "senha"));
        assert!(store.verify("USER@EXAMPLE.COM", "senha"));
    }

    #[test]
    fn test_remove_user() {
        let mut store = UserStore::new();
        store.add_user("user@example.com", "senha");
        store.remove_user("user@example.com");

        assert!(!store.verify("user@example.com", "senha"));
    }
}
