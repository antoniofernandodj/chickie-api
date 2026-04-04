// Repository trait definition
use std::sync::Arc;

use uuid::Uuid;

#[async_trait::async_trait]
pub trait Repository<T> {
    async fn criar(&self, item: &T) -> Result<Uuid, String>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<T>, String>;
    async fn atualizar(&self, item: T) -> Result<(), String>;
    async fn deletar(&self, uuid: Uuid) -> Result<(), String>;
    async fn listar_todos(&self) -> Result<Vec<T>, String>;
    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<T>, String>;
    fn table_name(&self) -> String;
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

// Re-export all repository structs
pub use usuario_repository::UsuarioRepository;
pub use loja_repository::LojaRepository;
pub use cliente_repository::ClienteRepository;
pub use produto_repository::ProdutoRepository;
pub use categoria_produtos_repository::CategoriaProdutosRepository;
pub use pedido_repository::PedidoRepository;
pub use adicional_repository::AdicionalRepository;
pub use ingrediente_repository::IngredienteRepository;
pub use endereco_loja_repository::EnderecoLojaRepository;
pub use entregador_repository::EntregadorRepository;
pub use funcionario_repository::FuncionarioRepository;
pub use avaliacao_de_loja_repository::AvaliacaoDeLojaRepository;
pub use avaliacao_de_produto_repository::AvaliacaoDeProdutoRepository;
pub use cupom_repository::CupomRepository;
pub use uso_cupom_repository::UsoCupomRepository;
pub use promocao_repository::PromocaoRepository;
pub use horario_funcionamento_repository::HorarioFuncionamentoRepository;
pub use configuracao_pedidos_loja_repository::ConfiguracaoPedidosLojaRepository;
pub use parte_de_item_pedido_repository::ParteDeItemPedidoRepository;
pub use endereco_usuario_repository::EnderecoUsuarioRepository;
pub use endereco_entrega_repository::EnderecoEntregaRepository;
