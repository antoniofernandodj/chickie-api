use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Model, Pedido, EstadoDePedido, ItemPedido, AdicionalDeItemDePedido, ParteDeItemPedido}, repositories::Repository};

pub struct PedidoRepository { pool: Arc<PgPool> }

impl PedidoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos WHERE usuario_uuid = $1")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_status(&self, status: EstadoDePedido) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE status = $1"
        )
        .bind(status.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_pendentes(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos WHERE loja_uuid = $1 AND (status = $2 OR status = $3)")
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
        let mut pedido = match sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos WHERE uuid = $1")
        .bind(uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())?
        {
            Some(p) => p,
            None => return Ok(None),
        };

        // 2. Busca todos os itens do pedido
        let mut itens = sqlx::query_as::<_, ItemPedido>("SELECT * FROM itens_pedido WHERE pedido_uuid = $1 ORDER BY criado_em ASC")
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

        let partes = sqlx::query_as::<_, ParteDeItemPedido>("SELECT * FROM partes_item_pedido WHERE item_uuid = $1 ORDER BY posicao ASC")
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

        let adicionais = sqlx::query_as::<_, AdicionalDeItemDePedido>("SELECT * FROM adicionais_item_pedido WHERE item_uuid = $1")
        .bind(item.uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string());

        adicionais
    }

    /// Mesma logica mas para multiplos pedidos (ex: listar pedidos de uma loja)
    pub async fn buscar_completos_por_loja(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<Pedido>, String> {
        let pedidos = sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos WHERE loja_uuid = $1 ORDER BY criado_em DESC")
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
        let pedidos = sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos WHERE usuario_uuid = $1 ORDER BY criado_em DESC")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        self.hidratar_pedidos(pedidos).await
    }

    /// Extrai a logica de hidratacao para reuso entre os metodos acima
    async fn hidratar_pedidos(
        &self,
        pedidos: Vec<Pedido>,
    ) -> Result<Vec<Pedido>, String> {
        if pedidos.is_empty() {
            return Ok(vec![]);
        }

        // Coleta todos os UUIDs dos pedidos para buscar itens em uma so query
        let uuids_pedidos: Vec<String> = pedidos
            .iter()
            .map(|p| format!("'{}'", p.uuid))
            .collect();

        let mut itens = // Seguro e idiomatico PostgreSQL
            sqlx::query_as::<_, ItemPedido>("SELECT * FROM itens_pedido WHERE pedido_uuid = ANY($1) ORDER BY pedido_uuid, criado_em ASC")
            .bind(&uuids_pedidos)  // &[Uuid]
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        // Busca adicionais e partes de todos os itens em duas queries unicas
        if !itens.is_empty() {
            let uuids_itens: Vec<String> = itens
                .iter()
                .map(|i| format!("'{}'", i.uuid))
                .collect();
            let placeholder_itens = uuids_itens.join(", ");

            let adicionais = sqlx::query_as::<_, AdicionalDeItemDePedido>(&format!(
                "SELECT * FROM adicionais_item_pedido WHERE item_uuid IN ({})", placeholder_itens
            ))
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let partes = sqlx::query_as::<_, ParteDeItemPedido>(&format!(
                "SELECT * FROM partes_item_pedido WHERE item_uuid IN ({}) ORDER BY item_uuid, posicao ASC", placeholder_itens
            ))
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
impl Repository<Pedido> for PedidoRepository {
    fn table_name(&self) -> &'static str { "pedidos" }
    fn entity_name(&self) -> &'static str { "Pedido" }
    fn pool(&self) -> &PgPool { &*self.pool }

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
            INSERT INTO pedidos (uuid, usuario_uuid, loja_uuid, status, total, subtotal, taxa_entrega, desconto, forma_pagamento, observacoes, tempo_estimado_min, criado_em, atualizado_em)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
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
                INSERT INTO itens_pedido (uuid, pedido_uuid, loja_uuid, quantidade, observacoes)
                VALUES ($1, $2, $3, $4, $5)
            ")
            .bind(i.uuid)
            .bind(i.pedido_uuid)
            .bind(i.loja_uuid)
            .bind(i.quantidade)
            .bind(&i.observacoes)
            .execute(&mut *tx) // Usa a transacao existente
            .await
            .map_err(|e| {
                println!("[ERRO FK] Falha ao inserir item de pedido: {:?}. Erro: {}", i, e);
                e.to_string()
            })?;

            if !i.partes.is_empty() {
                for parte in i.partes.iter() {
                    sqlx::query("
                        INSERT INTO partes_item_pedido (uuid, loja_uuid, item_uuid, produto_uuid, produto_nome, preco_unitario, posicao)
                        VALUES ($1, $2, $3, $4, $5, $6, $7);
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
                    INSERT INTO adicionais_item_pedido (uuid, item_uuid, loja_uuid, nome, descricao, preco)
                    VALUES ($1, $2, $3, $4, $5, $6)
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

        println!("[PEDIDO] Commitando transacao");
        tx.commit().await.map_err(|e| {
            println!("[PEDIDO] Erro no commit: {}", e);
            e.to_string()
        })?;

        println!("[PEDIDO] Transacao commitada com sucesso uuid={}", pedido.uuid);
        Ok(pedido.uuid)
    }

    async fn atualizar(&self, item: Pedido) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE pedidos SET status = $1, total = $2, subtotal = $3, taxa_entrega = $4, desconto = $5, forma_pagamento = $6, observacoes = $7, tempo_estimado_min = $8, atualizado_em = $9
            WHERE uuid = $10
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
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err(format!("{} não encontrad{}", self.entity_name(), self.entity_gender_suffix()))
        } else {
            Ok(())
        }
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}
