use std::sync::Arc;
use uuid::Uuid;

use crate::models::{ConfiguracaoDePedidosLoja, TipoCalculoPedido};
use crate::repositories::{ConfiguracaoPedidosLojaRepository};


#[derive(Clone)]
pub struct ConfiguracaoPedidosLojaService {
    repo: Arc<ConfiguracaoPedidosLojaRepository>,
}

#[allow(dead_code)]
impl ConfiguracaoPedidosLojaService {
    pub fn new(repo: Arc<ConfiguracaoPedidosLojaRepository>) -> Self {
        Self { repo }
    }

    pub async fn buscar(&self, loja_uuid: Uuid) -> Result<Option<ConfiguracaoDePedidosLoja>, String> {
        self.repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn salvar(&self, config: &ConfiguracaoDePedidosLoja) -> Result<(), String> {
        self.repo.salvar(config).await
    }

    pub async fn alterar_tipo_calculo(&self, loja_uuid: Uuid, novo_tipo: TipoCalculoPedido) -> Result<(), String> {
        self.repo.alterar_tipo_calculo(loja_uuid, novo_tipo).await
    }

    pub async fn alterar_max_partes(&self, loja_uuid: Uuid, novo_max: i32) -> Result<(), String> {
        self.repo.alterar_max_partes(loja_uuid, novo_max).await
    }
}
