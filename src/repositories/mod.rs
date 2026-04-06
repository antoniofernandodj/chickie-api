use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, ActiveModelBehavior, ModelTrait, DbErr, Set};
use uuid::Uuid;
use sea_orm::prelude::Uuid as SeaUuid;

#[async_trait::async_trait]
pub trait Repository<E>: Send + Sync
where
    E: EntityTrait + Sync,
    E::Model: Send + Sync + Into<E::ActiveModel>,
    E::ActiveModel: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send,
{
    /// Returns the database connection
    fn db(&self) -> &DatabaseConnection;

    /// Returns the entity being operated on
    fn entity(&self) -> E;

    /// Default: Find by UUID
    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<E::Model>, String> {
        self.entity().find_by_id(SeaUuid::from(uuid))
            .one(self.db())
            .await
            .map_err(|e| e.to_string())
    }

    /// Default: List all
    async fn listar_todos(&self) -> Result<Vec<E::Model>, String> {
        self.entity().find()
            .all(self.db())
            .await
            .map_err(|e| e.to_string())
    }

    /// Default: Delete by UUID
    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let model = self.buscar_por_uuid(uuid).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("{} não encontrad{}", self.entity_name(), self.entity_gender_suffix()))?;

        model.delete(self.db())
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    /// Helper for Portuguese gender in error messages (defaults to "o")
    fn entity_name(&self) -> &'static str { "Entidade" }
    fn entity_gender_suffix(&self) -> &'static str { "o" }

    /// Create a new record from a Model reference (converts to ActiveModel internally)
    async fn criar(&self, model: &E::Model) -> Result<E::Model, String> {
        let active: E::ActiveModel = model.clone().into();
        active.insert(self.db())
            .await
            .map_err(|e| e.to_string())
    }

    /// Update an existing record from a Model reference (converts to ActiveModel internally)
    async fn atualizar(&self, model: E::Model) -> Result<E::Model, String> {
        let active: E::ActiveModel = model.clone().into();
        active.update(self.db())
            .await
            .map_err(|e| e.to_string())
    }

    /// List all by loja_uuid (for entities that belong to a store)
    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<E::Model>, String>;
}

// Repository modules
pub mod usuario_repository;
pub mod loja_repository;
pub mod cliente_repository;
pub mod produto_repository;
pub mod categoria_produtos_repository;
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

// Re-export all repository structs
pub use usuario_repository::UsuarioRepository;
pub use loja_repository::LojaRepository;
pub use cliente_repository::ClienteRepository;
pub use produto_repository::ProdutoRepository;
pub use categoria_produtos_repository::CategoriaProdutosRepository;
pub use pedido_repository::PedidoRepository;
pub use adicional_repository::AdicionalRepository;
pub use ingrediente_repository::IngredienteRepository;
// pub use endereco_loja_repository::EnderecoLojaRepository;
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
