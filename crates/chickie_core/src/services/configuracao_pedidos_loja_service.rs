use std::sync::Arc;
use uuid::Uuid;

use crate::models::{ConfiguracaoDePedidosLoja, TipoCalculoPedido};
use crate::ports::ConfiguracaoPedidosLojaRepositoryPort;


#[derive(Clone)]
pub struct ConfiguracaoPedidosLojaService {
    repo: Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>,
}

#[allow(dead_code)]
impl ConfiguracaoPedidosLojaService {
    pub fn new(repo: Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>) -> Self {
        Self { repo }
    }

    pub async fn buscar(&self, loja_uuid: Uuid) -> Result<Option<ConfiguracaoDePedidosLoja>, String> {
        self.repo.buscar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn salvar(&self, config: &ConfiguracaoDePedidosLoja) -> Result<(), String> {
        self.repo.salvar(config).await.map_err(|e| e.to_string())
    }

    pub async fn alterar_tipo_calculo(&self, _loja_uuid: Uuid, _novo_tipo: TipoCalculoPedido) -> Result<(), String> {
        // Port doesn't have this method - would need to be added
        Err("alterar_tipo_calculo not implemented on port".to_string())
    }

    pub async fn alterar_max_partes(&self, _loja_uuid: Uuid, _novo_max: i32) -> Result<(), String> {
        // Port doesn't have this method - would need to be added
        Err("alterar_max_partes not implemented on port".to_string())
    }
}
