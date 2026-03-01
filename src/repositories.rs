use std::sync::Arc;

use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::{models::*, utils::agora};

/// Repositório genérico assíncrono
#[async_trait::async_trait]
pub trait Repository<T> {
    async fn criar(&self, item: &T) -> Result<Uuid, String>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<T>, String>;
    async fn atualizar(&self, item: T) -> Result<(), String>;
    async fn deletar(&self, uuid: Uuid) -> Result<(), String>;
    async fn listar_todos(&self) -> Result<Vec<T>, String>;
    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<T>, String>;
    fn table_name(&self) -> String;
}

// ==================== REPOSITÓRIO DE USUÁRIOS ====================
pub struct UsuarioRepository { pool: Arc<SqlitePool> }
impl UsuarioRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("
            SELECT * FROM usuarios WHERE email = ?;
        ")
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_username(&self, username: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("
            SELECT * FROM usuarios WHERE username = ?;
        ")
        .bind(username)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_telefone(&self, telefone: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("
            SELECT * FROM usuarios WHERE telefone = ?;
        ")
        .bind(telefone)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Usuario> for UsuarioRepository {
    fn table_name(&self) -> String { "usuarios".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Usuario>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Usuario>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Usuario) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO usuarios (
                uuid,
                nome,
                username,
                email,
                senha_hash,
                telefone,
                celular,
                criado_em,
                atualizado_em
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
        ")
        .bind(&item.uuid)
        .bind(&item.nome)
        .bind(&item.username)
        .bind(&item.email)
        .bind(&item.senha_hash)
        .bind(&item.telefone)
        .bind(&item.celular)
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }


    async fn atualizar(&self, item: Usuario) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE usuarios
            SET
                username = ?,
                email = ?,
                senha_hash = ?,
                telefone = ?,
                atualizado_em = ? 
            WHERE uuid = ?;
        ")
        .bind(&item.username)
        .bind(&item.email)
        .bind(&item.senha_hash)
        .bind(&item.telefone)
        .bind("")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Usuário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
                DELETE FROM usuarios WHERE uuid = ?;
            ")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Usuário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("SELECT * FROM usuarios;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("
                SELECT * FROM usuarios
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE LOJAS ====================
pub struct LojaRepository { pool: Arc<SqlitePool> }
impl LojaRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Loja>, String> {
        sqlx::query_as::<_, Loja>("
            SELECT * FROM lojas WHERE email = ?;
        ")
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn listar_ativas(&self) -> Result<Vec<Loja>, String> {
        sqlx::query_as::<_, Loja>("
            SELECT * FROM lojas WHERE ativa = true;
        ")
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Loja> for LojaRepository {
    fn table_name(&self) -> String { "lojas".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Loja>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Loja>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Loja) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO lojas (
                uuid,
                nome,
                slug,
                descricao,
                email,
                telefone,
                ativa,
                logo_url,
                banner_url, 
                horario_abertura,
                horario_fechamento,
                dias_funcionamento,
                tempo_preparo_min,
                taxa_entrega, 
                valor_minimo_pedido,
                raio_entrega_km,
                criado_em,
                atualizado_em
            ) 
            VALUES (
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?, 
                ?
            );
        ")
        .bind(item.uuid)
        .bind(&item.nome)
        .bind(&item.slug)
        .bind(&item.descricao)
        .bind(&item.email)
        .bind(&item.telefone)
        .bind(item.ativa)
        .bind(&item.logo_url)
        .bind(&item.banner_url)
        .bind(&item.horario_abertura)
        .bind(&item.horario_fechamento)
        .bind(&item.dias_funcionamento)
        .bind(item.tempo_preparo_min)
        .bind(item.taxa_entrega)
        .bind(item.valor_minimo_pedido)
        .bind(item.raio_entrega_km)
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }



    async fn atualizar(&self, item: Loja) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE lojas
            SET
                nome = ?,
                slug = ?,
                descricao = ?,
                email = ?,
                telefone = ?,
                ativa = ?, 
                logo_url = ?,
                banner_url = ?,
                horario_abertura = ?,
                horario_fechamento = ?, 
                dias_funcionamento = ?,
                tempo_preparo_min = ?,
                taxa_entrega = ?,
                valor_minimo_pedido = ?, 
                raio_entrega_km = ?,
                atualizado_em = ?
            WHERE uuid = ?;
        ")
        .bind(&item.nome)
        .bind(&item.slug)
        .bind(&item.descricao)
        .bind(&item.email)
        .bind(&item.telefone)
        .bind(item.ativa)
        .bind(&item.logo_url)
        .bind(&item.banner_url)
        .bind(&item.horario_abertura)
        .bind(&item.horario_fechamento)
        .bind(&item.dias_funcionamento)
        .bind(item.tempo_preparo_min)
        .bind(item.taxa_entrega)
        .bind(item.valor_minimo_pedido)
        .bind(item.raio_entrega_km)
        .bind(item.atualizado_em)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Loja não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM lojas WHERE uuid = ?;")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Loja não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Loja>, String> {
        sqlx::query_as::<_, Loja>("SELECT * FROM lojas;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<Loja>, String> {
        Err("Não é possivel executar este método".into())
    }
}

// ==================== REPOSITÓRIO DE CLIENTES ====================
pub struct ClienteRepository { pool: Arc<SqlitePool> }
impl<'a> ClienteRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("
            SELECT * FROM clientes WHERE usuario_uuid = ?;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("
            SELECT * FROM clientes WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
                SELECT * FROM produtos
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Cliente> for ClienteRepository {
    fn table_name(&self) -> String { "clientes".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Cliente>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Cliente>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Cliente) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO clientes (
                uuid,
                usuario_uuid,
                loja_uuid,
                criado_em
            )
            VALUES (?, ?, ?, ?);
        ")
        .bind(item.uuid)
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Cliente) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query(
            "UPDATE clientes
            SET
                usuario_uuid = ?,
                loja_uuid = ?
            WHERE uuid = ?;
        ")
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cliente não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
                DELETE FROM clientes WHERE uuid = ?;
            ")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cliente não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("SELECT * FROM clientes;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("
                SELECT * FROM clientes
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE PRODUTOS ====================
pub struct ProdutoRepository { pool: Arc<SqlitePool> }
impl<'a> ProdutoRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
            SELECT * FROM produtos
            WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_categoria(&self, categoria_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
            SELECT * FROM produtos
            WHERE categoria_uuid = ?;
        ")
        .bind(categoria_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
            SELECT * FROM produtos
            WHERE loja_uuid = ? AND disponivel = true;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
            SELECT * FROM produtos
            WHERE loja_uuid = ? AND nome LIKE ?;
        ")
        .bind(loja_uuid)
        .bind(format!("%{}%", nome))
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }


}

#[async_trait::async_trait]
impl<'a> Repository<Produto> for ProdutoRepository {
    fn table_name(&self) -> String { "produtos".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Produto>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Produto>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Produto) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO produtos (
                uuid,
                loja_uuid,
                categoria_uuid,
                nome,
                descricao,
                preco, 
                imagem_url,
                disponivel,
                tempo_preparo_min,
                destaque,
                criado_em,
                atualizado_em
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.categoria_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(&item.imagem_url)
        .bind(item.disponivel)
        .bind(item.tempo_preparo_min)
        .bind(item.destaque)
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Produto) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE produtos
            SET
                loja_uuid = ?,
                categoria_uuid = ?,
                nome = ?,
                descricao = ?, 
                preco = ?,
                imagem_url = ?,
                disponivel = ?,
                tempo_preparo_min = ?,
                destaque = ?, 
                atualizado_em = ?
            WHERE uuid = ?;
        ")
        .bind(item.loja_uuid)
        .bind(item.categoria_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(&item.imagem_url)
        .bind(item.disponivel)
        .bind(item.tempo_preparo_min)
        .bind(item.destaque)
        .bind(item.atualizado_em)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Produto não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
                DELETE FROM produtos WHERE uuid = ?
            ")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Produto não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("SELECT * FROM produtos")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
                SELECT * FROM produtos
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE CATEGORIAS ====================
pub struct CategoriaProdutosRepository { pool: Arc<SqlitePool> }
impl<'a> CategoriaProdutosRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(
        &self,
        loja_uuid: Uuid
    ) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("
            SELECT * FROM categorias_produtos
            WHERE loja_uuid = ?
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Option<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("
            SELECT * FROM categorias_produtos
            WHERE loja_uuid = ? AND nome = ?
        ")
        .bind(loja_uuid)
        .bind(nome)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<CategoriaProdutos> for CategoriaProdutosRepository {
    fn table_name(&self) -> String { "categorias_produtos".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<CategoriaProdutos>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, CategoriaProdutos>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &CategoriaProdutos) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO categorias_produtos (
                uuid,
                loja_uuid,
                nome,
                descricao,
                ordem,
                criado_em
            ) 
            VALUES (?, ?, ?, ?, ?, ?)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: CategoriaProdutos) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE categorias_produtos
                SET loja_uuid = ?,
                nome = ?,
                descricao = ?,
                ordem = ? 
             WHERE uuid = ?
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Categoria não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
                DELETE FROM categorias_produtos WHERE uuid = ?
            ")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Categoria não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("
                SELECT * FROM categorias_produtos
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE PEDIDOS ====================
pub struct PedidoRepository { pool: Arc<SqlitePool> }
impl<'a> PedidoRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("
            SELECT * FROM pedidos WHERE usuario_uuid = ?;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("
            SELECT * FROM pedidos WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_status(&self, status: EstadoDePedido) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE status = ?;"
        )
        .bind(status.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_pendentes(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("
            SELECT * FROM pedidos
            WHERE loja_uuid = ? AND (status = ? OR status = ?)
        ")
        .bind(loja_uuid)
        .bind(EstadoDePedido::EmPreparo.to_string())
        .bind(EstadoDePedido::Criado.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca um pedido completo com todos os seus itens, adicionais e partes
    pub async fn buscar_completo(
        &self,
        uuid: Uuid,
    ) -> Result<Option<Pedido>, String> {
        // 1. Busca o pedido base
        let mut pedido = match sqlx::query_as::<_, Pedido>("
            SELECT * FROM pedidos WHERE uuid = ?;
        ")
        .bind(uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())?
        {
            Some(p) => p,
            None => return Ok(None),
        };

        // 2. Busca todos os itens do pedido
        let mut itens = sqlx::query_as::<_, ItemPedido>("
            SELECT * FROM itens_pedido
            WHERE pedido_uuid = ?
            ORDER BY rowid ASC;
        ")
        .bind(uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        for item in &mut itens {
            item.adicionais = self.buscar_adicionais_de_item_de_pedido(&item).await?;
            item.partes = self.buscar_partes_de_item_de_pedido(&item).await?;
        }

        pedido.itens = itens;
        Ok(Some(pedido))
    }

    async fn buscar_partes_de_item_de_pedido(
        &self,
        item: &ItemPedido,
    ) -> Result<Vec<ParteDeItemPedido>, std::string::String> {

        let partes = sqlx::query_as::<_, ParteDeItemPedido>("
            SELECT * FROM partes_item_pedido
            WHERE item_uuid = ?
            ORDER BY posicao ASC;
        ")
        .bind(item.uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string());

        partes
    }

    async fn buscar_adicionais_de_item_de_pedido(
        &self,
        item: &ItemPedido
    ) -> Result<Vec<AdicionalDeItemDePedido>, String> {

        let adicionais = sqlx::query_as::<_, AdicionalDeItemDePedido>("
            SELECT * FROM adicionais_item_pedido
            WHERE item_uuid = ?;
        ")
        .bind(item.uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string());

        adicionais
    }

    /// Mesma lógica mas para múltiplos pedidos (ex: listar pedidos de uma loja)
    pub async fn buscar_completos_por_loja(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<Pedido>, String> {
        let pedidos = sqlx::query_as::<_, Pedido>("
            SELECT * FROM pedidos
            WHERE loja_uuid = ?
            ORDER BY criado_em DESC;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        self.hidratar_pedidos(pedidos).await
    }

    pub async fn buscar_completos_por_usuario(
        &self,
        usuario_uuid: Uuid,
    ) -> Result<Vec<Pedido>, String> {
        let pedidos = sqlx::query_as::<_, Pedido>("
            SELECT * FROM pedidos
            WHERE usuario_uuid = ?
            ORDER BY criado_em DESC;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        self.hidratar_pedidos(pedidos).await
    }

    /// Extrai a lógica de hidratação para reuso entre os métodos acima
    async fn hidratar_pedidos(
        &self,
        pedidos: Vec<Pedido>,
    ) -> Result<Vec<Pedido>, String> {
        if pedidos.is_empty() {
            return Ok(vec![]);
        }

        // Coleta todos os UUIDs dos pedidos para buscar itens em uma só query
        let uuids_pedidos: Vec<String> = pedidos
            .iter()
            .map(|p| format!("'{}'", p.uuid))
            .collect();
        let placeholder = uuids_pedidos.join(", ");

        let mut itens = sqlx::query_as::<_, ItemPedido>(&format!("
            SELECT * FROM itens_pedido
            WHERE pedido_uuid IN ({})
            ORDER BY pedido_uuid, rowid ASC;
        ", placeholder))
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Busca adicionais e partes de todos os itens em duas queries únicas
        if !itens.is_empty() {
            let uuids_itens: Vec<String> = itens
                .iter()
                .map(|i| format!("'{}'", i.uuid))
                .collect();
            let placeholder_itens = uuids_itens.join(", ");

            let adicionais = sqlx::query_as::<_, AdicionalDeItemDePedido>(&format!("
                SELECT * FROM adicionais_item_pedido
                WHERE item_uuid IN ({});
            ", placeholder_itens))
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let partes = sqlx::query_as::<_, ParteDeItemPedido>(&format!("
                SELECT * FROM partes_item_pedido
                WHERE item_uuid IN ({})
                ORDER BY item_uuid, posicao ASC;
            ", placeholder_itens))
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

            // Distribui adicionais e partes nos itens correspondentes
            for item in &mut itens {
                item.adicionais = adicionais
                    .iter()
                    .filter(|a| a.item_uuid == item.uuid)
                    .cloned()
                    .collect();

                item.partes = partes
                    .iter()
                    .filter(|s| s.item_uuid == Some(item.uuid))
                    .cloned()
                    .collect();
            }
        }

        // Distribui itens nos pedidos correspondentes
        let pedidos_hidratados = pedidos
            .into_iter()
            .map(|mut pedido| {
                pedido.itens = itens
                    .iter()
                    .filter(|i| i.pedido_uuid == pedido.uuid)
                    .cloned()
                    .collect();
                pedido
            })
            .collect();

        Ok(pedidos_hidratados)
    }

    
}

#[async_trait::async_trait]
impl<'a> Repository<Pedido> for PedidoRepository {
    fn table_name(&self) -> String { "pedidos".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Pedido>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Pedido>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(
        &self,
        pedido: &Pedido
    ) -> Result<Uuid, String> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| e.to_string())?;

        println!("[PEDIDO] Inserindo pedido uuid={}", pedido.uuid);

        sqlx::query("
            INSERT INTO pedidos (
                uuid,
                usuario_uuid,
                loja_uuid,
                status,
                total,
                subtotal,
                taxa_entrega,
                desconto,
                forma_pagamento,
                observacoes,
                tempo_estimado_min,
                criado_em,
                atualizado_em
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ")
        .bind(&pedido.uuid)
        .bind(&pedido.usuario_uuid)
        .bind(&pedido.loja_uuid)
        .bind(&pedido.status.to_string())
        .bind(&pedido.total)
        .bind(&pedido.subtotal)
        .bind(&pedido.taxa_entrega)
        .bind(&pedido.desconto)
        .bind(&pedido.forma_pagamento)
        .bind(&pedido.observacoes)
        .bind(&pedido.tempo_estimado_min)
        .bind(&pedido.criado_em)
        .bind(&pedido.atualizado_em)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            println!("[PEDIDO] Erro ao inserir pedido: {}", e);
            e.to_string()
        })?;

        println!("[PEDIDO] Pedido inserido, processando {} itens", pedido.itens.len());
        
        for i in pedido.itens.iter() {
            // 1. Inserir o Item
            sqlx::query("
                INSERT INTO itens_pedido (
                    uuid,
                    pedido_uuid,
                    loja_uuid, 
                    quantidade,
                    observacoes
                )
                VALUES (?, ?, ?, ?, ?)
            ")
            .bind(i.uuid)
            .bind(i.pedido_uuid)
            .bind(i.loja_uuid)
            .bind(i.quantidade)
            .bind(&i.observacoes)
            .execute(&mut *tx) // Usa a transação existente
            .await
            .map_err(|e| {
                println!("[ERRO FK] Falha ao inserir item de pedido: {:?}. Erro: {}", i, e);
                e.to_string()
            })?;

            if !i.partes.is_empty() {
                for parte in i.partes.iter() {
                    sqlx::query("
                        INSERT INTO partes_item_pedido (
                            uuid,
                            loja_uuid,
                            item_uuid,
                            produto_uuid,
                            produto_nome,
                            preco_unitario,
                            posicao
                        )
                        VALUES (?, ?, ?, ?, ?, ?, ?);
                    ")
                    .bind(&parte.uuid)
                    .bind(&parte.loja_uuid)
                    .bind(&parte.item_uuid)
                    .bind(&parte.produto_uuid)
                    .bind(&parte.produto_nome)
                    .bind(&parte.preco_unitario)
                    .bind(&parte.posicao)
                    .execute(&mut *tx) // MUITO IMPORTANTE: &mut *tx aqui
                    .await
                    .map_err(|e| {
                        println!("[ERRO FK] Falha ao inserir parte de item: {:?}. Erro: {}", i, e);
                        println!("p: {:?}, i: {:?}", parte.posicao, parte.item_uuid);
                        e.to_string()
                    })?;
                }
            }

            // 3. Inserir Adicionais
            for a in i.adicionais.iter() {
                sqlx::query("
                    INSERT INTO adicionais_item_pedido (
                        uuid,
                        item_uuid,
                        loja_uuid,
                        nome,
                        descricao,
                        preco
                    )
                    VALUES (?, ?, ?, ?, ?, ?)
                ")
                .bind(a.uuid)
                .bind(a.item_uuid)
                .bind(a.loja_uuid)
                .bind(&a.nome)
                .bind(&a.descricao)
                .bind(a.preco)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    println!("[ERRO FK] Falha ao inserir adicional de item: {:?}. Erro: {}", i, e);
                    e.to_string()
                })?;
            }
        }

        println!("[PEDIDO] Commitando transação");
        tx.commit().await.map_err(|e| {
            println!("[PEDIDO] Erro no commit: {}", e);
            e.to_string()
        })?;

        println!("[PEDIDO] Transação commitada com sucesso uuid={}", pedido.uuid);
        Ok(pedido.uuid)
    }

    async fn atualizar(&self, item: Pedido) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE pedidos
            SET
                status = ?,
                total = ?,
                subtotal = ?,
                taxa_entrega = ?, 
                desconto = ?,
                forma_pagamento = ?,
                observacoes = ?,
                tempo_estimado_min = ?, 
                atualizado_em = ?
            WHERE uuid = ?
        ")
        .bind(item.status.to_string())
        .bind(item.total)
        .bind(item.subtotal)
        .bind(item.taxa_entrega)
        .bind(item.desconto)
        .bind(&item.forma_pagamento)
        .bind(&item.observacoes)
        .bind(item.tempo_estimado_min)
        .bind(item.atualizado_em)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Pedido não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM pedidos WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Pedido não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
    
    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("
                SELECT * FROM pedidos
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE ADICIONAIS ====================
pub struct AdicionalRepository { pool: Arc<SqlitePool> }
impl<'a> AdicionalRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("
            SELECT * FROM adicionais WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>(
            "SELECT * FROM adicionais WHERE loja_uuid = ? AND disponivel = true;"
        )
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Adicional> for AdicionalRepository {
    fn table_name(&self) -> String { "adicionais".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Adicional>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Adicional>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Adicional) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO adicionais (
                uuid,
                loja_uuid, 
                nome, 
                descricao, 
                preco, 
                disponivel, 
                criado_em
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(item.disponivel)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Adicional) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query(
            "UPDATE adicionais
                SET loja_uuid = ?,
                nome = ?,
                descricao = ?,
                preco = ?, 
                disponivel = ?
            WHERE uuid = ?
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(item.disponivel)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Adicional não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM adicionais WHERE uuid = ?
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Adicional não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("SELECT * FROM adicionais")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("
                SELECT * FROM adicionais
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE INGREDIENTES ====================
pub struct IngredienteRepository { pool: Arc<SqlitePool> }
impl<'a> IngredienteRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>("
            SELECT * FROM ingredientes
            WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>("
            SELECT * FROM ingredientes
            WHERE loja_uuid = ? AND quantidade > 0
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Ingrediente> for IngredienteRepository {
    fn table_name(&self) -> String { "ingredientes".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Ingrediente>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Ingrediente>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Ingrediente) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO ingredientes (
                uuid,
                loja_uuid,
                nome,
                unidade_medida,
                quantidade, 
                preco_unitario,
                criado_em,
                atualizado_em
            )
            VALUES (
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
            )
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.unidade_medida)
        .bind(item.quantidade)
        .bind(item.preco_unitario)
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Ingrediente) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE ingredientes
            SET
                loja_uuid = ?,
                nome = ?,
                unidade_medida = ?, 
                quantidade = ?,
                preco_unitario = ?,
                atualizado_em = ?
            WHERE uuid = ?
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.unidade_medida)
        .bind(item.quantidade)
        .bind(item.preco_unitario)
        .bind(item.atualizado_em)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Ingrediente não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM ingredientes WHERE uuid = ?
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Ingrediente não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>("SELECT * FROM ingredientes")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>("
                SELECT * FROM ingredientes
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE ENDEREÇOS DE LOJA ====================
pub struct EnderecoLojaRepository { pool: Arc<SqlitePool> }
impl<'a> EnderecoLojaRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoLoja>, String> {
        sqlx::query_as::<_, EnderecoLoja>("
            SELECT * FROM enderecos_loja WHERE loja_uuid = ?;
        "
        )
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<EnderecoLoja> for EnderecoLojaRepository {
    fn table_name(&self) -> String { "enderecos_loja".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<EnderecoLoja>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, EnderecoLoja>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &EnderecoLoja) -> Result<Uuid, String> {
        let uuid = item.get_uuid();
        sqlx::query("
            INSERT INTO enderecos_loja (
                uuid,
                loja_uuid,
                cep,
                logradouro,
                numero,
                complemento, 
                bairro,
                cidade,
                estado,
                latitude,
                longitude
            )
            VALUES (
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
            )
        ")
        .bind(uuid)
        .bind(item.loja_uuid)
        .bind(&item.cep)
        .bind(&item.logradouro)
        .bind(&item.numero)
        .bind(&item.complemento)
        .bind(&item.bairro)
        .bind(&item.cidade)
        .bind(&item.estado)
        .bind(item.latitude)
        .bind(item.longitude)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(uuid)
    }

    async fn atualizar(&self, item: EnderecoLoja) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE enderecos_loja
                SET
                    loja_uuid = ?,
                    cep = ?,
                    logradouro = ?,
                    numero = ?, 
                    complemento = ?,
                    bairro = ?,
                    cidade = ?,
                    estado = ?,
                    latitude = ?,
                    longitude = ? 
             WHERE uuid = ?
        ")
        .bind(item.loja_uuid)
        .bind(&item.cep)
        .bind(&item.logradouro)
        .bind(&item.numero)
        .bind(&item.complemento)
        .bind(&item.bairro)
        .bind(&item.cidade)
        .bind(&item.estado)
        .bind(item.latitude)
        .bind(item.longitude)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Endereço não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM enderecos_loja WHERE uuid = ?")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Endereço não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<EnderecoLoja>, String> {
        sqlx::query_as::<_, EnderecoLoja>("SELECT * FROM enderecos_loja")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoLoja>, String> {
        sqlx::query_as::<_, EnderecoLoja>("
                SELECT * FROM enderecos_loja
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE ENTREGADORES ====================
pub struct EntregadorRepository { pool: Arc<SqlitePool> }
impl<'a> EntregadorRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("
            SELECT * FROM entregadores WHERE loja_uuid = ?
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("
            SELECT * FROM entregadores
            WHERE loja_uuid = ? AND disponivel = true;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_telefone(&self, telefone: &str) -> Result<Option<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("
            SELECT * FROM entregadores WHERE telefone = ?;
        ")
        .bind(telefone)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Entregador> for EntregadorRepository {
    fn table_name(&self) -> String { "entregadores".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Entregador>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Entregador>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Entregador) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO entregadores (
                uuid,
                loja_uuid,
                nome,
                telefone,
                veiculo,
                placa, 
                disponivel,
                criado_em
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.telefone)
        .bind(&item.veiculo)
        .bind(&item.placa)
        .bind(item.disponivel)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Entregador) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE entregadores
            SET
                loja_uuid = ?,
                nome = ?,
                telefone = ?,
                veiculo = ?, 
                placa = ?,
                disponivel = ?
            WHERE uuid = ?
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.telefone)
        .bind(&item.veiculo)
        .bind(&item.placa)
        .bind(item.disponivel)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Entregador não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM entregadores WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Entregador não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("SELECT * FROM entregadores")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("
                SELECT * FROM entregadores
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

}

// ==================== REPOSITÓRIO DE FUNCIONÁRIOS ====================
pub struct FuncionarioRepository { pool: Arc<SqlitePool> }
impl<'a> FuncionarioRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("
            SELECT * FROM funcionarios WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_cargo(&self, cargo: &str, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("
            SELECT * FROM funcionarios
            WHERE loja_uuid = ? AND cargo = ?
        ")
        .bind(loja_uuid)
        .bind(cargo)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("
            SELECT * FROM funcionarios
            WHERE email = ?;
        ")
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Funcionario> for FuncionarioRepository {
    fn table_name(&self) -> String { "funcionarios".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Funcionario>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Funcionario>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Funcionario) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO funcionarios (
                uuid,
                loja_uuid,
                nome,
                email,
                cargo,
                salario, 
                data_admissao,
                criado_em
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.email)
        .bind(&item.cargo)
        .bind(item.salario)
        .bind(&item.data_admissao.to_string())
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Funcionario) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE funcionarios
            SET
                loja_uuid = ?,
                nome = ?,
                email = ?,
                cargo = ?, 
                salario = ?,
                data_admissao = ?
            WHERE uuid = ?;
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.email)
        .bind(&item.cargo)
        .bind(item.salario)
        .bind(item.data_admissao)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Funcionário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM funcionarios WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Funcionário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("SELECT * FROM funcionarios")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("
                SELECT * FROM funcionarios
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE AVALIAÇÕES DE LOJA ====================
pub struct AvaliacaoDeLojaRepository { pool: Arc<SqlitePool> }
impl<'a> AvaliacaoDeLojaRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("
            SELECT * FROM avaliacoes_loja WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("
            SELECT * FROM avaliacoes_loja WHERE usuario_uuid = ?;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn calcular_media(&self, loja_uuid: Uuid) -> Result<f64, String> {
        let result = sqlx::query("
            SELECT AVG(nota) as media FROM avaliacoes_loja
            WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get("media").unwrap_or(0.0))
    }

}

#[async_trait::async_trait]
impl<'a> Repository<AvaliacaoDeLoja> for AvaliacaoDeLojaRepository {
    fn table_name(&self) -> String { "avaliacoes_loja".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<AvaliacaoDeLoja>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, AvaliacaoDeLoja>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &AvaliacaoDeLoja) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO avaliacoes_loja (
                uuid,
                loja_uuid,
                usuario_uuid,
                nota,
                comentario,
                criado_em
            ) 
            VALUES (?, ?, ?, ?, ?, ?);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.usuario_uuid)
        .bind(item.nota)
        .bind(&item.comentario)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: AvaliacaoDeLoja) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE avaliacoes_loja
            SET 
                loja_uuid = ?,
                usuario_uuid = ?,
                nota = ?, 
                comentario = ?
            WHERE uuid = ?;
        ")
        .bind(item.loja_uuid)
        .bind(item.usuario_uuid)
        .bind(item.nota)
        .bind(&item.comentario)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliação não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM avaliacoes_loja WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliação não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("
                SELECT * FROM avaliacoes_loja
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE AVALIAÇÕES DE PRODUTO ====================
pub struct AvaliacaoDeProdutoRepository { pool: Arc<SqlitePool> }
impl<'a> AvaliacaoDeProdutoRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_produto(&self, produto_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("
            SELECT * FROM avaliacoes_produto
            WHERE produto_uuid = ?;
        ")
        .bind(produto_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("
            SELECT * FROM avaliacoes_produto
            WHERE usuario_uuid = ?;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("
            SELECT * FROM avaliacoes_produto
            WHERE pedido_uuid = ?;
        ")
        .bind(pedido_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn calcular_media(&self, produto_uuid: Uuid) -> Result<f64, String> {
        let result = sqlx::query("
            SELECT AVG(nota) as media FROM avaliacoes_produto
            WHERE produto_uuid = ?;
        ")
        .bind(produto_uuid)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get("media").unwrap_or(0.0))
    }
}

#[async_trait::async_trait]
impl<'a> Repository<AvaliacaoDeProduto> for AvaliacaoDeProdutoRepository {
    fn table_name(&self) -> String { "avaliacoes_produto".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<AvaliacaoDeProduto>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, AvaliacaoDeProduto>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &AvaliacaoDeProduto) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO avaliacoes_produto (
                uuid,
                usuario_uuid,
                loja_uuid,
                produto_uuid,
                nota,
                descricao,
                comentario,
                criado_em
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?);
        ")
        .bind(item.uuid)
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .bind(item.produto_uuid)
        .bind(item.nota)
        .bind(item.descricao.clone())
        .bind(&item.comentario)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: AvaliacaoDeProduto) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE avaliacoes_produto
            SET
                produto_uuid = ?,
                usuario_uuid = ?, 
                nota = ?,
                comentario = ?
            WHERE uuid = ?;
        ")
        .bind(item.usuario_uuid)
        .bind(item.produto_uuid)
        .bind(item.nota)
        .bind(&item.comentario)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliação não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM avaliacoes_produto
            WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliação não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("
            SELECT * FROM avaliacoes_produto;
        ")
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("
                SELECT * FROM avaliacoes_produto
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE CUPONS ====================
pub struct CupomRepository { pool: Arc<SqlitePool> }
impl<'a> CupomRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_codigo(&self, codigo: &str) -> Result<Option<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("
            SELECT * FROM cupons
            WHERE UPPER(codigo) = UPPER(?);
        ")
        .bind(codigo)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("
            SELECT * FROM cupons WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativos(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("
            SELECT * FROM cupons
            WHERE loja_uuid = ? AND status = ?;
        ")
        .bind(loja_uuid)
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Cupom> for CupomRepository {
    fn table_name(&self) -> String { "cupons".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Cupom>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Cupom>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Cupom) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO cupons (
                uuid,
                loja_uuid,
                codigo,
                descricao,
                tipo_desconto, 
                valor_desconto,
                valor_minimo,
                data_validade,
                limite_uso,
                status,
                criado_em
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.codigo)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.valor_minimo)
        .bind(&item.data_validade)
        .bind(item.limite_uso)
        .bind(item.status.to_string())
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Cupom) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE cupons
            SET 
                loja_uuid = ?,
                codigo = ?,
                descricao = ?,
                tipo_desconto = ?, 
                valor_desconto = ?,
                valor_minimo = ?,
                data_validade = ?,
                limite_uso = ?, 
                status = ?
            WHERE uuid = ?;
        ")
        .bind(item.loja_uuid)
        .bind(&item.codigo)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.valor_minimo)
        .bind(item.data_validade)
        .bind(item.limite_uso)
        .bind(item.status.to_string())
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cupom não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM cupons WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cupom não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("
                SELECT * FROM cupons
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE USO DE CUPONS ====================
pub struct UsoCupomRepository { pool: Arc<SqlitePool> }
impl<'a> UsoCupomRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("
            SELECT * FROM uso_cupons
            WHERE usuario_uuid = ?;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_cupom(&self, cupom_uuid: Uuid) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("
            SELECT * FROM uso_cupons
            WHERE cupom_uuid = ?;
        ")
        .bind(cupom_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn contar_usos_usuario(&self, usuario_uuid: Uuid, cupom_uuid: Uuid) -> Result<u32, String> {
        let result = sqlx::query("
            SELECT COUNT(*) as count FROM uso_cupons
            WHERE usuario_uuid = ? AND cupom_uuid = ?;
        ")
        .bind(usuario_uuid)
        .bind(cupom_uuid)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get::<i64, _>("count").unwrap_or(0) as u32)
    }
}

#[async_trait::async_trait]
impl<'a> Repository<UsoCupom> for UsoCupomRepository {
    fn table_name(&self) -> String { "uso_cupons".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<UsoCupom>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, UsoCupom>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &UsoCupom) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO uso_cupons (
                uuid,
                cupom_uuid,
                usuario_uuid,
                pedido_uuid,
                usado_em
            ) 
            VALUES (?, ?, ?, ?, ?);
        ")
        .bind(item.uuid)
        .bind(item.cupom_uuid)
        .bind(item.usuario_uuid)
        .bind(item.pedido_uuid)
        .bind(&item.usado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: UsoCupom) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE uso_cupons
            SET
                cupom_uuid = ?,
                usuario_uuid = ?,
                pedido_uuid = ?, 
                usado_em = ?
            WHERE uuid = ?;
        ")
        .bind(item.cupom_uuid)
        .bind(item.usuario_uuid)
        .bind(item.pedido_uuid)
        .bind(item.usado_em)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Uso de cupom não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM uso_cupons
            WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Uso de cupom não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("SELECT * FROM uso_cupons")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("
                SELECT * FROM uso_cupons
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE PROMOÇÕES ====================
pub struct PromocaoRepository { pool: Arc<SqlitePool> }
impl<'a> PromocaoRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("
            SELECT * FROM promocoes
            WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativas(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("
            SELECT * FROM promocoes
            WHERE loja_uuid = ? AND status = ?;
        ")
        .bind(loja_uuid)
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_prioridade(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("
            SELECT * FROM promocoes
            WHERE loja_uuid = ? AND status = ?
            ORDER BY prioridade DESC;
        ")
        .bind(loja_uuid)
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Promocao> for PromocaoRepository {
    fn table_name(&self) -> String { "promocoes".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Promocao>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, Promocao>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Promocao) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO promocoes (
                uuid,
                loja_uuid,
                nome,
                descricao,
                tipo_desconto, 
                valor_desconto,
                data_inicio,
                data_fim,
                prioridade,
                status,
                criado_em
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        ")
        .bind(&item.uuid)
        .bind(&item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(&item.tipo_desconto)
        .bind(&item.valor_desconto)
        .bind(&item.data_inicio)
        .bind(&item.data_fim)
        .bind(&item.prioridade)
        .bind(item.status.to_string())
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Promocao) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE promocoes
            SET
                loja_uuid = ?,
                nome = ?,
                descricao = ?,
                tipo_desconto = ?, 
                valor_desconto = ?,
                data_inicio = ?,
                data_fim = ?,
                prioridade = ?,
                status = ? 
             WHERE uuid = ?;
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.data_inicio)
        .bind(item.data_fim)
        .bind(item.prioridade)
        .bind(item.status.to_string())
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Promoção não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM promocoes
            WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Promoção não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("SELECT * FROM promocoes")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("
                SELECT * FROM promocoes
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}


// ==================== REPOSITÓRIO DE HORÁRIOS DE FUNCIONAMENTO ====================
pub struct HorarioFuncionamentoRepository { pool: Arc<SqlitePool> }
impl<'a> HorarioFuncionamentoRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    /// Busca todos os horários de uma loja, ordenados pelo dia da semana
    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("
            SELECT * FROM horarios_funcionamento
            WHERE loja_uuid = ?
            ORDER BY dia_semana ASC;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca o horário de um dia específico da loja
    pub async fn buscar_por_dia(
        &self,
        loja_uuid: Uuid,
        dia_semana: i32,
    ) -> Result<Option<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("
            SELECT * FROM horarios_funcionamento
            WHERE loja_uuid = ? AND dia_semana = ?;
        ")
        .bind(loja_uuid)
        .bind(dia_semana)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca apenas os dias ativos
    pub async fn buscar_ativos(&self, loja_uuid: Uuid) -> Result<Vec<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("
            SELECT * FROM horarios_funcionamento
            WHERE loja_uuid = ? AND ativo = TRUE
            ORDER BY dia_semana ASC;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Adiciona ou atualiza (upsert) o horário de um dia.
    /// Se já existir um horário para esse dia, sobrescreve com os novos valores.
    pub async fn adicionar_ou_atualizar(
        &self,
        horario: &HorarioFuncionamento,
    ) -> Result<(), String> {
        sqlx::query("
            INSERT INTO horarios_funcionamento (
                uuid,
                loja_uuid,
                dia_semana,
                abertura,
                fechamento,
                ativo,
                criado_em
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (loja_uuid, dia_semana)
            DO UPDATE SET
                abertura   = excluded.abertura,
                fechamento = excluded.fechamento,
                ativo      = excluded.ativo;
        ")
        .bind(&horario.uuid)
        .bind(&horario.loja_uuid)
        .bind(&horario.dia_semana)
        .bind(&horario.abertura)
        .bind(&horario.fechamento)
        .bind(horario.ativo)
        .bind(&horario.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Tenta inserir sem permitir sobrescrita — retorna erro se o dia já existir
    pub async fn adicionar_sem_sobrescrever(
        &self,
        horario: &HorarioFuncionamento,
    ) -> Result<(), String> {
        // Verifica duplicata explicitamente para dar mensagem clara
        let existe = self
            .buscar_por_dia(horario.loja_uuid, horario.dia_semana)
            .await?;

        if existe.is_some() {
            return Err(format!(
                "Já existe um horário cadastrado para {} nessa loja.",
                horario.nome_dia()
            ));
        }

        sqlx::query("
            INSERT INTO horarios_funcionamento (
                uuid,
                loja_uuid,
                dia_semana,
                abertura,
                fechamento,
                ativo,
                criado_em
            )
            VALUES (?, ?, ?, ?, ?, ?, ?);
        ")
        .bind(&horario.uuid)
        .bind(&horario.loja_uuid)
        .bind(horario.dia_semana)
        .bind(&horario.abertura)
        .bind(&horario.fechamento)
        .bind(horario.ativo)
        .bind(&horario.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Ativa ou desativa um dia sem apagar o registro
    pub async fn definir_ativo(
        &self,
        loja_uuid: Uuid,
        dia_semana: i32,
        ativo: bool,
    ) -> Result<(), String> {
        let result = sqlx::query("
            UPDATE horarios_funcionamento
            SET ativo = ?
            WHERE loja_uuid = ? AND dia_semana = ?;
        ")
        .bind(ativo)
        .bind(loja_uuid)
        .bind(dia_semana)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horário não encontrado".into())
        } else {
            Ok(())
        }
    }

    pub async fn deletar_por_dia(
        &self,
        loja_uuid: Uuid,
        dia_semana: i32,
    ) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM horarios_funcionamento
            WHERE loja_uuid = ? AND dia_semana = ?;
        ")
        .bind(loja_uuid)
        .bind(dia_semana)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horário não encontrado".into())
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl<'a> Repository<HorarioFuncionamento> for HorarioFuncionamentoRepository {
    fn table_name(&self) -> String { "horarios_funcionamento".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<HorarioFuncionamento>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);
        sqlx::query_as::<_, HorarioFuncionamento>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &HorarioFuncionamento) -> Result<Uuid, String> {
        self.adicionar_sem_sobrescrever(item).await?;
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: HorarioFuncionamento) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE horarios_funcionamento
            SET
                abertura = ?,
                fechamento = ?,
                ativo = ?
            WHERE uuid = ?;
        ")
        .bind(&item.abertura)
        .bind(&item.fechamento)
        .bind(item.ativo)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horário não encontrado".into())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM horarios_funcionamento WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horário não encontrado".into())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("
            SELECT * FROM horarios_funcionamento ORDER BY loja_uuid, dia_semana;
        ")
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("
                SELECT * FROM horarios_funcionamento
                WHERE loja_uuid = ?;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}


// ==================== REPOSITÓRIO DE CONFIGURAÇÃO DE PARTES ====================
pub struct ConfiguracaoPedidosLojaRepository { pool: Arc<SqlitePool> }

impl<'a> ConfiguracaoPedidosLojaRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    /// Busca a configuração de pedidos da loja (única por loja)
    pub async fn buscar_por_loja(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Option<ConfiguracaoDePedidosLoja>, String> {
        sqlx::query_as::<_, ConfiguracaoDePedidosLoja>("
            SELECT * FROM configuracoes_pedidos_loja
            WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Salva (insert) a configuração. Retorna erro se já existir uma.
    /// Use `atualizar_ou_criar` se quiser upsert.
    pub async fn criar_configuracao(
        &self,
        config: &ConfiguracaoDePedidosLoja,
    ) -> Result<(), String> {
        let existe = self.buscar_por_loja(config.loja_uuid).await?;
        if existe.is_some() {
            return Err(
                "Essa loja já possui uma configuração de pedidos. Use atualizar_ou_criar.".into()
            );
        }

        sqlx::query("
            INSERT INTO configuracoes_pedidos_loja (
                uuid,
                loja_uuid,
                max_pedidos,
                tipo_calculo,
                criado_em,
                atualizado_em
            )
            VALUES (?, ?, ?, ?, ?, ?);
        ")
        .bind(config.uuid)
        .bind(config.loja_uuid)
        .bind(config.max_partes)
        .bind(config.tipo_calculo.to_string())
        .bind(&config.criado_em)
        .bind(&config.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Upsert: cria se não existir, atualiza se já existir.
    pub async fn salvar(
        &self,
        config: &ConfiguracaoDePedidosLoja,
    ) -> Result<(), String> {
        sqlx::query("
            INSERT INTO configuracoes_pedidos_loja (
                uuid,
                loja_uuid,
                max_partes,
                tipo_calculo,
                criado_em,
                atualizado_em
            )
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT (loja_uuid) DO UPDATE SET
                max_partes   = excluded.max_partes,
                tipo_calculo  = excluded.tipo_calculo,
                atualizado_em = excluded.atualizado_em;
        ")
        .bind(config.uuid)
        .bind(config.loja_uuid)
        .bind(config.max_partes)
        .bind(config.tipo_calculo.to_string())
        .bind(&config.criado_em)
        .bind(&config.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Troca apenas o tipo de cálculo sem recriar toda a config
    pub async fn alterar_tipo_calculo(
        &self,
        loja_uuid: Uuid,
        novo_tipo: TipoCalculoPedido,
    ) -> Result<(), String> {
        let result = sqlx::query("
            UPDATE configuracoes_pedidos_loja
            SET tipo_calculo = ?, atualizado_em = ?
            WHERE loja_uuid = ?;
        ")
        .bind(novo_tipo.to_string())
        .bind(agora())
        .bind(loja_uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Configuração não encontrada para essa loja".into())
        } else {
            Ok(())
        }
    }

    /// Troca apenas o máximo de partes
    pub async fn alterar_max_partes(
        &self,
        loja_uuid: Uuid,
        novo_max: i32,
    ) -> Result<(), String> {
        if novo_max < 1 {
            return Err("max_partes deve ser >= 1".into());
        }

        let result = sqlx::query("
            UPDATE configuracoes_pedidos_loja
            SET
                max_partes = ?,
                atualizado_em = ?
            WHERE loja_uuid = ?;
        ")
        .bind(novo_max)
        .bind(agora())
        .bind(loja_uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Configuração não encontrada para essa loja".into())
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl<'a> Repository<ConfiguracaoDePedidosLoja> for ConfiguracaoPedidosLojaRepository {
    fn table_name(&self) -> String { "configuracoes_pedidos_loja".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<ConfiguracaoDePedidosLoja>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = ?;", t);        
        sqlx::query_as::<_, ConfiguracaoDePedidosLoja>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &ConfiguracaoDePedidosLoja) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO configuracoes_pedidos_loja (
                uuid,
                loja_uuid,
                max_partes,
                tipo_calculo,
                criado_em,
                atualizado_em
            )
            VALUES (?, ?, ?, ?, ?, ?);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.max_partes)
        .bind(item.tipo_calculo.to_string())
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: ConfiguracaoDePedidosLoja) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE configuracoes_pedidos_loja
            SET
                loja_uuid = ?,
                max_partes = ?,
                tipo_calculo = ?,
                atualizado_em = ?
            WHERE uuid = ?;
        ")
        .bind(item.loja_uuid)
        .bind(item.max_partes)
        .bind(item.tipo_calculo.to_string())
        .bind(&item.atualizado_em)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Configuração de pedidos não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM configuracoes_pedidos_loja WHERE uuid = ?;
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Configuração de pedidos não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<ConfiguracaoDePedidosLoja>, String> {
        let query = format!(
            "SELECT * FROM {};",
            self.table_name()
        );
        
        sqlx::query_as::<_, ConfiguracaoDePedidosLoja>(&query)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<ConfiguracaoDePedidosLoja>, String> {
        // Como há apenas 1 configuração por loja, retorna Vec com 0 ou 1 elemento
        sqlx::query_as::<_, ConfiguracaoDePedidosLoja>("
            SELECT * FROM configuracoes_pedidos_loja
            WHERE loja_uuid = ?;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
// ==================== REPOSITÓRIO DE PARTES DE ITEM ====================
pub struct ParteDeItemPedidoRepository { pool: Arc<SqlitePool> }
impl<'a> ParteDeItemPedidoRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self { Self { pool } }

    pub async fn buscar_por_item(
        &self,
        item_uuid: Uuid,
    ) -> Result<Vec<ParteDeItemPedido>, String> {
        sqlx::query_as::<_, ParteDeItemPedido>("
            SELECT * FROM partes_item_pedido
            WHERE item_uuid = ?
            ORDER BY posicao ASC;
        ")
        .bind(item_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Insere todos as partes de um item dentro de uma transação.
    /// Valida contra a configuração da loja antes de inserir.
    pub async fn salvar_partes_do_item(
        &self,
        partes: &[ParteDeItemPedido],
        config: &ConfiguracaoDePedidosLoja,
    ) -> Result<f64, String> {
        // if partes.is_empty() {
        //     return Err("Lista de partes não pode ser vazia".into());
        // }

        if partes.len() as i32 > config.max_partes {
            return Err(format!(
                "Máximo de {} partes permitido, recebeu {}",
                config.max_partes,
                partes.len()
            ));
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| e.to_string())?;

        for parte in partes {
            sqlx::query("
                INSERT INTO partes_item_pedido (
                    uuid,
                    item_uuid,
                    produto_nome,
                    preco_unitario,
                    posicao
                )
                VALUES (?, ?, ?, ?, ?);
            ")
            .bind(&parte.uuid)
            .bind(&parte.item_uuid)
            .bind(&parte.produto_nome)
            .bind(parte.preco_unitario)
            .bind(parte.posicao)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        // Retorna o preço calculado conforme a configuração da loja
        let preco = calcular_preco_por_partes(partes, &config.tipo_calculo);
        Ok(preco)
    }

    pub async fn deletar_por_item(&self, item_uuid: Uuid) -> Result<(), String> {
        sqlx::query("DELETE FROM partes_item_pedido WHERE item_uuid = ?;")
            .bind(item_uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
