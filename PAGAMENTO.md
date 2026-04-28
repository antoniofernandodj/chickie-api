# Fluxo de Pagamento — Chickie × Asaas PIX

Documento de referência completo sobre a integração de pagamentos com a API Asaas.
Cobre arquitetura, banco de dados, variáveis de ambiente, endpoints e todos os arquivos Rust relevantes.

---

## Visão Geral

```
Frontend
  │
  ├─ POST /api/pedidos/criar            → cria pedido (pago = false por default)
  │
  └─ POST /api/pagamentos/{pedido_uuid} → cria cobrança PIX no Asaas
         │                                 retorna QR Code + copia-e-cola
         │
         └─ [usuário paga via app bancário]
                │
                └─ Asaas dispara webhook
                       │
                       └─ POST /api/pagamentos/webhook
                                │
                                └─ pedido.pago = true
```

O campo `pago: bool` no pedido é a flag que a loja lê para saber se deve cobrar na entrega (`false`) ou se já foi pago digitalmente (`true`).

---

## Variáveis de Ambiente

| Variável | Obrigatória | Padrão | Descrição |
|---|---|---|---|
| `ASAAS_API_KEY` | **Sim** em produção | `""` | Chave de API do Asaas (começa com `$aact_...`) |
| `ASAAS_BASE_URL` | Não | `https://api-sandbox.asaas.com/v3` | URL base — troque para `https://api.asaas.com/v3` em produção |

---

## Banco de Dados — Mudanças

### Tabela `usuarios`

```sql
-- migration 0001_criar_tabelas.sql (atualizada)
cpf VARCHAR(11) NOT NULL DEFAULT '',     -- CPF sem pontuação, 11 dígitos
asaas_customer_id VARCHAR,               -- ID do cliente no Asaas (cacheado após 1ª cobrança)
```

**`asaas_customer_id`** é salvo na primeira cobrança para evitar chamadas redundantes à API do Asaas em pagamentos futuros do mesmo usuário.

### Tabela `pedidos`

```sql
-- migration 0006_consolidar_pedidos_jsonb.sql (atualizada)
pago BOOLEAN NOT NULL DEFAULT FALSE,     -- false = cobrar na entrega | true = pago digitalmente
```

---

## Endpoints

### `POST /api/pagamentos/{pedido_uuid}`

Cria a cobrança PIX no Asaas e retorna o QR Code.

**Auth:** Opcional via `Authorization: Bearer <token>`.

**Usuário autenticado** — body pode ser vazio:
```json
{}
```

**Usuário anônimo** — campo `pagador` obrigatório:
```json
{
  "pagador": {
    "nome": "João Silva",
    "cpf": "123.456.789-01"
  }
}
```
O CPF é normalizado automaticamente (apenas dígitos).

**Response 200:**
```json
{
  "payment_id": "pay_123abc",
  "qr_code_image": "<base64 do PNG do QR Code>",
  "pix_copia_cola": "00020101021226870014br.gov.bcb...",
  "vencimento": "2026-04-28"
}
```

**Erros:**
- `400` — Pedido já foi pago
- `400` — Usuário anônimo sem `pagador.nome` ou `pagador.cpf`
- `400` — Falha na API do Asaas (CPF inválido, API key errada, etc.)

---

### `POST /api/pagamentos/webhook`

Endpoint público chamado pelo Asaas quando o pagamento é confirmado.
Marca `pedidos.pago = true` via `externalReference` (que é o `pedido_uuid`).

**Body (enviado pelo Asaas):**
```json
{
  "event": "PAYMENT_CONFIRMED",
  "payment": {
    "id": "pay_123abc",
    "externalReference": "uuid-do-pedido",
    "status": "CONFIRMED"
  }
}
```

Eventos tratados: `PAYMENT_CONFIRMED`, `PAYMENT_RECEIVED`.
Todos os outros eventos retornam `200 OK` sem efeito.

**Configuração no painel Asaas:**
No painel Asaas → Configurações → Notificações → adicionar a URL:
`https://seu-dominio.com/api/pagamentos/webhook`

---

## Fluxo Detalhado por Caso

### Usuário Autenticado com CPF Cadastrado

```
1. Frontend: POST /api/pagamentos/{pedido_uuid}  (com token JWT)
2. Handler extrai usuario do Extension<Usuario>
3. PagamentoUsecase.criar_pagamento_pix(pedido_uuid, Some(&usuario), None)
4. Verifica pedido.pago == false
5. usuario.asaas_customer_id já está preenchido?
   - SIM → usa diretamente (sem chamar Asaas /customers)
   - NÃO → chama AsaasService.cadastrar_ou_buscar_usuario_no_asaas(nome, cpf)
              salva asaas_customer_id no banco
6. AsaasService.criar_cobranca_pix(customer_id, pedido.total, pedido_uuid)
   - POST /v3/payments → obtém payment_id
   - GET /v3/payments/{id}/pixQrCode → obtém QR Code
7. Retorna PagamentoOutput ao frontend
```

### Usuário Anônimo

```
1. Frontend: POST /api/pagamentos/{pedido_uuid}
   body: { "pagador": { "nome": "...", "cpf": "..." } }
2. Handler valida presença de pagador.nome e pagador.cpf
3. PagamentoUsecase.criar_pagamento_pix(pedido_uuid, None, Some(pagador))
4. cpf é limpo (apenas dígitos)
5. Chama AsaasService.cadastrar_ou_buscar_usuario_no_asaas(nome, cpf)
   (não salva em banco pois não há usuario_uuid)
6. Cria cobrança PIX normalmente
```

### Confirmação via Webhook

```
1. Asaas → POST /api/pagamentos/webhook
2. Handler deserializa evento
3. Se event == PAYMENT_CONFIRMED ou PAYMENT_RECEIVED:
   a. Extrai externalReference (= pedido_uuid string)
   b. Faz Uuid::parse_str
   c. PagamentoUsecase.confirmar_pagamento(pedido_uuid)
      → UPDATE pedidos SET pago = TRUE WHERE uuid = $1
4. Retorna 200 OK (sempre, para não gerar retentativas desnecessárias)
```

---

## Código Rust

### `crates/core/src/services/asaas_service.rs`

Responsável por toda comunicação HTTP com a API do Asaas.

```rust
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc};

#[derive(Clone)]
pub struct AsaasService {
    client: Client,
    api_key: String,
    base_url: String,
}

// ─── Payloads de saída ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct CriarClientePayload {
    name: String,
    #[serde(rename = "cpfCnpj")]
    cpf_cnpj: String,
    #[serde(rename = "externalReference", skip_serializing_if = "Option::is_none")]
    external_reference: Option<String>,
}

#[derive(Serialize)]
struct CriarCobrancaPayload {
    customer: String,
    #[serde(rename = "billingType")]
    billing_type: String,
    value: f64,
    #[serde(rename = "dueDate")]
    due_date: String,
    #[serde(rename = "externalReference")]
    external_reference: String,
}

// ─── Payloads de entrada (respostas Asaas) ────────────────────────────────────

#[derive(Deserialize)]
struct AsaasCliente { id: String }

#[derive(Deserialize)]
struct AsaasListagem<T> {
    data: Vec<T>,
    #[serde(rename = "totalCount")]
    #[allow(dead_code)]
    total_count: i32,
}

#[derive(Deserialize)]
struct AsaasCobranca { id: String }

#[derive(Deserialize)]
struct AsaasPixQrCode {
    #[serde(rename = "encodedImage")]
    encoded_image: String,
    payload: String,
    #[serde(rename = "expirationDate")]
    expiration_date: Option<String>,
}

// ─── Resposta pública ─────────────────────────────────────────────────────────

pub struct PagamentoCriado {
    pub payment_id: String,
    pub qr_code_image: String,   // base64 PNG
    pub pix_copia_cola: String,  // string copia-e-cola
    pub vencimento: String,      // "YYYY-MM-DD"
}

impl AsaasService {
    pub fn new() -> Self {
        let api_key = std::env::var("ASAAS_API_KEY").unwrap_or_default();
        let base_url = std::env::var("ASAAS_BASE_URL")
            .unwrap_or_else(|_| "https://api-sandbox.asaas.com/v3".to_string());
        Self { client: Client::new(), api_key, base_url }
    }

    /// Busca customer no Asaas pelo CPF; cria um novo se não existir.
    pub async fn cadastrar_ou_buscar_usuario_no_asaas(
        &self,
        nome: &str,
        cpf: &str,         // 11 dígitos sem pontuação
    ) -> Result<String, String> {
        // 1. Buscar por CPF
        let resp = self.client
            .get(&format!("{}/customers?cpfCnpj={}", self.base_url, cpf))
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .send().await
            .map_err(|e| format!("Erro ao buscar cliente no Asaas: {}", e))?;

        if resp.status().is_success() {
            let body: AsaasListagem<AsaasCliente> = resp.json().await
                .map_err(|e| format!("Erro ao deserializar listagem Asaas: {}", e))?;
            if let Some(cliente) = body.data.into_iter().next() {
                return Ok(cliente.id);
            }
        }

        // 2. Criar novo customer
        let resp = self.client
            .post(&format!("{}/customers", self.base_url))
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&CriarClientePayload {
                name: nome.to_string(),
                cpf_cnpj: cpf.to_string(),
                external_reference: None,
            })
            .send().await
            .map_err(|e| format!("Erro ao criar cliente no Asaas: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Asaas retornou {} ao criar cliente: {}", status, body));
        }

        let cliente: AsaasCliente = resp.json().await
            .map_err(|e| format!("Erro ao deserializar cliente criado: {}", e))?;
        Ok(cliente.id)
    }

    /// Cria cobrança PIX e busca QR Code em sequência.
    pub async fn criar_cobranca_pix(
        &self,
        asaas_customer_id: &str,
        valor: Decimal,
        pedido_uuid: Uuid,   // salvo como externalReference
    ) -> Result<PagamentoCriado, String> {
        let due_date = (Utc::now() + Duration::days(1))
            .format("%Y-%m-%d").to_string();

        let valor_f64: f64 = valor.to_string().parse()
            .map_err(|_| "Erro ao converter valor para f64".to_string())?;

        let resp = self.client
            .post(&format!("{}/payments", self.base_url))
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&CriarCobrancaPayload {
                customer: asaas_customer_id.to_string(),
                billing_type: "PIX".to_string(),
                value: valor_f64,
                due_date: due_date.clone(),
                external_reference: pedido_uuid.to_string(),
            })
            .send().await
            .map_err(|e| format!("Erro ao criar cobrança PIX: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Asaas retornou {} ao criar cobrança: {}", status, body));
        }

        let cobranca: AsaasCobranca = resp.json().await
            .map_err(|e| format!("Erro ao deserializar cobrança: {}", e))?;

        // Buscar QR Code PIX
        let qr_resp = self.client
            .get(&format!("{}/payments/{}/pixQrCode", self.base_url, cobranca.id))
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .send().await
            .map_err(|e| format!("Erro ao buscar QR Code PIX: {}", e))?;

        if !qr_resp.status().is_success() {
            let status = qr_resp.status();
            let body = qr_resp.text().await.unwrap_or_default();
            return Err(format!("Asaas retornou {} ao buscar QR code: {}", status, body));
        }

        let qr: AsaasPixQrCode = qr_resp.json().await
            .map_err(|e| format!("Erro ao deserializar QR Code: {}", e))?;

        Ok(PagamentoCriado {
            payment_id: cobranca.id,
            qr_code_image: qr.encoded_image,
            pix_copia_cola: qr.payload,
            vencimento: qr.expiration_date.unwrap_or(due_date),
        })
    }
}
```

**Chamadas à API do Asaas:**

| Operação | Método | Endpoint |
|---|---|---|
| Buscar cliente por CPF | `GET` | `/v3/customers?cpfCnpj={cpf}` |
| Criar cliente | `POST` | `/v3/customers` |
| Criar cobrança PIX | `POST` | `/v3/payments` |
| Buscar QR Code | `GET` | `/v3/payments/{id}/pixQrCode` |

Todas as chamadas usam o header `access_token: $ASAAS_API_KEY`.

---

### `crates/core/src/usecases/pagamento.rs`

```rust
use std::sync::Arc;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    models::Usuario,
    ports::{PedidoRepositoryPort, UsuarioRepositoryPort},
    services::AsaasService,
};

pub struct PagadorInput {
    pub nome: String,
    pub cpf: String,
}

#[derive(Serialize)]
pub struct PagamentoOutput {
    pub payment_id: String,
    pub qr_code_image: String,
    pub pix_copia_cola: String,
    pub vencimento: String,
}

pub struct PagamentoUsecase {
    asaas: Arc<AsaasService>,
    pedido_repo: Arc<dyn PedidoRepositoryPort>,
    usuario_repo: Arc<dyn UsuarioRepositoryPort>,
}

impl PagamentoUsecase {
    pub fn new(
        asaas: Arc<AsaasService>,
        pedido_repo: Arc<dyn PedidoRepositoryPort>,
        usuario_repo: Arc<dyn UsuarioRepositoryPort>,
    ) -> Self {
        Self { asaas, pedido_repo, usuario_repo }
    }

    pub async fn criar_pagamento_pix(
        &self,
        pedido_uuid: Uuid,
        usuario: Option<&Usuario>,
        pagador: Option<PagadorInput>,
    ) -> Result<PagamentoOutput, String> {
        // 1. Verificar pedido
        let pedido = self.pedido_repo
            .buscar_por_uuid(pedido_uuid).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Pedido {} não encontrado", pedido_uuid))?;

        if pedido.pago {
            return Err("Pedido já foi pago".to_string());
        }

        // 2. Resolver nome, cpf e cache do asaas_customer_id
        let (nome, cpf, asaas_id_cache, usuario_uuid) = match usuario {
            Some(u) => (u.nome.clone(), u.cpf.clone(), u.asaas_customer_id.clone(), Some(u.uuid)),
            None => {
                let p = pagador.ok_or(
                    "Usuário não autenticado — forneça nome e CPF no campo 'pagador'".to_string()
                )?;
                let cpf_limpo: String = p.cpf.chars().filter(|c| c.is_ascii_digit()).collect();
                (p.nome, cpf_limpo, None, None)
            }
        };

        // 3. Obter ou criar customer no Asaas
        let asaas_customer_id = match asaas_id_cache {
            Some(id) => id,
            None => {
                let id = self.asaas
                    .cadastrar_ou_buscar_usuario_no_asaas(&nome, &cpf).await?;
                if let Some(uid) = usuario_uuid {
                    if let Err(e) = self.usuario_repo.salvar_asaas_customer_id(uid, &id).await {
                        tracing::warn!("Falha ao salvar asaas_customer_id usuario={}: {}", uid, e);
                    }
                }
                id
            }
        };

        // 4. Criar cobrança PIX
        let pagamento = self.asaas
            .criar_cobranca_pix(&asaas_customer_id, pedido.total, pedido_uuid).await?;

        Ok(PagamentoOutput {
            payment_id: pagamento.payment_id,
            qr_code_image: pagamento.qr_code_image,
            pix_copia_cola: pagamento.pix_copia_cola,
            vencimento: pagamento.vencimento,
        })
    }

    /// Chamado pelo handler do webhook para marcar o pedido como pago.
    pub async fn confirmar_pagamento(&self, pedido_uuid: Uuid) -> Result<(), String> {
        self.pedido_repo.marcar_como_pago(pedido_uuid).await.map_err(|e| e.to_string())
    }
}
```

---

### `crates/api/src/handlers/pagamento/criar_pagamento.rs`

```rust
use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    ports::{PedidoRepositoryPort, UsuarioRepositoryPort},
    usecases::{PagamentoUsecase, PagadorInput},
};
use crate::handlers::{AppState, dto::AppError};

#[derive(Deserialize)]
pub struct PagadorPayload {
    pub nome: Option<String>,
    pub cpf: Option<String>,
}

#[derive(Deserialize)]
pub struct CriarPagamentoRequest {
    pub pagador: Option<PagadorPayload>,
}

pub async fn criar_pagamento(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    usuario_ext: Option<Extension<Usuario>>,  // Some se token válido, None se anônimo
    Json(body): Json<CriarPagamentoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let usuario = usuario_ext.map(|Extension(u)| u);

    let pagador = if usuario.is_none() {
        let p = body.pagador.ok_or_else(|| {
            AppError::BadRequest("Usuário não autenticado — forneça 'pagador' com nome e CPF".to_string())
        })?;
        let nome = p.nome.ok_or_else(|| AppError::BadRequest("Campo 'pagador.nome' é obrigatório".to_string()))?;
        let cpf  = p.cpf .ok_or_else(|| AppError::BadRequest("Campo 'pagador.cpf' é obrigatório".to_string()))?;
        Some(PagadorInput { nome, cpf })
    } else {
        None
    };

    let usecase = PagamentoUsecase::new(
        Arc::clone(&state.asaas_service),
        Arc::clone(&state.pedido_repo) as Arc<dyn PedidoRepositoryPort>,
        Arc::clone(&state.usuario_repo) as Arc<dyn UsuarioRepositoryPort>,
    );

    let output = usecase
        .criar_pagamento_pix(pedido_uuid, usuario.as_ref(), pagador)
        .await
        .map_err(AppError::BadRequest)?;

    Ok(Json(output))
}
```

---

### `crates/api/src/handlers/pagamento/webhook.rs`

```rust
use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    ports::{PedidoRepositoryPort, UsuarioRepositoryPort},
    usecases::PagamentoUsecase,
};
use crate::handlers::AppState;

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub payment: Option<WebhookPayment>,
}

#[derive(Deserialize)]
pub struct WebhookPayment {
    #[serde(rename = "externalReference")]
    pub external_reference: Option<String>,
}

pub async fn webhook_asaas(
    State(state): State<Arc<AppState>>,
    Json(body): Json<WebhookPayload>,
) -> impl IntoResponse {
    let confirmar = matches!(
        body.event.as_str(),
        "PAYMENT_CONFIRMED" | "PAYMENT_RECEIVED"
    );

    if !confirmar {
        return StatusCode::OK;
    }

    let pedido_uuid_str = body
        .payment
        .and_then(|p| p.external_reference)
        .unwrap_or_default();

    let pedido_uuid = match Uuid::parse_str(&pedido_uuid_str) {
        Ok(id) => id,
        Err(_) => {
            tracing::warn!("webhook_asaas: externalReference inválido '{}'", pedido_uuid_str);
            return StatusCode::OK;
        }
    };

    let usecase = PagamentoUsecase::new(
        Arc::clone(&state.asaas_service),
        Arc::clone(&state.pedido_repo) as Arc<dyn PedidoRepositoryPort>,
        Arc::clone(&state.usuario_repo) as Arc<dyn UsuarioRepositoryPort>,
    );

    if let Err(e) = usecase.confirmar_pagamento(pedido_uuid).await {
        tracing::error!("webhook_asaas: falha ao marcar pedido={} como pago: {}", pedido_uuid, e);
    } else {
        tracing::info!("webhook_asaas: pedido={} marcado como pago", pedido_uuid);
    }

    StatusCode::OK
}
```

---

### `crates/api/src/handlers/routers/pagamento.rs`

```rust
use axum::{Router, routing::post, middleware::from_fn_with_state};
use std::sync::Arc;
use crate::handlers::{AppState, optional_auth_middleware, criar_pagamento, webhook_asaas};

pub fn pagamento_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    // Webhook público (Asaas chama sem auth)
    let public = Router::new()
        .route("/webhook", post(webhook_asaas));

    // Criar pagamento: auth opcional
    let pagamento = Router::new()
        .route("/{pedido_uuid}", post(criar_pagamento))
        .layer(from_fn_with_state(s.clone(), optional_auth_middleware));

    Router::new()
        .merge(public)
        .merge(pagamento)
}
```

Montado em `api_routes` como `.nest("/pagamentos", pagamento_routes(s))`.

---

### Ports — Métodos Adicionados

**`crates/core/src/ports/pedido_port.rs`**
```rust
async fn marcar_como_pago(&self, uuid: Uuid) -> DomainResult<()>;
```

**`crates/core/src/ports/usuario_port.rs`**
```rust
async fn salvar_asaas_customer_id(&self, uuid: Uuid, customer_id: &str) -> DomainResult<()>;
```

---

### Repositories — Implementações Adicionadas

**`crates/core/src/repositories/pedido_repository.rs`**
```rust
async fn marcar_como_pago(&self, uuid: Uuid) -> DomainResult<()> {
    sqlx::query("UPDATE pedidos SET pago = TRUE, atualizado_em = NOW() WHERE uuid = $1")
        .bind(uuid)
        .execute(self.pool())
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(())
}
```

**`crates/core/src/repositories/usuario_repository.rs`**
```rust
pub async fn salvar_asaas_customer_id(&self, uuid: Uuid, customer_id: &str) -> Result<(), String> {
    sqlx::query(
        "UPDATE usuarios SET asaas_customer_id = $2, atualizado_em = NOW() WHERE uuid = $1 AND deletado = false"
    )
    .bind(uuid)
    .bind(customer_id)
    .execute(self.pool())
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}
```

---

### Models — Campos Adicionados

**`Usuario`** (`crates/core/src/models/usuario.rs`):
```rust
pub cpf: String,
#[serde(default)]
pub asaas_customer_id: Option<String>,
```
`Usuario::new()` agora recebe `cpf: String` como 6º parâmetro e inicializa `asaas_customer_id: None`.

**`Pedido`** (`crates/core/src/models/pedido.rs`):
```rust
pub pago: bool,   // false = a pagar na entrega | true = pago digitalmente
```
Inicializado como `pago: false` em `Pedido::new()`.

---

### AppState — Campo Adicionado

**`crates/api/src/handlers/state.rs`**:
```rust
pub struct AppState {
    pub asaas_service: Arc<AsaasService>,
    // ... demais campos
}

// Em AppState::new():
let asaas_service = Arc::new(AsaasService::new());
```

---

## Signup — Campo CPF

O `cpf` passou a ser obrigatório no cadastro de usuário.

**Request `POST /api/auth/signup`:**
```json
{
  "nome": "João Silva",
  "username": "joao",
  "email": "joao@email.com",
  "senha": "senha123",
  "celular": "(11) 99999-9999",
  "cpf": "123.456.789-01",
  "auth_method": "email"
}
```

O CPF é normalizado para apenas dígitos antes de ser salvo (`12345678901`).

Usuários criados internamente pelo sistema (funcionários, entregadores, clientes via `LojaService`) recebem CPF vazio (`""`). O CPF deve ser atualizado manualmente nesses casos antes de usar o fluxo de pagamento.

---

## Roteamento

```
POST  /api/pagamentos/{pedido_uuid}   → criar_pagamento (auth opcional)
POST  /api/pagamentos/webhook         → webhook_asaas  (público)
```

---

## Pontos de Atenção

| Situação | Comportamento |
|---|---|
| Pedido já pago (`pago = true`) | `400 Bad Request` — "Pedido já foi pago" |
| CPF inválido ou recusado pelo Asaas | `400 Bad Request` com mensagem do Asaas |
| `ASAAS_API_KEY` vazia | Asaas retorna 401; handler retorna `400` com mensagem de erro |
| Webhook com `externalReference` inválido | Log de warning, retorna `200 OK` sem efeito |
| Usuário autenticado sem CPF cadastrado (`""`) | Asaas rejeita — orientar usuário a atualizar o CPF |
| Funcionários/entregadores criados pelo admin | CPF vazio — não conseguem usar o fluxo PIX até CPF ser atualizado |

---

## Sandbox vs Produção

| Ambiente | `ASAAS_BASE_URL` | CPF necessário |
|---|---|---|
| Sandbox | `https://api-sandbox.asaas.com/v3` | Não (aceita CPF vazio/inválido) |
| Produção | `https://api.asaas.com/v3` | **Sim** (obrigatório e válido) |
