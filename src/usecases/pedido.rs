

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::{Produto, Usuario}, services::{CatalogoService, PedidoService}};

pub struct PedidoUsecase {
    pub pedido_service: Arc<PedidoService>,
    pub loja_uuid: Uuid,
    pub usuario: Usuario,
}


impl PedidoUsecase {
    pub fn new(
        pedido_service: Arc<PedidoService>,
        loja_uuid: Uuid,
        usuario: Usuario
    ) -> Self {

        Self {
            pedido_service,
            loja_uuid,
            usuario
        }

    }
}