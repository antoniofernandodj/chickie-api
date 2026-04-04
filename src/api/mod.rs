mod pedido;
mod usuario;
mod state;
mod dto;
mod loja;
mod produto;
mod cupom;
mod routers;
mod auth;
mod wipe;
mod marketing;
mod catalogo;
mod endereco_entrega;
mod endereco_usuario;

// Re-export usecases from the top-level module
pub use crate::usecases::{
    CatalogoUsecase,
    MarketingUsecase
};

pub use routers::api_routes;

pub use auth::{
    auth_middleware,
    create_jwt
};

pub use state::{AppState};

pub use dto::{
    CreateUsuarioRequest,
    CreateLojaRequest,
    CreatePedidoRequest,
    Claims
};

pub use pedido::{
    criar_pedido,
    listar_pedidos,
    buscar_pedido, // buscar_pedido_por_usuario, buscar_pedido_por_loja
};

pub use usuario::{
    criar_usuario,
    listar_usuarios, // buscar_usuario
};

pub use loja::{
    criar_loja,
    listar_lojas, // buscar_loja
    adicionar_funcionario,
    adicionar_entregador,
    listar_lojas_admin
};

pub use produto::{
    criar_produto,
    listar_produtos, // buscar_produto
    atualizar_produto
};

pub use cupom::{
    criar_cupom,
    validar_cupom, // listar_cupons, buscar_cupom
};

pub use wipe::wipe_database;

pub use marketing::{
    avaliar_loja,
    avaliar_produto
};

pub use catalogo::{
    criar_adicional,
    criar_categoria
};

pub use endereco_entrega::{
    criar_para_pedido,
    buscar_por_pedido
};

pub use endereco_usuario::{
    criar_endereco,
    listar_enderecos,
    buscar_endereco,
    atualizar_endereco,
    deletar_endereco
};