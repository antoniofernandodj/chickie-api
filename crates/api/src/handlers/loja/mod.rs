mod criar_loja;
mod listar_lojas;
mod adicionar_funcionario;
mod adicionar_entregador;
mod listar_lojas_admin;
mod adicionar_cliente;
mod listar_minhas_lojas;
mod pesquisar_lojas;
mod buscar_loja;
mod buscar_por_slug;
mod verificar_slug_disponivel;
mod soft_delete;

pub use criar_loja::criar_loja;
pub use listar_lojas::listar_lojas;
pub use adicionar_funcionario::adicionar_funcionario;
pub use adicionar_entregador::adicionar_entregador;
pub use listar_lojas_admin::listar_lojas_admin;
pub use adicionar_cliente::adicionar_cliente;
pub use listar_minhas_lojas::listar_minhas_lojas;
pub use pesquisar_lojas::pesquisar_lojas;
pub use buscar_loja::buscar_loja;
pub use buscar_por_slug::buscar_loja_por_slug;
pub use verificar_slug_disponivel::verificar_slug_disponivel;
pub use soft_delete::{
    marcar_loja_remocao,
    desmarcar_loja_remocao,
    alternar_loja_ativo,
};