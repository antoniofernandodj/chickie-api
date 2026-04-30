use std::sync::Arc;

use uuid::Uuid;
use rust_decimal::Decimal;

use crate::models::{
    Adicional, CategoriaProdutos, CategoriaProdutosOrdenada, Produto
};

use crate::ports::{
    ProdutoRepositoryPort,
    CategoriaRepositoryPort,
    AdicionalRepositoryPort,
    OrdemCategoriaRepositoryPort,
};

#[derive(Clone)]
pub struct CatalogoService {
    produto_repo: Arc<dyn ProdutoRepositoryPort>,
    categoria_repo: Arc<dyn CategoriaRepositoryPort>,
    adicional_repo: Arc<dyn AdicionalRepositoryPort>,
    ordem_categoria_repo: Arc<dyn OrdemCategoriaRepositoryPort>,
}

impl CatalogoService {
    pub fn new(
        produto_repo: Arc<dyn ProdutoRepositoryPort>,
        categoria_repo: Arc<dyn CategoriaRepositoryPort>,
        adicional_repo: Arc<dyn AdicionalRepositoryPort>,
        ordem_categoria_repo: Arc<dyn OrdemCategoriaRepositoryPort>,
    ) -> Self {
        Self { produto_repo, categoria_repo, adicional_repo, ordem_categoria_repo }
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
        loja_uuid: Option<Uuid>,
        pizza_mode: bool,
        drink_mode: bool,
    ) -> Result<CategoriaProdutos, String> {

        let categoria = CategoriaProdutos::new(
            nome,
            descricao,
            loja_uuid,
            pizza_mode,
            drink_mode
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
        self.produto_repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
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
        self.adicional_repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn listar_adicionais_disponiveis(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<Adicional>, String> {
        self.adicional_repo.listar_disponiveis(loja_uuid).await.map_err(|e| e.to_string())
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

        self.adicional_repo.atualizar_disponibilidade(adicional_uuid, disponivel).await.map_err(|e| e.to_string())
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

        self.adicional_repo.deletar(adicional_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn listar_categorias(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<CategoriaProdutosOrdenada>, String> {
        self.categoria_repo.listar_por_loja_com_ordem(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn listar_categorias_globais(
        &self,
    ) -> Result<Vec<CategoriaProdutos>, String> {
        self.categoria_repo.listar_globais().await.map_err(|e| e.to_string())
    }

    pub async fn verificar_cobertura_categorias_globais(
        &self,
    ) -> Result<Vec<crate::models::StatusCategoriaGlobal>, String> {
        self.categoria_repo.verificar_cobertura_globais().await.map_err(|e| e.to_string())
    }

    pub async fn atualizar_categoria(
        &self,
        uuid: Uuid,
        loja_uuid: Uuid,
        nome: String,
        descricao: Option<String>,
        pizza_mode: bool,
        drink_mode: bool,
    ) -> Result<CategoriaProdutos, String> {
        let mut categoria = self.categoria_repo.buscar_por_uuid(uuid).await?
            .ok_or("Categoria não encontrada")?;

        if categoria.loja_uuid != Some(loja_uuid) {
            return Err("Categoria não pertence a esta loja".to_string());
        }

        categoria.nome = nome;
        categoria.descricao = descricao;
        categoria.pizza_mode = pizza_mode;
        categoria.drink_mode = drink_mode;

        self.categoria_repo.atualizar(categoria.clone()).await?;
        Ok(categoria)
    }

    pub async fn reordenar_categorias(
        &self,
        loja_uuid: Uuid,
        reordenacoes: Vec<(Uuid, i32)>,
    ) -> Result<(), String> {
        self.ordem_categoria_repo.definir_ordens(loja_uuid, reordenacoes).await.map_err(|e| e.to_string())
    }

    pub async fn deletar_categoria(
        &self,
        uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Result<(), String> {
        let categoria = self.categoria_repo.buscar_por_uuid(uuid).await?
            .ok_or("Categoria não encontrada")?;

        if categoria.loja_uuid != Some(loja_uuid) {
            return Err("Categoria não pertence a esta loja".to_string());
        }

        let total_produtos = self.categoria_repo.contar_produtos(uuid).await.map_err(|e| e.to_string())?;
        if total_produtos > 0 {
            return Err(format!(
                "Não é possível deletar categoria com {} produto(s). Remova os produtos primeiro.",
                total_produtos
            ));
        }

        self.categoria_repo.deletar(uuid).await.map_err(|e| e.to_string())
    }

    pub async fn atualizar_categoria_global(
        &self,
        uuid: Uuid,
        nome: String,
        descricao: Option<String>,
        pizza_mode: bool,
        drink_mode: bool,
    ) -> Result<CategoriaProdutos, String> {
        let mut categoria = self.categoria_repo.buscar_por_uuid(uuid).await?
            .ok_or("Categoria não encontrada")?;

        if categoria.loja_uuid.is_some() {
            return Err("Categoria não é global".to_string());
        }

        categoria.nome = nome;
        categoria.descricao = descricao;
        categoria.pizza_mode = pizza_mode;
        categoria.drink_mode = drink_mode;

        self.categoria_repo.atualizar(categoria.clone()).await?;
        Ok(categoria)
    }

    pub async fn deletar_categoria_global(
        &self,
        uuid: Uuid,
    ) -> Result<(), String> {
        let categoria = self.categoria_repo.buscar_por_uuid(uuid).await?
            .ok_or("Categoria não encontrada")?;

        if categoria.loja_uuid.is_some() {
            return Err("Categoria não é global".to_string());
        }

        let total_produtos = self.categoria_repo.contar_produtos(uuid).await.map_err(|e| e.to_string())?;
        if total_produtos > 0 {
            return Err(format!(
                "Não é possível deletar categoria com {} produto(s). Remova os produtos primeiro.",
                total_produtos
            ));
        }

        self.categoria_repo.deletar(uuid).await.map_err(|e| e.to_string())
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
        loja_uuid: Uuid,
        categoria_uuid: Uuid,
    ) -> Result<Vec<Produto>, String> {
        self.produto_repo.listar_por_categoria(loja_uuid, categoria_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn deletar_produto(
        &self,
        uuid: Uuid,
    ) -> Result<(), String> {
        self.produto_repo.deletar(uuid).await.map_err(|e| e.to_string())
    }

    pub async fn listar_produtos_por_categoria_global(
        &self,
        categoria_uuid: Uuid,
    ) -> Result<Vec<Produto>, String> {
        let categoria = self.categoria_repo.buscar_por_uuid(categoria_uuid).await?
            .ok_or("Categoria não encontrada")?;

        if categoria.loja_uuid.is_some() {
            return Err("Categoria não é global".to_string());
        }

        self.produto_repo.listar_por_categoria_global(categoria_uuid).await.map_err(|e| e.to_string())
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

        self.produto_repo.atualizar_disponibilidade(produto_uuid, disponivel).await.map_err(|e| e.to_string())
    }

}
