// Repository trait definition
// use std::sync::Arc;

use sqlx::postgres::{PgPool, PgRow};
use uuid::Uuid;
use crate::models::Model;

#[async_trait::async_trait]
pub trait Repository<T>: Send + Sync
where
    T: Model + Send + Sync + Unpin + for<'r> sqlx::FromRow<'r, PgRow>,
{
    fn table_name(&self) -> &'static str;
    fn entity_name(&self) -> &'static str;
    fn pool(&self) -> &PgPool;

    // Default implementation - reusable across all repos
    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<T>, String> {
        let query = format!("SELECT * FROM {} WHERE uuid = $1", self.table_name());
        sqlx::query_as::<_, T>(&query)
            .bind(uuid)
            .fetch_optional(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

    // Default implementation - reusable across all repos
    async fn listar_todos(&self) -> Result<Vec<T>, String> {
        let query = format!("SELECT * FROM {}", self.table_name());
        sqlx::query_as::<_, T>(&query)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

    // Default implementation - reusable across all repos
    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let query = format!("DELETE FROM {} WHERE uuid = $1", self.table_name());
        let result = sqlx::query(&query)
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err(format!("{} não encontrad{}", self.entity_name(), self.entity_gender_suffix()))
        } else {
            Ok(())
        }
    }

    // Helper for Portuguese gender in error messages (defaults to "o")
    fn entity_gender_suffix(&self) -> &'static str { "o" }

    // These remain abstract - column-specific
    async fn criar(&self, item: &T) -> Result<Uuid, String>;
    async fn atualizar(&self, item: T) -> Result<(), String>;
    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<T>, String>;
}

// Repository modules
pub mod usuario_repository;
pub mod loja_repository;
pub mod cliente_repository;
pub mod produto_repository;
pub mod categoria_produtos_repository;
pub mod categoria_ordem_repository;
pub mod pedido_repository;
pub mod adicional_repository;
pub mod ingrediente_repository;
pub mod endereco_loja_repository;
pub mod entregador_repository;
pub mod funcionario_repository;
pub mod avaliacao_de_loja_repository;
pub mod avaliacao_de_produto_repository;
pub mod cupom_repository;
pub mod uso_cupom_repository;
pub mod promocao_repository;
pub mod horario_funcionamento_repository;
pub mod configuracao_pedidos_loja_repository;
pub mod parte_de_item_pedido_repository;
pub mod endereco_usuario_repository;
pub mod endereco_entrega_repository;
pub mod loja_favorita_repository;
pub mod chat_repository;
pub mod pre_cadastro_repository;
pub mod whatsapp_repository;

// Re-export all repository structs
pub use usuario_repository::UsuarioRepository;
pub use loja_repository::LojaRepository;
pub use cliente_repository::ClienteRepository;
pub use produto_repository::ProdutoRepository;
pub use categoria_produtos_repository::CategoriaProdutosRepository;
pub use categoria_ordem_repository::CategoriaOrdemRepository;
pub use pedido_repository::{PedidoRepository, PedidoComEntregador};
pub use adicional_repository::AdicionalRepository;
pub use ingrediente_repository::IngredienteRepository;
pub use endereco_loja_repository::EnderecoLojaRepository;
pub use entregador_repository::EntregadorRepository;
pub use funcionario_repository::FuncionarioRepository;
pub use avaliacao_de_loja_repository::AvaliacaoDeLojaRepository;
pub use avaliacao_de_produto_repository::AvaliacaoDeProdutoRepository;
pub use cupom_repository::CupomRepository;
// pub use uso_cupom_repository::UsoCupomRepository;
pub use promocao_repository::PromocaoRepository;
pub use horario_funcionamento_repository::HorarioFuncionamentoRepository;
pub use configuracao_pedidos_loja_repository::ConfiguracaoPedidosLojaRepository;
// pub use parte_de_item_pedido_repository::ParteDeItemPedidoRepository;
pub use endereco_usuario_repository::EnderecoUsuarioRepository;
pub use endereco_entrega_repository::EnderecoEntregaRepository;
pub use loja_favorita_repository::LojaFavoritaRepository;
pub use chat_repository::ChatRepository;
pub use pre_cadastro_repository::PreCadastroRepository;
pub use whatsapp_repository::WhatsAppRepository;
