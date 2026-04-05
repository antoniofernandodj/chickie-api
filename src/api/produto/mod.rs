mod atualizar_produto;
mod buscar_produto;
mod criar_produto;
mod deletar_produto;
mod listar_produtos;
mod listar_produtos_por_categoria;
mod subir_imagem;

pub use atualizar_produto::atualizar_produto;
pub use buscar_produto::buscar_produto_por_uuid;
pub use criar_produto::criar_produto;
pub use deletar_produto::deletar_produto;
pub use listar_produtos::listar_produtos;
pub use listar_produtos_por_categoria::listar_produtos_por_categoria;
pub use subir_imagem::subir_imagem_produto;