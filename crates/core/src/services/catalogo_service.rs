use std::sync::Arc;

use uuid::Uuid;
use rust_decimal::Decimal;

use crate::models::{
    Adicional, CategoriaProdutos, Produto
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
        preco: Decimal,
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
        pizza_mode: bool,
    ) -> Result<CategoriaProdutos, String> {

        let categoria: CategoriaProdutos = CategoriaProdutos::new(
            nome,
            descricao,
            loja_uuid,
            ordem,
            pizza_mode
        );

        self.categoria_repo.criar(&categoria).await?;

        Ok(categoria)
    }

    pub async fn criar_produto(
        &self,
        nome: String,
        descricao: Option<String>,
        preco: Decimal,
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
        preco: Decimal,
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

    pub async fn listar_adicionais(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<Adicional>, String> {
        self.adicional_repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn listar_adicionais_disponiveis(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<Adicional>, String> {
        self.adicional_repo.buscar_disponiveis(loja_uuid).await
    }

    pub async fn atualizar_disponibilidade(
        &self,
        adicional_uuid: Uuid,
        loja_uuid: Uuid,
        disponivel: bool,
    ) -> Result<(), String> {
        let adicional = self.adicional_repo.buscar_por_uuid(adicional_uuid).await?
            .ok_or("Adicional não encontrado")?;

        if adicional.loja_uuid != loja_uuid {
            return Err("Adicional não pertence a esta loja".to_string());
        }

        self.adicional_repo.atualizar_disponibilidade(adicional_uuid, disponivel).await
    }

    pub async fn atualizar_adicional(
        &self,
        adicional_uuid: Uuid,
        loja_uuid: Uuid,
        nome: String,
        descricao: String,
        preco: Decimal,
    ) -> Result<Adicional, String> {
        let mut adicional = self.adicional_repo.buscar_por_uuid(adicional_uuid).await?
            .ok_or("Adicional não encontrado")?;

        if adicional.loja_uuid != loja_uuid {
            return Err("Adicional não pertence a esta loja".to_string());
        }

        adicional.nome = nome;
        adicional.descricao = descricao;
        adicional.preco = preco;

        self.adicional_repo.atualizar(adicional.clone()).await?;
        Ok(adicional)
    }

    pub async fn deletar_adicional(
        &self,
        adicional_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Result<(), String> {
        let adicional = self.adicional_repo.buscar_por_uuid(adicional_uuid).await?
            .ok_or("Adicional não encontrado")?;

        if adicional.loja_uuid != loja_uuid {
            return Err("Adicional não pertence a esta loja".to_string());
        }

        self.adicional_repo.deletar(adicional_uuid).await
    }

    pub async fn listar_categorias(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<CategoriaProdutos>, String> {
        self.categoria_repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn atualizar_categoria(
        &self,
        uuid: Uuid,
        loja_uuid: Uuid,
        nome: String,
        descricao: Option<String>,
        ordem: Option<i32>,
        pizza_mode: bool,
    ) -> Result<CategoriaProdutos, String> {
        let mut categoria = self.categoria_repo.buscar_por_uuid(uuid).await?
            .ok_or("Categoria não encontrada")?;

        if categoria.loja_uuid != loja_uuid {
            return Err("Categoria não pertence a esta loja".to_string());
        }

        categoria.nome = nome;
        categoria.descricao = descricao;
        categoria.ordem = ordem;
        categoria.pizza_mode = pizza_mode;

        self.categoria_repo.atualizar(categoria.clone()).await?;
        Ok(categoria)
    }

    pub async fn deletar_categoria(
        &self,
        uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Result<(), String> {
        let categoria = self.categoria_repo.buscar_por_uuid(uuid).await?
            .ok_or("Categoria não encontrada")?;

        if categoria.loja_uuid != loja_uuid {
            return Err("Categoria não pertence a esta loja".to_string());
        }

        // Verificar se a categoria está vazia (sem produtos)
        let produtos = self.produto_repo.buscar_por_categoria(uuid).await?;
        if !produtos.is_empty() {
            return Err(format!(
                "Não é possível deletar categoria com {} produto(s). Remova os produtos primeiro.",
                produtos.len()
            ));
        }

        self.categoria_repo.deletar(uuid).await
    }

    pub async fn buscar_produto_por_uuid(
        &self,
        uuid: Uuid,
    ) -> Result<Produto, String> {
        self.produto_repo.buscar_por_uuid(uuid).await?
            .ok_or("Produto não encontrado".to_string())
    }

    pub async fn listar_produtos_por_categoria(
        &self,
        categoria_uuid: Uuid,
    ) -> Result<Vec<Produto>, String> {
        self.produto_repo.buscar_por_categoria(categoria_uuid).await
    }

    pub async fn deletar_produto(
        &self,
        uuid: Uuid,
    ) -> Result<(), String> {
        self.produto_repo.deletar(uuid).await
    }

    pub async fn atualizar_disponibilidade_produto(
        &self,
        produto_uuid: Uuid,
        loja_uuid: Uuid,
        disponivel: bool,
    ) -> Result<(), String> {
        let produto = self.produto_repo.buscar_por_uuid(produto_uuid).await?
            .ok_or("Produto não encontrado")?;

        if produto.loja_uuid != loja_uuid {
            return Err("Produto não pertence a esta loja".to_string());
        }

        self.produto_repo.atualizar_disponibilidade(produto_uuid, disponivel).await
    }

}
