use std::sync::Arc;
use uuid::Uuid;

use crate::models::EnderecoEntrega;
use crate::ports::EnderecoEntregaRepositoryPort;

#[derive(Clone)]
pub struct EnderecoEntregaService {
    repo: Arc<dyn EnderecoEntregaRepositoryPort>,
}

impl EnderecoEntregaService {
    pub fn new(repo: Arc<dyn EnderecoEntregaRepositoryPort>) -> Self {
        Self { repo }
    }

    /// Cria um endereço de entrega vinculado a um pedido (snapshot imutável)
    pub async fn criar_para_pedido(
        &self,
        pedido_uuid: Uuid,
        loja_uuid: Uuid,
        cep: Option<String>,
        logradouro: String,
        numero: String,
        complemento: Option<String>,
        bairro: String,
        cidade: String,
        estado: String,
        // latitude: Option<f64>,
        // longitude: Option<f64>,
    ) -> Result<EnderecoEntrega, String> {

        let endereco = EnderecoEntrega::new(
            pedido_uuid,
            loja_uuid,
            cep,
            logradouro,
            numero,
            complemento,
            bairro,
            cidade,
            estado,
            // latitude,
            // longitude,
        );

        self.repo.criar(&endereco).await.map_err(|e| e.to_string())?;
        Ok(endereco)
    }

    /// Busca o endereço de entrega de um pedido
    pub async fn buscar_por_pedido(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<Option<EnderecoEntrega>, String> {
        self.repo.buscar_por_pedido(pedido_uuid).await.map_err(|e| e.to_string())
    }

    /// Lista endereços de entrega de uma loja (para relatórios/auditoria)
    pub async fn listar_por_loja(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<EnderecoEntrega>, String> {
        self.repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }
}