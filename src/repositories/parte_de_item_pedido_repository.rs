use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set, QueryOrder, TransactionTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::partes_item_pedido::{self, Entity, Model, ActiveModel},
    entities::configuracoes_pedidos_loja::Model as ConfigModel,
};
use sea_orm::prelude::Uuid as SeaUuid;
use rust_decimal::Decimal;
use crate::models::calcular_preco_por_partes;

/// Repositório de partes de item de pedido.
/// NOTE: Este repositorio NAO implementa o trait Repository<T>.
/// Ele possui apenas metodos especificos para o dominio.
pub struct ParteDeItemPedidoRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl ParteDeItemPedidoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_item(&self, item_uuid: Uuid) -> Result<Vec<Model>, String> {
        partes_item_pedido::Entity::find()
            .filter(partes_item_pedido::Column::ItemUuid.eq(SeaUuid::from(item_uuid)))
            .order_by_asc(partes_item_pedido::Column::Posicao)
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Insere todos as partes de um item dentro de uma transacao.
    /// Valida contra a configuracao da loja antes de inserir.
    pub async fn salvar_partes_do_item(
        &self,
        partes: &[Model],
        config: &ConfigModel,
    ) -> Result<Decimal, String> {
        if partes.len() as i32 > config.max_partes {
            return Err(format!(
                "Maximo de {} partes permitido, recebeu {}",
                config.max_partes,
                partes.len()
            ));
        }

        let txn = self.db.begin().await
            .map_err(|e| e.to_string())?;

        for parte in partes {
            let mut active: ActiveModel = parte.clone().into();
            active.insert(&txn)
                .await
                .map_err(|e| e.to_string())?;
        }

        txn.commit().await
            .map_err(|e| e.to_string())?;

        // Retorna o preco calculado conforme a configuracao da loja
        let preco = calcular_preco_por_partes(partes, &config.tipo_calculo);
        Ok(preco)
    }

    pub async fn deletar_por_item(&self, item_uuid: Uuid) -> Result<(), String> {
        let sql = format!(
            "DELETE FROM partes_item_pedido WHERE item_uuid = '{}'",
            item_uuid
        );
        
        self.db.execute_unprepared(&sql)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }
}
