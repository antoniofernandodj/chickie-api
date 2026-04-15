# Roadmap — Scheduler & Jobs Assíncronos

> Planejamento completo de scheduler (time-triggered) e jobs assíncronos via message queue (event-triggered) para a Chickie API.
>
> **Data:** 2026-04-05 | **Status:** Planejado (não iniciado)

---

## Visão Geral

Atualmente a Chickie API opera **100% de forma síncrona** — toda ação é triggerada por uma requisição HTTP e completa dentro do request. Este documento cataloga todas as oportunidades identificadas para automação via scheduler e jobs assíncronos.

### Infraestrutura Atual

| Componente | Status |
|------------|--------|
| Scheduler/Cron | ❌ Inexistente |
| Message Queue (RMQ/Redis) | ❌ Inexistente |
| Background Workers | ❌ Inexistente |

### Microserviço Futuro

Conforme documentação, o **ChickieWorker** é o microserviço planejado para "tarefas assíncronas em background".

---

## 🔧 Scheduler Jobs (Time-Triggered)

Executados periodicamente via cron. Sugerido: crate `tokio-cron-scheduler` (zero dependência externa).

### 🔴 Críticos (antes de produção)

| # | Job | O que faz | Entidades | Frequência | Esforço |
|---|-----|-----------|-----------|------------|---------|
| 1 | **Auto-expirar cupons** | Scan `cupons` onde `data_validade < NOW()` → `status = 'expirado'` | `cupons` | 5-15 min | Baixo |
| 2 | **Auto-expirar promoções** | Scan `promocoes` onde `data_fim < NOW()` → `status = 'expirado'` | `promocoes` | 5-15 min | Baixo |

**Por quê é crítico:** Cupons/promoções vencidos ainda aparecem como "ativo" no admin. O model tem `StatusCupom::Expirado` mas nada faz a transição automaticamente.

### 🟡 Importantes

| # | Job | O que faz | Entidades | Frequência | Esforço |
|---|-----|-----------|-----------|------------|---------|
| 3 | **Soft-delete de usuários** | Users com `a_remover <= NOW() - 30d` → marca `excluida = true` | `usuarios` (precisa de migration) | Diário | Médio |
| 4 | **Soft-delete de lojas** | Lojas com `a_remover <= NOW() - 30d` → marca `excluida = true` | `lojas` (precisa de migration) | Diário | Médio |
| 5 | **Auto-cancelar pedidos órfãos** | Pedidos em `criado`/`aguardando_confirmacao` por >X min → cancela | `pedidos` | 5 min | Médio |
| 6 | **Auto-atribuir entregador** | Pedidos em `saiu_para_entrega` sem entregador → atribui primeiro `disponivel = true` | `pedidos`, `entregadores` | 1-2 min | Médio |

**Notas:**
- Jobs 3 e 4: pendências #10 e #11 no `pendencias.md`. Requer migration para adicionar colunas `a_remover TIMESTAMPTZ` e `excluida BOOLEAN`.
- Job 5: sem timeout, pedido nunca confirmado fica pendurado indefinidamente.
- Job 6: pendência #5 — sem endpoint para atribuir entregador.

### 🟢 Nice-to-have

| # | Job | O que faz | Entidades | Frequência |
|---|-----|-----------|-----------|------------|
| 7 | **Verificar horário de funcionamento** | Marca lojas como "fechadas" fora do horário | `horarios_funcionamento`, `lojas` | 1-5 min |
| 8 | **Agregação diária de vendas** | Materializa views em tabelas de métricas (ticket médio, top produtos) | `pedidos`, `itens_pedido`, `produtos`, `lojas` | Diário |
| 9 | **Recalcular nota média da loja** | Quando nova avaliação, recalcula `nota_media` | `avaliacoes_loja`, `lojas` | Event-driven (pode ser sync) |

---

## 📨 RabbitMQ Jobs (Event-Triggered)

Executados assincronamente quando um evento ocorre. Sugerido: crate `lapin` (AMQP) ou `tokio::spawn` como fase inicial sem broker.

### 🔴 Críticos

| # | Job | Trigger | O que faz | Por quê |
|---|-----|---------|-----------|---------|
| 1 | **Notificar loja de novo pedido** | `PedidoCreated` | Push/email/SMS para funcionários da loja | Loja precisa saber imediatamente |
| 2 | **Rastrear uso de cupom** | `PedidoCreated` (com `codigo_cupom`) | Incrementa `uso_atual` + cria registro em `uso_cupons` | Sem isso, limite de uso não é enforceado |

### 🟡 Importantes

| # | Job | Trigger | O que faz |
|---|-----|---------|-----------|
| 3 | **Push notification de status** | `PedidoStatusChanged` | Notifica cliente quando pedido muda de estado |
| 4 | **Email de comprovante** | `PedidoConfirmed` | Envia recibo detalhado por email |
| 5 | **Baixa de estoque** | `PedidoCreated` ou `PedidoConfirmado` | Decrementa `quantidade` em `ingredientes` |
| 6 | **Recalcular rating da loja** | `AvaliacaoCreated` | Atualiza nota média da loja |

### 🟢 Nice-to-have

| # | Job | Trigger | O que faz |
|---|-----|---------|-----------|
| 7 | **Lembrete de avaliação** | `PedidoEntregue` + delay 1h | Notifica cliente para avaliar loja/produtos |
| 8 | **Alerta de estoque baixo** | `IngredienteQuantidadeBaixa` | Notifica gerente da loja |
| 9 | **Relatório semanal** | `SundayNight` | Envia resumo de vendas por email |

---

## 🏗️ Arquitetura Recomendada

### Fase 1 — Scheduler In-Process (sem infra externa)

```rust
// main.rs — spawn junto com o servidor
use tokio_cron_scheduler::{JobScheduler, Job};

async fn main() {
    // ... setup do servidor axum ...

    let sched = JobScheduler::new().await.unwrap();

    // Auto-expirar cupons a cada 10 min
    sched.add(Job::new_async("*/10 * * * *", |_, _| Box::pin(async {
        expirar_cupons_vencidos(&pool).await;
    })).unwrap()).await;

    // Auto-expirar promoções a cada 10 min (pode rodar junto)
    sched.add(Job::new_async("*/10 * * * *", |_, _| Box::pin(async {
        expirar_promocoes_vencidas(&pool).await;
    })).unwrap()).await;

    // Soft-delete diário à meia-noite
    sched.add(Job::new_async("0 0 * * *", |_, _| Box::pin(async {
        processar_soft_deletes(&pool).await;
    })).unwrap()).await;

    // Auto-cancelar pedidos órfãos a cada 5 min
    sched.add(Job::new_async("*/5 * * * *", |_, _| Box::pin(async {
        cancelar_pedidos_orfaos(&pool).await;
    })).unwrap()).await;

    sched.start().await;

    // ... iniciar servidor ...
}
```

**Crate sugerido:** `tokio-cron-scheduler` — zero dependência externa, roda dentro do Tokio runtime.

### Fase 2 — Jobs Assíncronos In-Process (sem broker)

```rust
// Handler de criar pedido
async fn criar_pedido(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePedidoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let pedido = state.pedido_usecase.criar(req).await?;

    // Fire-and-forget: não bloqueia resposta HTTP
    tokio::spawn(notificar_loja(pedido.uuid, state.clone()));
    tokio::spawn(rastrear_cupom(req.codigo_cupom, pedido.uuid, state.clone()));

    Ok((StatusCode::CREATED, Json(json!({ "uuid": pedido.uuid }))))
}
```

**Limitação:** Se o processo restartar, jobs pendentes se perdem. Ok para MVP.

### Fase 3 — RabbitMQ (escala produção)

```yaml
# docker-compose.yml
services:
  rabbitmq:
    image: rabbitmq:4-management
    ports:
      - "5672:5672"
      - "15672:15672"  # management UI
    environment:
      RABBITMQ_DEFAULT_USER: chickie
      RABBITMQ_DEFAULT_PASS: ${RABBITMQ_PASS}

  worker:
    build: .
    command: worker  # binário separado ou mesmo binário com subcommand
    environment:
      RABBITMQ_URL: amqp://chickie:${RABBITMQ_PASS}@rabbitmq:5672
      DATABASE_URL: ${DATABASE_URL}
    depends_on:
      - rabbitmq
      - postgres
```

**Crate sugerido:** `lapin` (cliente AMQP para Rust).

### Quando migrar de `tokio::spawn` para RMQ?

| Sinal | Hora de migrar |
|-------|----------------|
| Jobs falhando sem retry | Precisa de fila com redelivery |
| Perda de mensagens no restart | Precisa de persistência |
| Múltiplos workers | Precisa de load balancing |
| Jobs demorando >30s | Timeout de HTTP request |
| Escala horizontal (múltiplos pods) | Precisa de broker centralizado |

---

## 📊 Prioridade de Implementação

| Fase | Jobs | Tipo | Esforço | Impacto |
|------|------|------|---------|---------|
| **1 — Pré-produção** | Auto-expirar cupons + promoções | Scheduler | Baixo | 🔴 Crítico |
| **2 — Produção** | Notificar loja de novo pedido + Rastrear uso de cupom | RMQ/spawn | Médio | 🔴 Crítico |
| **3 — Compliance** | Soft-delete users/lojas + Auto-cancelar pedidos | Scheduler | Médio | 🟡 Importante |
| **4 — UX** | Push notifications + Email + Auto-atribuir entregador | RMQ/spawn | Alto | 🟡 Importante |
| **5 — Analytics** | Agregação diária + relatórios + lembretes de avaliação | Scheduler + RMQ | Médio | 🟢 Nice-to-have |

---

## 🔗 Dependências entre Jobs

```
PedidoCreated ─┬─→ Notificar loja (RMQ)
               ├─→ Rastrear cupom (RMQ/Sync)
               ├─→ Baixa de estoque (RMQ)
               └─→ Email comprovante (RMQ)

PedidoStatusChanged ──→ Push notification (RMQ)

PedidoEntregue ──→ Lembrete de avaliação (RMQ, delayed 1h)

AvaliacaoCreated ──→ Recalcular rating (RMQ/Sync)

Scheduler 5min ──→ Auto-cancelar pedidos órfãos
Scheduler 10min ──→ Expirar cupons/promoções
Scheduler diário ──→ Soft-delete + Agregação de métricas
```

---

## 📦 Crates Sugeridos

| Crate | Uso |
|-------|-----|
| `tokio-cron-scheduler` | Scheduler in-process (Fase 1) |
| `lapin` | Cliente RabbitMQ AMQP (Fase 3) |
| `serde` | Serialização de mensagens (Fase 2+) |
| `tokio` (built-in) | `tokio::spawn` para fire-and-forget (Fase 2) |

---

## 📝 Notas de Implementação

### Padrão sugerido para Scheduler Jobs

```rust
// src/scheduler/mod.rs
pub async fn iniciar_schedulers(pool: PgPool) -> Result<(), String> {
    let sched = JobScheduler::new().await.map_err(|e| e.to_string())?;

    // Registrar todos os jobs
    registrar_expiracao_cupons(&sched, pool.clone()).await?;
    registrar_expiracao_promocoes(&sched, pool.clone()).await?;
    registrar_soft_delete(&sched, pool.clone()).await?;
    registrar_cancelamento_orfaos(&sched, pool.clone()).await?;

    sched.start().await.map_err(|e| e.to_string())?;
    Ok(())
}
```

### Padrão sugerido para RMQ Jobs

```rust
// src/workers/mod.rs
pub enum WorkerMessage {
    PedidoCriado { pedido_uuid: Uuid, loja_uuid: Uuid },
    PedidoStatusChanged { pedido_uuid: Uuid, novo_status: String },
    AvaliacaoCriada { loja_uuid: Uuid },
}

pub async fn processar_mensagem(msg: WorkerMessage, pool: PgPool) -> Result<(), String> {
    match msg {
        WorkerMessage::PedidoCriado { pedido_uuid, loja_uuid } => {
            notificar_loja(pedido_uuid, loja_uuid, &pool).await?;
            rastrear_cupom_se_existir(pedido_uuid, &pool).await?;
        }
        // ...
    }
    Ok(())
}
```

---

## 🗓️ Histórico

| Data | Mudança |
|------|---------|
| 2026-04-05 | Documento criado com planejamento completo de scheduler e jobs assíncronos |
