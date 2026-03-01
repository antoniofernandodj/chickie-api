use std::sync::Arc;

use uuid::Uuid;

use crate::models::{
    Produto,
    CategoriaProdutos,
    Adicional
};

use crate::repositories::{
    ProdutoRepository,
    CategoriaProdutosRepository,
    AdicionalRepository,
    Repository as _
};

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
    
    pub async fn listar_produtos(&self) -> Result<Vec<Produto>, String> {
        self.produto_repo.listar_todos().await
    }

}