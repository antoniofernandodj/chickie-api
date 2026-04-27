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
mod ingrediente;
mod horario;
mod endereco_loja;
mod config_pedido;
mod funcionario;
mod entregador;
mod openapi;

use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn ok_handler() -> impl IntoResponse {
    Json(json!({"msg": "ok"}))
}

// Re-export usecases from the top-level module
// pub use chickie_core::usecases::{
//     CatalogoUsecase,
//     MarketingUsecase
// };

pub use routers::api_routes;
pub use openapi::swagger_router;

pub use auth::{
    auth_middleware,
    optional_auth_middleware,
    create_jwt,
    OwnerPermission,
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
    buscar_pedido_por_codigo,
    listar_por_loja,
    buscar_pedido_com_entrega,
    atualizar_status,
    avancar_status,
    listar_meus_pedidos,
    atribuir_entregador,
    remover_entregador,
    buscar_pedido_com_entregador,
};

pub use usuario::{
    criar_usuario,
    listar_usuarios, // buscar_usuario
    me,
    verificar_email,
    verificar_username,
    verificar_celular,
    marcar_usuario_remocao,
    desmarcar_usuario_remocao,
    alternar_usuario_ativo,
    toggle_usuario_bloqueado,
};

pub use loja::{
    criar_loja,
    listar_lojas,
    buscar_loja,
    adicionar_funcionario,
    adicionar_entregador,
    adicionar_cliente,
    listar_lojas_admin,
    listar_minhas_lojas,
    pesquisar_lojas,
    buscar_loja_por_slug,
    verificar_slug_disponivel,
    marcar_loja_remocao,
    desmarcar_loja_remocao,
    alternar_loja_ativo,
    toggle_loja_bloqueado,
};

pub use produto::{
    criar_produto,
    listar_produtos,
    listar_produtos_por_categoria,
    buscar_produto_por_uuid,
    deletar_produto,
    atualizar_produto,
    subir_imagem_produto,
    atualizar_disponibilidade_produto
};

pub use cupom::{
    criar_cupom,
    validar_cupom,
    listar_cupons,
    atualizar_cupom,
    deletar_cupom,
    buscar_cupom,
    listar_todos_cupons,
    criar_cupom_generico,
    atualizar_status_cupom
};

pub use wipe::wipe_database;

pub use marketing::{
    avaliar_loja,
    avaliar_produto,
    criar_promocao,
    listar_promocoes,
    atualizar_promocao,
    deletar_promocao,
    listar_avaliacoes_loja,
    buscar_avaliacao_loja,
    atualizar_avaliacao_loja,
    deletar_avaliacao_loja,
    listar_avaliacoes_produto_por_loja,
    listar_avaliacoes_produto_por_produto,
    buscar_avaliacao_produto,
    atualizar_avaliacao_produto,
    deletar_avaliacao_produto
};

pub use ingrediente::{
    criar_ingrediente,
    listar_ingredientes,
    atualizar_ingrediente,
    deletar_ingrediente
};

pub use horario::{
    listar_horarios,
    criar_ou_atualizar_horario,
    definir_ativo,
    deletar_horario_dia
};

pub use endereco_loja::{
    listar_enderecos_loja,
    criar_endereco_loja,
    atualizar_endereco_loja,
    deletar_endereco_loja
};

pub use config_pedido::{
    buscar_config_pedido,
    salvar_config_pedido
};

pub use funcionario::{
    atualizar_funcionario,
    listar_funcionarios,
    funcionario_trocar_email_senha
};

pub use entregador::{
    atualizar_entregador,
    atualizar_disponibilidade_entregador,
    listar_entregadores,
    entregador_trocar_email_senha
};

pub use catalogo::{
    criar_adicional,
    atualizar_adicional,
    deletar_adicional,
    atualizar_disponibilidade,
    criar_categoria,
    criar_categoria_global,
    atualizar_categoria,
    atualizar_categoria_global,
    deletar_categoria,
    deletar_categoria_global,
    listar_adicionais,
    listar_adicionais_disponiveis,
    listar_categorias,
    listar_categorias_globais,
    reordenar_categorias,
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
