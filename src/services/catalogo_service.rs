use std::sync::Arc;

use uuid::Uuid;

use crate::models::{
    Adicional, CategoriaProdutos, Model, Produto
};

use crate::repositories::{
    ProdutoRepository,
    CategoriaProdutosRepository,
    AdicionalRepository,
    Repository as _
};

#[derive(Clone)]
pub struct CatalogoService {
    produto_repo: Arc<ProdutoRepository>,
    categoria_repo: Arc<CategoriaProdutosRepository>,
    adicional_repo: Arc<AdicionalRepository>,
}

impl CatalogoService {
    pub fn new(
        produto_repo: Arc<ProdutoRepository>,
        categoria_repo: Arc<CategoriaProdutosRepository>,
        adicional_repo: Arc<AdicionalRepository>,
    ) -> Self {
        Self { produto_repo, categoria_repo, adicional_repo }
    }

    pub async fn criar_adicional(
        &self,
        nome: String,
        loja_uuid: Uuid,
        descricao: String,
        preco: f64,
    ) -> Result<Adicional, String> {

        let adicional = Adicional::new(
            nome,
            loja_uuid,
            descricao,
            preco
        );

        self.adicional_repo.criar(&adicional).await?;

        Ok(adicional)
    }

    pub async fn criar_categoria(
        &self,
        nome: String,
        descricao: Option<String>,
        loja_uuid: Uuid,
        ordem: Option<i32>,
    ) -> Result<CategoriaProdutos, String> {

        let categoria: CategoriaProdutos = CategoriaProdutos::new(
            nome,
            descricao,
            loja_uuid,
            ordem
        );

        self.categoria_repo.criar(&categoria).await?;

        Ok(categoria)
    }

    pub async fn criar_produto(
        &self,
        nome: String,
        descricao: Option<String>,
        preco: f64,
        categoria_uuid: Uuid,
        loja_uuid: Uuid,
        tempo_preparo_min: Option<i32>,
    ) -> Result<Produto, String> {

        let produto = Produto::new(
            nome,
            descricao,
            preco,
            categoria_uuid,
            loja_uuid,
            tempo_preparo_min
        );

        self.produto_repo.criar(&produto).await?;

        Ok(produto)
    }
    
    pub async fn listar_produtos_de_loja(
        &self,
        loja_uuid: Uuid
    ) -> Result<Vec<Produto>, String> {
        self.produto_repo.listar_todos_por_loja(loja_uuid).await
    }

    pub async fn atualizar_produto(
        &self,
        produto_uuid: Uuid,
        nome: String,
        descricao: Option<String>,
        preco: f64,
        categoria_uuid: Uuid,
        tempo_preparo_min: Option<i32>,
    ) -> Result<Produto, String> {


        let produto_antigo_busca =
            self.produto_repo.buscar_por_uuid(produto_uuid)
            .await?;

        if let Some(mut produto_atualizado) = produto_antigo_busca {

            produto_atualizado.nome = nome;
            produto_atualizado.descricao = descricao;
            produto_atualizado.preco = preco;
            produto_atualizado.categoria_uuid = categoria_uuid;
            produto_atualizado.tempo_preparo_min = tempo_preparo_min;

            self.produto_repo.atualizar(
                produto_atualizado.clone()
            )
            .await?;

            return Ok(produto_atualizado)

        } else {
            Err("Produto não encontrado".to_string())
        }

    }

}
