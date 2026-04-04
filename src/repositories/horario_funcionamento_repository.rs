use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{HorarioFuncionamento, Model}, repositories::Repository};

pub struct HorarioFuncionamentoRepository { pool: Arc<PgPool> }

impl HorarioFuncionamentoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    /// Busca todos os horarios de uma loja, ordenados pelo dia da semana
    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("SELECT * FROM horarios_funcionamento WHERE loja_uuid = $1 ORDER BY dia_semana ASC")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca o horario de um dia especifico da loja
    pub async fn buscar_por_dia(
        &self,
        loja_uuid: Uuid,
        dia_semana: i32,
    ) -> Result<Option<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("SELECT * FROM horarios_funcionamento WHERE loja_uuid = $1 AND dia_semana = $2")
        .bind(loja_uuid)
        .bind(dia_semana)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca apenas os dias ativos
    pub async fn buscar_ativos(&self, loja_uuid: Uuid) -> Result<Vec<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("SELECT * FROM horarios_funcionamento WHERE loja_uuid = $1 AND ativo = TRUE ORDER BY dia_semana ASC")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Adiciona ou atualiza (upsert) o horario de um dia.
    /// Se ja existir um horario para esse dia, sobrescreve com os novos valores.
    pub async fn adicionar_ou_atualizar(
        &self,
        horario: &HorarioFuncionamento,
    ) -> Result<(), String> {
        sqlx::query("
            INSERT INTO horarios_funcionamento (uuid, loja_uuid, dia_semana, abertura, fechamento, ativo, criado_em)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (loja_uuid, dia_semana) DO UPDATE SET abertura = excluded.abertura, fechamento = excluded.fechamento, ativo = excluded.ativo;
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

    /// Tenta inserir sem permitir sobrescrita -- retorna erro se o dia ja existir
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
                "Ja existe um horario cadastrado para {} nessa loja.",
                horario.nome_dia()
            ));
        }

        sqlx::query("
            INSERT INTO horarios_funcionamento (uuid, loja_uuid, dia_semana, abertura, fechamento, ativo, criado_em)
            VALUES ($1, $2, $3, $4, $5, $6, $7);
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
            SET ativo = $1
            WHERE loja_uuid = $2 AND dia_semana = $3;
        ")
        .bind(ativo)
        .bind(loja_uuid)
        .bind(dia_semana)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horario no encontrado".into())
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
            WHERE loja_uuid = $1 AND dia_semana = $2;
        ")
        .bind(loja_uuid)
        .bind(dia_semana)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horario no encontrado".into())
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
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
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
            UPDATE horarios_funcionamento SET abertura = $1, fechamento = $2, ativo = $3 WHERE uuid = $4
        ")
        .bind(&item.abertura)
        .bind(&item.fechamento)
        .bind(item.ativo)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horario no encontrado".into())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM horarios_funcionamento WHERE uuid = $1")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Horario no encontrado".into())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("SELECT * FROM horarios_funcionamento ORDER BY loja_uuid, dia_semana")
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<HorarioFuncionamento>, String> {
        sqlx::query_as::<_, HorarioFuncionamento>("SELECT * FROM horarios_funcionamento WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
