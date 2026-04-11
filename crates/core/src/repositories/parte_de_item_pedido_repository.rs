use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::{
    models::{ParteDeItemPedido, ConfiguracaoDePedidosLoja},
    models::calcular_preco_por_partes
};

/// Repositório de partes de item de pedido.
/// NOTE: Este repositorio NAO implementa o trait Repository<T>.
/// Ele possui apenas metodos especificos para o dominio.
pub struct ParteDeItemPedidoRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl ParteDeItemPedidoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_item(
        &self,
        item_uuid: Uuid,
    ) -> Result<Vec<ParteDeItemPedido>, String> {
        sqlx::query_as::<_, ParteDeItemPedido>("SELECT * FROM partes_item_pedido WHERE item_uuid = $1 ORDER BY posicao ASC")
        .bind(item_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Insere todos as partes de um item dentro de uma transacao.
    /// Valida contra a configuracao da loja antes de inserir.
    pub async fn salvar_partes_do_item(
        &self,
        partes: &[ParteDeItemPedido],
        config: &ConfiguracaoDePedidosLoja,
    ) -> Result<Decimal, String> {
        // if partes.is_empty() {
        //     return Err("Lista de partes nao pode ser vazia".into());
        // }

        if partes.len() as i32 > config.max_partes {
            return Err(format!(
                "Maximo de {} partes permitido, recebeu {}",
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
                INSERT INTO partes_item_pedido (uuid, item_uuid, produto_nome, preco_unitario, posicao)
                VALUES ($1, $2, $3, $4, $5);
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

        // Retorna o preco calculado conforme a configuracao da loja
        let preco = calcular_preco_por_partes(partes, &config.tipo_calculo);
        Ok(preco)
    }

    pub async fn deletar_por_item(&self, item_uuid: Uuid) -> Result<(), String> {
        sqlx::query("DELETE FROM partes_item_pedido WHERE item_uuid = $1;")
            .bind(item_uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
