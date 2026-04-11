/// Domain errors — independentes de HTTP, database, ou qualquer infraestrutura.
/// Services e UseCases retornam `Result<T, DomainError>`.
#[derive(Debug, Clone)]
pub enum DomainError {
    /// Entidade não encontrada
    NotFound {
        entity: &'static str,
        id: String,
    },
    /// Regra de negócio violada
    BusinessRule(String),
    /// Dados inválidos para operação
    Validation(String),
    /// Conflito (ex: slug/email já existe)
    Conflict {
        entity: &'static str,
        field: String,
    },
    /// Estado inválido para transição
    InvalidState {
        current: String,
        attempted: String,
        allowed: Vec<String>,
    },
    /// Erro interno inesperado
    Internal(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::NotFound { entity, id } => {
                write!(f, "{} not found: {}", entity, id)
            }
            DomainError::BusinessRule(msg) => write!(f, "Business rule violated: {}", msg),
            DomainError::Validation(msg) => write!(f, "Validation error: {}", msg),
            DomainError::Conflict { entity, field } => {
                write!(f, "{} conflict on field '{}'", entity, field)
            }
            DomainError::InvalidState { current, attempted, allowed } => {
                write!(
                    f,
                    "Invalid state transition: {} -> {}. Allowed: {:?}",
                    current, attempted, allowed
                )
            }
            DomainError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

/// Convenience conversions
impl From<String> for DomainError {
    fn from(s: String) -> Self {
        DomainError::Internal(s)
    }
}

impl From<&str> for DomainError {
    fn from(s: &str) -> Self {
        DomainError::Internal(s.to_string())
    }
}

/// Result type alias for domain operations
pub type DomainResult<T> = Result<T, DomainError>;
