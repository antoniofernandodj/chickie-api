use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{ConfiguracaoDePedidosLoja, Model, TipoCalculoPedido}, repositories::Repository, utils::agora};

pub struct ConfiguracaoPedidosLojaRepository { pool: Arc<PgPool> }

impl ConfiguracaoPedidosLojaRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    /// Busca a configuracao de pedidos da loja (unica por loja)
    pub async fn buscar_por_loja(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Option<ConfiguracaoDePedidosLoja>, String> {
        sqlx::query_as::<_, ConfiguracaoDePedidosLoja>("SELECT * FROM configuracoes_pedidos_loja WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    /// Salva (insert) a configuracao. Retorna erro se ja existir uma.
    /// Use `salvar` se quiser upsert.
    pub async fn criar_configuracao(
        &self,
        config: &ConfiguracaoDePedidosLoja,
    ) -> Result<(), String> {
        let existe = self.buscar_por_loja(config.loja_uuid).await?;
        if existe.is_some() {
            return Err(
                "Essa loja ja possui uma configuracao de pedidos. Use salvar.".into()
            );
        }

        sqlx::query("
            INSERT INTO configuracoes_pedidos_loja (uuid, loja_uuid, max_partes, tipo_calculo, criado_em, atualizado_em)
            VALUES ($1, $2, $3, $4, $5, $6);
        ")
        .bind(config.uuid)
        .bind(config.loja_uuid)
        .bind(config.max_partes)
        .bind(config.tipo_calculo.to_string())
        .bind(&config.criado_em)
        .bind(&config.atualizado_em)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Upsert: cria se nao existir, atualiza se ja existir.
    pub async fn salvar(
        &self,
        config: &ConfiguracaoDePedidosLoja,
    ) -> Result<(), String> {
        sqlx::query("
            INSERT INTO configuracoes_pedidos_loja (uuid, loja_uuid, max_partes, tipo_calculo, criado_em, atualizado_em)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (loja_uuid) DO UPDATE SET max_partes = excluded.max_partes, tipo_calculo = excluded.tipo_calculo, atualizado_em = excluded.atualizado_em;
        ")
        .bind(config.uuid)
        .bind(config.loja_uuid)
        .bind(config.max_partes)
        .bind(config.tipo_calculo.to_string())
        .bind(&config.criado_em)
        .bind(&config.atualizado_em)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Troca apenas o tipo de calculo sem recriar toda a config
    pub async fn alterar_tipo_calculo(
        &self,
        loja_uuid: Uuid,
        novo_tipo: TipoCalculoPedido,
    ) -> Result<(), String> {
        let result = sqlx::query("
            UPDATE configuracoes_pedidos_loja
            SET tipo_calculo = $1, atualizado_em = $2
            WHERE loja_uuid = $3;
        ")
        .bind(novo_tipo.to_string())
        .bind(agora())
        .bind(loja_uuid)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Configuracao no encontrada para essa loja".into())
        } else {
            Ok(())
        }
    }

    /// Troca apenas o maximo de partes
    pub async fn alterar_max_partes(
        &self,
        loja_uuid: Uuid,
        novo_max: i32,
    ) -> Result<(), String> {
        if novo_max < 1 {
            return Err("max_partes deve ser >= 1".into());
        }

        let result = sqlx::query("
            UPDATE configuracoes_pedidos_loja SET max_partes = $1, atualizado_em = $2 WHERE loja_uuid = $3;
        ")
        .bind(novo_max)
        .bind(agora())
        .bind(loja_uuid)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Configuracao no encontrada para essa loja".into())
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl Repository<ConfiguracaoDePedidosLoja> for ConfiguracaoPedidosLojaRepository {
    fn table_name(&self) -> &'static str { "configuracoes_pedidos_loja" }
    fn entity_name(&self) -> &'static str { "Configuracao de pedidos" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &ConfiguracaoDePedidosLoja) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO configuracoes_pedidos_loja (uuid, loja_uuid, max_partes, tipo_calculo, criado_em, atualizado_em)
            VALUES ($1, $2, $3, $4, $5, $6);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.max_partes)
        .bind(item.tipo_calculo.to_string())
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: ConfiguracaoDePedidosLoja) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE configuracoes_pedidos_loja SET loja_uuid = $1, max_partes = $2, tipo_calculo = $3, atualizado_em = $4
            WHERE uuid = $5
        ")
        .bind(item.loja_uuid)
        .bind(item.max_partes)
        .bind(item.tipo_calculo.to_string())
        .bind(&item.atualizado_em)
        .bind(uuid)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err(format!("{} não encontrad{}", self.entity_name(), self.entity_gender_suffix()))
        } else {
            Ok(())
        }
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<ConfiguracaoDePedidosLoja>, String> {
        // Como ha apenas 1 configuracao por loja, retorna Vec com 0 ou 1 elemento
        sqlx::query_as::<_, ConfiguracaoDePedidosLoja>("SELECT * FROM configuracoes_pedidos_loja WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}
