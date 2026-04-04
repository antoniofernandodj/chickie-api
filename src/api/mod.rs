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
mod loja_favorita;

// Re-export usecases from the top-level module
// pub use crate::usecases::{
//     CatalogoUsecase,
//     MarketingUsecase
// };

pub use routers::api_routes;

pub use auth::{
    auth_middleware,
    create_jwt
};

pub use state::{AppState};

pub use dto::{
    CreateUsuarioRequest,
    CreateLojaRequest,
    Claims
};

pub use pedido::{
    criar_pedido,
    listar_pedidos,
    buscar_pedido,
    listar_por_loja,
    buscar_pedido_com_entrega,
    atualizar_status
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
    adicionar_cliente,
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
    listar_cupons
};

pub use wipe::wipe_database;

pub use marketing::{
    avaliar_loja,
    avaliar_produto,
    criar_promocao,
    listar_promocoes,
    atualizar_promocao,
    deletar_promocao
};

pub use catalogo::{
    criar_adicional,
    criar_categoria,
    listar_adicionais,
    listar_adicionais_disponiveis,
    marcar_indisponivel
};

pub use endereco_entrega::{
    criar_para_pedido,
    buscar_por_pedido,
    listar_por_loja as listar_enderecos_por_loja
};

pub use endereco_usuario::{
    criar_endereco,
    listar_enderecos,
    buscar_endereco,
    atualizar_endereco,
    deletar_endereco
};

pub use loja_favorita::{
    adicionar_favorita,
    remover_favorita,
    listar_minhas_favoritas,
    verificar_favorita
};