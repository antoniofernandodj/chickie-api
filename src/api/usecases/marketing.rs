

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::{Cupom, UsoCupom, Promocao, Usuario}, services::{CatalogoService, MarketingService}};

pub struct MarketingUsecase {
    pub pedido_service: Arc<MarketingService>,
    pub loja_uuid: Uuid,
    pub usuario: Usuario,
}


impl MarketingUsecase {
    pub fn new(
        pedido_service: Arc<MarketingService>,
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