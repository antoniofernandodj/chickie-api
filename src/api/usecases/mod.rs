mod catalogo;
mod pedido;
mod marketing;

pub use catalogo::{CatalogoUsecase, AtualizarProdutoRequest, CreateProdutoRequest};
pub use marketing::MarketingUsecase;
