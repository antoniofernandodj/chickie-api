mod usuario_service;
mod loja_service;
mod catalogo_service;
mod pedido_service;
mod marketing_service;
mod endereco_usuario_service;
mod endereco_entrega_service;
mod loja_favorita_service;

pub use usuario_service::UsuarioService;
pub use loja_service::LojaService;
pub use catalogo_service::CatalogoService;
pub use pedido_service::{PedidoService, PedidoComEntrega};
pub use marketing_service::MarketingService;
pub use endereco_usuario_service::EnderecoUsuarioService;
pub use endereco_entrega_service::EnderecoEntregaService;
pub use loja_favorita_service::LojaFavoritaService;
