use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::Loja, repositories::Repository};

pub struct LojaRepository { pool: Arc<PgPool> }

impl LojaRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Loja>, String> {
        sqlx::query_as::<_, Loja>("
            SELECT * FROM lojas WHERE email = $1;
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
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
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
                $1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10,
                $11,  $12,  $13,  $14,  $15,  $16,  $17,  $18
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
                nome = $1,
                slug = $2,
                descricao = $3,
                email = $4,
                telefone = $5,
                ativa = $6,
                logo_url = $7,
                banner_url = $8,
                horario_abertura = $9,
                horario_fechamento = $10,
                dias_funcionamento = $11,
                tempo_preparo_min = $12,
                taxa_entrega = $13,
                valor_minimo_pedido = $14,
                raio_entrega_km = $15,
                atualizado_em = $16
            WHERE uuid = $17
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
        let result = sqlx::query("DELETE FROM lojas WHERE uuid = $1")
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
        Err("não se aplica".into())
    }
}
