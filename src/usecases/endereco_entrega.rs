use std::sync::Arc;

use uuid::Uuid;

use crate::models::{EnderecoEntrega, Usuario};
use crate::services::EnderecoEntregaService;

pub struct ListarEnderecosEntregaPorLojaUsecase {
    pub endereco_entrega_service: Arc<EnderecoEntregaService>,
    pub usuario: Usuario,
    pub loja_uuid: Uuid,
}

impl ListarEnderecosEntregaPorLojaUsecase {
    pub fn new(
        endereco_entrega_service: Arc<EnderecoEntregaService>,
        usuario: Usuario,
        loja_uuid: Uuid,
    ) -> Self {
        Self {
            endereco_entrega_service,
            usuario,
            loja_uuid,
        }
    }

    pub async fn executar(&self) -> Result<Vec<EnderecoEntrega>, String> {
        self.endereco_entrega_service
            .listar_por_loja(self.loja_uuid)
            .await
    }
}
