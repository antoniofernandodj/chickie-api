use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,QueryOrder,ConnectionTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::horarios_funcionamento::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;
use chrono::NaiveTime;

pub struct HorarioFuncionamentoRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl HorarioFuncionamentoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    /// Busca todos os horarios de uma loja, ordenados pelo dia da semana
    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        horarios_funcionamento::Entity::find()
            .filter(horarios_funcionamento::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .order_by_asc(horarios_funcionamento::Column::DiaSemana)
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Busca o horario de um dia especifico da loja
    pub async fn buscar_por_dia(&self, loja_uuid: Uuid, dia_semana: i32) -> Result<Option<Model>, String> {
        horarios_funcionamento::Entity::find()
            .filter(horarios_funcionamento::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(horarios_funcionamento::Column::DiaSemana.eq(dia_semana))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Busca apenas os dias ativos
    pub async fn buscar_ativos(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        horarios_funcionamento::Entity::find()
            .filter(horarios_funcionamento::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(horarios_funcionamento::Column::Ativo.eq(true))
            .order_by_asc(horarios_funcionamento::Column::DiaSemana)
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Adiciona ou atualiza (upsert) o horario de um dia
    pub async fn adicionar_ou_atualizar(&self, horario: &Model) -> Result<(), String> {
        // Uses raw SQL for upsert as sea-orm doesn't have native UPSERT support
        let sql = format!(
            "INSERT INTO horarios_funcionamento (uuid, loja_uuid, dia_semana, abertura, fechamento, ativo)
             VALUES ('{}', '{}', {}, '{}', '{}', {})
             ON CONFLICT (loja_uuid, dia_semana) DO UPDATE SET abertura = excluded.abertura, fechamento = excluded.fechamento, ativo = excluded.ativo",
            horario.uuid, horario.loja_uuid, horario.dia_semana, 
            horario.abertura.format("%H:%M"), horario.fechamento.format("%H:%M"), horario.ativo
        );
        
        self.db.execute_unprepared(&sql)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    /// Tenta inserir sem permitir sobrescrita
    pub async fn adicionar_sem_sobrescrever(&self, horario: &Model) -> Result<(), String> {
        let existe = self.buscar_por_dia(horario.loja_uuid, horario.dia_semana).await?;
        if existe.is_some() {
            return Err(format!(
                "Ja existe um horario cadastrado para dia {} nessa loja.",
                horario.dia_semana
            ));
        }

        let active: ActiveModel = ActiveModel {
            uuid: Set(SeaUuid::from(horario.uuid)),
            loja_uuid: Set(SeaUuid::from(horario.loja_uuid)),
            dia_semana: Set(horario.dia_semana),
            abertura: Set(horario.abertura),
            fechamento: Set(horario.fechamento),
            ativo: Set(horario.ativo),
            criado_em: Set(horario.criado_em),
        };
        
        active.insert(&*self.db)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    /// Ativa ou desativa um dia sem apagar o registro
    pub async fn definir_ativo(&self, loja_uuid: Uuid, dia_semana: i32, ativo: bool) -> Result<(), String> {
        let sql = format!(
            "UPDATE horarios_funcionamento SET ativo = {} WHERE loja_uuid = '{}' AND dia_semana = {}",
            ativo, loja_uuid, dia_semana
        );
        
        let result = self.db.execute_unprepared(&sql)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horario nao encontrado".into())
        } else {
            Ok(())
        }
    }

    pub async fn deletar_por_dia(&self, loja_uuid: Uuid, dia_semana: i32) -> Result<(), String> {
        let sql = format!(
            "DELETE FROM horarios_funcionamento WHERE loja_uuid = '{}' AND dia_semana = {}",
            loja_uuid, dia_semana
        );
        
        let result = self.db.execute_unprepared(&sql)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horario nao encontrado".into())
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for HorarioFuncionamentoRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        horarios_funcionamento::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Horário" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
