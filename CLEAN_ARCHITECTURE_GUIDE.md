# Guia de Arquitetura — Chickie API

> Tutorial completo para criar novas entidades, repositórios, services, ports e endpoints seguindo Clean Architecture.

---

## Visão Geral da Arquitetura

```
┌─────────────────────────────────────────────────────────────────┐
│                        API Layer (Axum)                         │
│  Handlers → extraem request → chamam usecase → retornam response │
├─────────────────────────────────────────────────────────────────┤
│                     Application Layer                           │
│  Usecases (orquestram) → Services (regras de negócio)            │
├─────────────────────────────────────────────────────────────────┤
│                        Port Layer (Traits)                       │
│  RepositoryPort traits — contratos sem infraestrutura            │
├─────────────────────────────────────────────────────────────────┤
│                       Infrastructure Layer                       │
│  Repositories (sqlx) — implementam os ports                     │
├─────────────────────────────────────────────────────────────────┤
│                        Domain Layer                             │
│  DomainError, enums puros, models (com FromRow/serde)            │
└─────────────────────────────────────────────────────────────────┘
```

### Regra de Ouro

**Toda requisição segue esta pilha, sem exceções:**

```
HTTP Request → Handler → Usecase → Service → Port (trait) → Repository → Database
```

Handlers **nunca** contêm lógica de negócio, queries SQL, ou chamadas diretas a repositórios.

---

## Tutorial Completo: Criando a Entidade `Pagamento`

Vamos criar um CRUD completo de pagamentos para um pedido, seguindo cada camada.

### Passo 0: Entender o que precisamos

| Camada | Arquivo | Responsabilidade |
|--------|---------|-----------------|
| **Model** | `models/pagamento.rs` | Struct com `FromRow`, `Serialize`, `Deserialize` |
| **Migration** | `migrations/0006_criar_pagamentos.sql` | SQL para criar tabela |
| **Repository** | `repositories/pagamento_repository.rs` | Queries SQL, implementa o port |
| **Port** | `ports/pagamento_port.rs` | Trait sem sqlx |
| **Service** | `services/pagamento_service.rs` | Regras de negócio |
| **Usecase** | `usecases/pagamento.rs` | Orquestrador para API |
| **API Handler** | `api_handlers/pagamento/criar.rs` | Extrai request, chama usecase |
| **API Handler** | `api_handlers/pagamento/listar.rs` | Extrai request, chama usecase |
| **Router** | `api_handlers/routers/mod.rs` | Registra rotas |
| **AppState** | `api_handlers/state.rs` | Injeta dependências |

---

### Passo 1: Criar o Model

**Arquivo:** `crates/core/src/models/pagamento.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Pagamento {
    pub uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub forma_pagamento: String,
    pub valor: Decimal,
    pub status: String, // "pendente", "confirmado", "falhou", "estornado"
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl Pagamento {
    pub fn new(
        pedido_uuid: Uuid,
        forma_pagamento: String,
        valor: Decimal,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            pedido_uuid,
            forma_pagamento,
            valor,
            status: "pendente".to_string(),
            criado_em: Utc::now(),
        }
    }
}

impl Model for Pagamento {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
```

**Registrar no `models/mod.rs`:**

```rust
mod pagamento;
pub use pagamento::Pagamento;
```

---

### Passo 2: Criar a Migration

**Arquivo:** `migrations/0006_criar_pagamentos.sql`

```sql
CREATE TABLE IF NOT EXISTS pagamentos (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pedido_uuid UUID NOT NULL REFERENCES pedidos(uuid) ON DELETE CASCADE,
    forma_pagamento TEXT NOT NULL,
    valor NUMERIC(10, 2) NOT NULL,
    status TEXT NOT NULL DEFAULT 'pendente',
    criado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pagamentos_pedido_uuid ON pagamentos(pedido_uuid);
```

---

### Passo 3: Criar o Repository

**Arquivo:** `crates/core/src/repositories/pagamento_repository.rs`

```rust
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::models::{Pagamento, Model};
use crate::repositories::Repository;
use crate::domain::errors::{DomainError, DomainResult};
use crate::ports::PagamentoRepositoryPort;

pub struct PagamentoRepository {
    pool: Arc<PgPool>,
}

impl PagamentoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Busca todos os pagamentos de um pedido
    pub async fn buscar_por_pedido(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<Vec<Pagamento>, String> {
        sqlx::query_as::<_, Pagamento>(
            "SELECT uuid, pedido_uuid, forma_pagamento, valor, status, criado_em 
             FROM pagamentos WHERE pedido_uuid = $1 ORDER BY criado_em DESC"
        )
        .bind(pedido_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

// Implementa o Repository trait genérico para métodos padrão
#[async_trait]
impl Repository<Pagamento> for PagamentoRepository {
    fn pool(&self) -> &PgPool { &self.pool }
    fn table_name(&self) -> &'static str { "pagamentos" }
    fn entity_name(&self) -> &'static str { "pagamento" }

    async fn criar(&self, item: &Pagamento) -> Result<Uuid, String> {
        sqlx::query!(
            r#"INSERT INTO pagamentos (uuid, pedido_uuid, forma_pagamento, valor, status)
               VALUES ($1, $2, $3, $4, $5)"#,
            item.uuid,
            item.pedido_uuid,
            item.forma_pagamento,
            item.valor,
            item.status,
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(item.uuid)
    }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Pagamento>, String> {
        sqlx::query_as::<_, Pagamento>(
            "SELECT uuid, pedido_uuid, forma_pagamento, valor, status, criado_em FROM pagamentos WHERE uuid = $1"
        )
        .bind(uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn listar_todos(&self) -> Result<Vec<Pagamento>, String> {
        sqlx::query_as::<_, Pagamento>(
            "SELECT uuid, pedido_uuid, forma_pagamento, valor, status, criado_em FROM pagamentos ORDER BY criado_em DESC"
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, item: Pagamento) -> Result<(), String> {
        sqlx::query!(
            r#"UPDATE pagamentos SET forma_pagamento = $2, valor = $3, status = $4
               WHERE uuid = $1"#,
            item.uuid,
            item.forma_pagamento,
            item.valor,
            item.status,
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query!("DELETE FROM pagamentos WHERE uuid = $1", uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// Implementa o Port trait — delega para o Repository trait ou métodos próprios
#[async_trait]
impl PagamentoRepositoryPort for PagamentoRepository {
    async fn criar(&self, entity: &Pagamento) -> DomainResult<Uuid> {
        <Self as Repository<Pagamento>>::criar(self, entity)
            .await
            .map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Pagamento>> {
        <Self as Repository<Pagamento>>::buscar_por_uuid(self, uuid)
            .await
            .map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> DomainResult<Vec<Pagamento>> {
        self.buscar_por_pedido(pedido_uuid)
            .await
            .map_err(|e| DomainError::Internal(e))
    }

    async fn listar_todos(&self) -> DomainResult<Vec<Pagamento>> {
        <Self as Repository<Pagamento>>::listar_todos(self)
            .await
            .map_err(|e| DomainError::Internal(e))
    }

    async fn atualizar(&self, entity: Pagamento) -> DomainResult<()> {
        <Self as Repository<Pagamento>>::atualizar(self, entity)
            .await
            .map_err(|e| DomainError::Internal(e))
    }

    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<Pagamento>>::deletar(self, uuid)
            .await
            .map_err(|e| DomainError::Internal(e))
    }
}
```

**Registrar em `repositories/mod.rs`:**

```rust
mod pagamento_repository;
pub use pagamento_repository::PagamentoRepository;
```

---

### Passo 4: Criar o Port (Trait)

**Arquivo:** `crates/core/src/ports/pagamento_port.rs`

```rust
use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Pagamento;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait PagamentoRepositoryPort: Send + Sync {
    async fn criar(&self, entity: &Pagamento) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Pagamento>>;
    async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> DomainResult<Vec<Pagamento>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Pagamento>>;
    async fn atualizar(&self, entity: Pagamento) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
```

**Registrar em `ports/mod.rs`:**

```rust
pub mod pagamento_port;
pub use pagamento_port::PagamentoRepositoryPort;
```

---

### Passo 5: Criar o Service

**Arquivo:** `crates/core/src/services/pagamento_service.rs`

```rust
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::Pagamento;
use crate::ports::PagamentoRepositoryPort;

pub struct PagamentoService {
    repo: Arc<dyn PagamentoRepositoryPort>,
}

impl PagamentoService {
    pub fn new(repo: Arc<dyn PagamentoRepositoryPort>) -> Self {
        Self { repo }
    }

    /// Registra um pagamento para um pedido
    pub async fn registrar(
        &self,
        pedido_uuid: Uuid,
        forma_pagamento: String,
        valor: Decimal,
    ) -> Result<Pagamento, String> {
        let pagamento = Pagamento::new(pedido_uuid, forma_pagamento, valor);
        self.repo.criar(&pagamento).await?;
        Ok(pagamento)
    }

    /// Lista todos os pagamentos de um pedido
    pub async fn listar_por_pedido(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<Vec<Pagamento>, String> {
        self.repo.buscar_por_pedido(pedido_uuid).await
    }

    /// Confirma um pagamento pendente
    pub async fn confirmar(&self, uuid: Uuid) -> Result<(), String> {
        let mut pagamento = self.repo.buscar_por_uuid(uuid).await?
            .ok_or("Pagamento não encontrado")?;

        if pagamento.status != "pendente" {
            return Err(format!(
                "Pagamento não pode ser confirmado. Status atual: {}",
                pagamento.status
            ));
        }

        pagamento.status = "confirmado".to_string();
        self.repo.atualizar(pagamento).await
    }
}
```

**Registrar em `services/mod.rs`:**

```rust
mod pagamento_service;
pub use pagamento_service::PagamentoService;
```

---

### Passo 6: Criar o Usecase

**Arquivo:** `crates/core/src/usecases/pagamento.rs`

```rust
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Usuario, Pagamento};
use crate::services::PagamentoService;

pub struct PagamentoUsecase {
    pagamento_service: Arc<PagamentoService>,
    pub usuario: Usuario,
    pub pedido_uuid: Uuid,
}

impl PagamentoUsecase {
    pub fn new(
        pagamento_service: Arc<PagamentoService>,
        usuario: Usuario,
        pedido_uuid: Uuid,
    ) -> Self {
        Self {
            pagamento_service,
            usuario,
            pedido_uuid,
        }
    }

    pub async fn registrar_pagamento(
        &self,
        forma_pagamento: String,
        valor: Decimal,
    ) -> Result<Pagamento, String> {
        self.pagamento_service
            .registrar(self.pedido_uuid, forma_pagamento, valor)
            .await
    }

    pub async fn listar_pagamentos_pedido(&self) -> Result<Vec<Pagamento>, String> {
        self.pagamento_service
            .listar_por_pedido(self.pedido_uuid)
            .await
    }

    pub async fn confirmar_pagamento(&self, pagamento_uuid: Uuid) -> Result<(), String> {
        self.pagamento_service.confirmar(pagamento_uuid).await
    }
}
```

**Registrar em `usecases/mod.rs`:**

```rust
mod pagamento;
pub use pagamento::PagamentoUsecase;
```

---

### Passo 7: Criar Request DTOs

**Arquivo:** `crates/api/src/api_handlers/dto/mod.rs` (adicionar no final)

```rust
#[derive(Deserialize, ToSchema)]
pub struct CriarPagamentoRequest {
    pub forma_pagamento: String,
    pub valor: Decimal,
}
```

---

### Passo 8: Criar Handlers

**Arquivo:** `crates/api/src/api_handlers/pagamento/mod.rs`

```rust
pub mod criar;
pub mod listar;
pub mod confirmar;
```

**Arquivo:** `crates/api/src/api_handlers/pagamento/criar.rs`

```rust
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::api_handlers::{AppState, dto::CriarPagamentoRequest};
use crate::api_handlers::auth::UsuarioAuth;
use chickie_core::usecases::PagamentoUsecase;

pub async fn handler(
    Path(pedido_uuid): uuid::Uuid,
    UsuarioAuth(usuario): UsuarioAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CriarPagamentoRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let usecase = PagamentoUsecase::new(
        state.pagamento_service.clone(),
        usuario,
        pedido_uuid,
    );

    let pagamento = usecase
        .registrar_pagamento(body.forma_pagamento, body.valor)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "uuid": pagamento.uuid,
            "pedido_uuid": pagamento.pedido_uuid,
            "forma_pagamento": pagamento.forma_pagamento,
            "valor": pagamento.valor,
            "status": pagamento.status,
        })),
    ))
}
```

**Arquivo:** `crates/api/src/api_handlers/pagamento/listar.rs`

```rust
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use crate::api_handlers::{AppState, AppError};
use crate::api_handlers::auth::UsuarioAuth;
use chickie_core::usecases::PagamentoUsecase;

pub async fn handler(
    Path(pedido_uuid): uuid::Uuid,
    UsuarioAuth(usuario): UsuarioAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let usecase = PagamentoUsecase::new(
        state.pagamento_service.clone(),
        usuario,
        pedido_uuid,
    );

    let pagamentos = usecase.listar_pagamentos_pedido().await?;

    Ok(Json(
        pagamentos
            .iter()
            .map(|p| {
                serde_json::json!({
                    "uuid": p.uuid,
                    "pedido_uuid": p.pedido_uuid,
                    "forma_pagamento": p.forma_pagamento,
                    "valor": p.valor,
                    "status": p.status,
                })
            })
            .collect(),
    ))
}
```

**Arquivo:** `crates/api/src/api_handlers/pagamento/confirmar.rs`

```rust
use axum::{extract::{Path, State}, http::StatusCode};
use std::sync::Arc;
use crate::api_handlers::{AppState, AppError};
use crate::api_handlers::auth::UsuarioAuth;
use chickie_core::usecases::PagamentoUsecase;

pub async fn handler(
    Path((pedido_uuid, pagamento_uuid)): (uuid::Uuid, uuid::Uuid),
    UsuarioAuth(usuario): UsuarioAuth,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, AppError> {
    let usecase = PagamentoUsecase::new(
        state.pagamento_service.clone(),
        usuario,
        pedido_uuid,
    );

    usecase.confirmar_pagamento(pagamento_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}
```

---

### Passo 9: Registrar Rotas

**Arquivo:** `crates/api/src/api_handlers/routers/mod.rs`

Adicionar o módulo:

```rust
pub mod pagamento;
```

E registrar as rotas no `api_routes()`:

```rust
// Pagamentos
.route("/pagamentos/{pedido_uuid}", post(pagamento::criar::handler))
.route("/pagamentos/{pedido_uuid}", get(pagamento::listar::handler))
.route("/pagamentos/{pedido_uuid}/{pagamento_uuid}/confirmar", put(pagamento::confirmar::handler))
```

---

### Passo 10: Registrar no AppState

**Arquivo:** `crates/api/src/api_handlers/state.rs`

1. Adicionar ao `use chickie_core::ports`:
```rust
PagamentoRepositoryPort,
```

2. Adicionar ao `use chickie_core::repositories`:
```rust
PagamentoRepository,
```

3. Adicionar ao `use chickie_core::services`:
```rust
PagamentoService,
```

4. Adicionar campo na struct `AppState`:
```rust
pub pagamento_service: Arc<PagamentoService>,
```

5. Inicializar no `AppState::new()`:
```rust
let pagamento_repo = Arc::new(PagamentoRepository::new(pool.clone()));

let pagamento_service = Arc::new(
    PagamentoService::new(
        Arc::clone(&pagamento_repo) as Arc<dyn PagamentoRepositoryPort>
    )
);
```

6. Adicionar ao `AppState { ... }`:
```rust
pagamento_service: Arc::clone(&pagamento_service),
```

---

### Passo 11: Compilar e Testar

```bash
cargo run
```

Testar com curl:

```bash
# Criar pagamento
curl -X POST http://localhost:3000/api/pagamentos/{pedido_uuid} \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"forma_pagamento": "PIX", "valor": 65.90}'

# Listar pagamentos
curl http://localhost:3000/api/pagamentos/{pedido_uuid} \
  -H "Authorization: Bearer $TOKEN"

# Confirmar pagamento
curl -X PUT http://localhost:3000/api/pagamentos/{pedido_uuid}/{pagamento_uuid}/confirmar \
  -H "Authorization: Bearer $TOKEN"
```

---

## Checklist Rápido (Copy & Paste)

```
[ ] 1. models/{entidade}.rs — struct com FromRow, Serialize, Deserialize, impl Model
[ ] 2. models/mod.rs — mod + pub use
[ ] 3. migrations/000N_nome.sql — CREATE TABLE
[ ] 4. repositories/{entidade}_repository.rs — struct + impl Repository + impl Port
[ ] 5. repositories/mod.rs — mod + pub use
[ ] 6. ports/{entidade}_port.rs — trait com async_trait + DomainResult
[ ] 7. ports/mod.rs — mod + pub use
[ ] 8. services/{entidade}_service.rs — struct com Arc<dyn Port>, regras
[ ] 9. services/mod.rs — mod + pub use
[ ] 10. usecases/{entidade}.rs — struct com service, orquestra
[ ] 11. usecases/mod.rs — mod + pub use
[ ] 12. api_handlers/dto/mod.rs — Request DTOs
[ ] 13. api_handlers/{entidade}/mod.rs — módulo de handlers
[ ] 14. api_handlers/{entidade}/criar.rs — handler
[ ] 15. api_handlers/{entidade}/listar.rs — handler
[ ] 16. api_handlers/routers/mod.rs — registrar rotas
[ ] 17. api_handlers/state.rs — port, repo, service, field, init
[ ] 18. cargo check — compilar
```

---

## Princípios que Nunca Devem Ser Violados

1. **Handler nunca faz query SQL** — usa usecase
2. **Usecase nunca chama repository concreto** — usa service ou port
3. **Service nunca conhece o banco** — dependem de traits (ports)
4. **Port nunca importam sqlx** — são contratos puros
5. **DomainError nunca mapeia HTTP status** — isso é papel do AppError na API
6. **Nunca bypass a pilha** — `Handler → Usecase → Service → Port → Repository`

---

## Estrutura Final do Projeto

```
crates/core/src/
├── domain/
│   ├── errors/mod.rs       # DomainError, DomainResult<T>
│   └── enums/              # Enums puros (sem sqlx)
├── ports/                  # 23 traits — contratos sem infra
├── adapters/               # SQLx adapters (em construção)
├── models/                 # Entidades com FromRow + serde
├── repositories/           # Queries SQL, implementam ports
├── services/               # Regras de negócio, dependem de ports
├── usecases/               # Orquestradores para API
└── utils/                  # Utilitários

crates/api/src/
├── main.rs                 # Bootstrap
├── infrastructure/
│   └── database.rs         # Pool + migrations
└── api_handlers/
    ├── dto/mod.rs          # Request DTOs + AppError
    ├── state.rs            # AppState (injeção de dependência)
    ├── routers.rs          # Todas as rotas
    ├── auth.rs             # JWT middleware
    ├── {modulo}/           # Handlers por entidade
    └── routers/            # Arquivos de rota individuais
```
