use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::{models::{Adicional, Produto, Usuario}, services::CatalogoService};

#[derive(Debug, Deserialize, Serialize)]
pub struct AtualizarProdutoRequest {
    nome: String,
    descricao: Option<String>,
    preco: Decimal,
    categoria_uuid: Uuid,
    tempo_preparo_min: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProdutoRequest {
    pub uuid: Option<Uuid>,
    pub loja_uuid: Uuid,
    pub categoria_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: Decimal,
    pub imagem_url: Option<String>,
    pub disponivel: bool,
    pub tempo_preparo_min: Option<i32>,
    pub destaque: bool,
}


pub struct CatalogoUsecase {
    pub catalogo_service: Arc<CatalogoService>,
    pub loja_uuid: Uuid,
    pub _usuario: Usuario,
}


impl CatalogoUsecase {
    pub fn new(
        catalogo_service: Arc<CatalogoService>,
        loja_uuid: Uuid,
        _usuario: Usuario
    ) -> Self {
        Self { catalogo_service, loja_uuid, _usuario }
    }

    pub async fn listar_produtos(&self) -> Result<Vec<Produto>, String> {
        // Llamar al servicio de catalogo
        self.catalogo_service.listar_produtos_de_loja(self.loja_uuid).await
    }

    pub async fn atualizar_produto(
        &self,
        produto_uuid: Uuid,
        data: AtualizarProdutoRequest
    ) -> Result<Produto, String> {
        let produto = self
            .catalogo_service
            .atualizar_produto(
                produto_uuid,
                data.nome,
                data.descricao,
                data.preco, 
                data.categoria_uuid,
                data.tempo_preparo_min
            )
            .await?;

        Ok(produto)
    }

    pub async fn criar_produto(
        &self,
        data: CreateProdutoRequest,
    ) -> Result<Produto, String> {

        let produto = self
            .catalogo_service
            .criar_produto(
                data.nome,
                data.descricao,
                data.preco,
                data.categoria_uuid,
                self.loja_uuid,
                data.tempo_preparo_min
            )
            .await?;

        Ok(produto)
    }

    // ─── Adicionais ───

    pub async fn listar_adicionais(&self) -> Result<Vec<Adicional>, String> {
        self.catalogo_service.listar_adicionais(self.loja_uuid).await
    }

    pub async fn listar_adicionais_disponiveis(&self) -> Result<Vec<Adicional>, String> {
        self.catalogo_service.listar_adicionais_disponiveis(self.loja_uuid).await
    }

    pub async fn marcar_adicional_indisponivel(
        &self,
        adicional_uuid: Uuid,
    ) -> Result<(), String> {
        self.catalogo_service.marcar_adicional_indisponivel(adicional_uuid).await
    }

    pub async fn deletar_produto(&self, produto_uuid: Uuid) -> Result<(), String> {
        self.catalogo_service.deletar_produto(produto_uuid).await
    }
}