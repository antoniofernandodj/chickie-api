mod catalogo;
mod pedido;
mod marketing;
mod endereco_entrega;
mod loja_favorita;

pub use catalogo::{CatalogoUsecase, AtualizarProdutoRequest, CreateProdutoRequest};
pub use marketing::MarketingUsecase;
pub use endereco_entrega::ListarEnderecosEntregaPorLojaUsecase;
pub use loja_favorita::{
    AdicionarLojaFavoritaUsecase,
    RemoverLojaFavoritaUsecase,
    ListarLojasFavoritasUsecase,
    VerificarLojaFavoritaUsecase
};
