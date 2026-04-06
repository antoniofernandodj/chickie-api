use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,ConnectionTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::configuracoes_pedidos_loja::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct ConfiguracaoPedidosLojaRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl ConfiguracaoPedidosLojaRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    /// Busca a configuracao de pedidos da loja (unica por loja)
    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Option<Model>, String> {
        configuracoes_pedidos_loja::Entity::find()
            .filter(configuracoes_pedidos_loja::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Upsert: cria se nao existir, atualiza se ja existir
    pub async fn salvar(&self, config: &Model) -> Result<(), String> {
        let sql = format!(
            "INSERT INTO configuracoes_pedidos_loja (uuid, loja_uuid, max_partes, tipo_calculo)
             VALUES ('{}', '{}', {}, '{}')
             ON CONFLICT (loja_uuid) DO UPDATE SET max_partes = excluded.max_partes, tipo_calculo = excluded.tipo_calculo",
            config.uuid, config.loja_uuid, config.max_partes, config.tipo_calculo
        );
        
        self.db.execute_unprepared(&sql)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    /// Troca apenas o tipo de calculo sem recriar toda a config
    pub async fn alterar_tipo_calculo(&self, loja_uuid: Uuid, novo_tipo: String) -> Result<(), String> {
        let sql = format!(
            "UPDATE configuracoes_pedidos_loja SET tipo_calculo = '{}' WHERE loja_uuid = '{}'",
            novo_tipo, loja_uuid
        );
        
        let result = self.db.execute_unprepared(&sql)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Configuracao nao encontrada para essa loja".into())
        } else {
            Ok(())
        }
    }

    /// Troca apenas o maximo de partes
    pub async fn alterar_max_partes(&self, loja_uuid: Uuid, novo_max: i32) -> Result<(), String> {
        if novo_max < 1 {
            return Err("max_partes deve ser >= 1".into());
        }

        let sql = format!(
            "UPDATE configuracoes_pedidos_loja SET max_partes = {} WHERE loja_uuid = '{}'",
            novo_max, loja_uuid
        );
        
        let result = self.db.execute_unprepared(&sql)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Configuracao nao encontrada para essa loja".into())
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for ConfiguracaoPedidosLojaRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        configuracoes_pedidos_loja::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Configuracao de pedidos" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        configuracoes_pedidos_loja::Entity::find()
            .filter(configuracoes_pedidos_loja::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}
