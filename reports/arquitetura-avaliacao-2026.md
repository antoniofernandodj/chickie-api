# Avaliação de Arquitetura — Chickie API
> Data: 2026-04-21 | Revisor: Claude Sonnet 4.6 | Versão analisada: branch `main-api`

---

## Veredicto

**A arquitetura é adequada para 3-4 anos de desenvolvimento profissional assistido por IA.**

A base hexagonal com ports está correta, o workspace multi-crate está bem organizado, e a documentação sustenta o contexto necessário para que o desenvolvimento com IA funcione de forma consistente ao longo do tempo. O problema crítico não é arquitetural — é a ausência de testes automatizados, que sem resolução vai acumular dívida de confiança à medida que a codebase cresce.

---

## Sumário Executivo

| Critério | Avaliação | Nota |
|---|---|---|
| Separação de camadas | Hexagonal bem implementada, consistente | ★★★★★ |
| Contratos de domínio (ports) | 23 traits, domínio isolado de infraestrutura | ★★★★★ |
| Documentação | CLAUDE.md, API.md, guias, pendências | ★★★★★ |
| Organização do workspace | Multi-crate com responsabilidades claras | ★★★★☆ |
| Tratamento de erros | DomainError bom, mas `Result<T, String>` nos services | ★★★☆☆ |
| Segurança | Argon2, JWT, middleware de permissões | ★★★★☆ |
| Testes automatizados | Praticamente inexistentes | ★☆☆☆☆ |
| CI/CD | Não existe | ★☆☆☆☆ |
| Performance / escalabilidade | Async correto, sem paginação nem cache | ★★★☆☆ |
| Produção-readiness | Docker multi-stage, variáveis de ambiente | ★★★☆☆ |

**Média geral: 3.6 / 5**

---

## 1. Estrutura do Workspace

O projeto é um **monorepo com 5 crates** gerenciados por Cargo workspace:

```
chickie/
├── crates/
│   ├── core/        → domínio, ports, repositories, services, usecases
│   ├── api/         → handlers HTTP (Axum), rotas, estado compartilhado
│   ├── worker/      → consumidor de fila RabbitMQ (background jobs)
│   ├── scheduler/   → tarefas cron agendadas
│   └── cli/         → operações administrativas locais
├── migrations/      → 11 arquivos SQL sequenciais
├── dockerfiles/     → api.dockerfile, worker.dockerfile, scheduler.dockerfile
├── docker-compose.yml
└── reports/
```

### O que está certo

O isolamento entre `core` e `api` é a decisão mais importante do projeto. Lógica de negócio em `chickie-core` nunca depende de Axum, JSON ou HTTP. Handlers em `chickie-api` nunca contêm SQL ou regras de negócio. Esse contrato arquitetural, quando respeitado, garante que o desenvolvimento assistido por IA siga o padrão correto mecanicamente — o Claude sabe exatamente onde colocar cada coisa porque não há ambiguidade.

A separação `worker` + `scheduler` como crates independentes é boa arquitetura: tarefas assíncronas (envio de email, notificações) e tarefas cron (soft-delete programado) não bloqueiam a API e podem ser escaladas independentemente.

### Dependências centralizadas

```toml
# Cargo.toml (workspace)
[workspace.dependencies]
sqlx = { version = "0.8.6", features = [...] }
chrono = { version = "0.4.43", features = ["serde"] }
rust_decimal = { version = "1.38", features = ["serde"] }
```

Todas as versões de dependência são definidas uma vez no workspace. Isso evita divergências de versão entre crates e simplifica atualizações.

---

## 2. Arquitetura Hexagonal (Ports & Adapters)

O projeto implementa Clean Architecture em 5 camadas com dependência unidirecional:

```
┌──────────────────────────────────────────────┐
│  API Layer  (Axum Handlers + DTOs)           │
│  → extrai request, delega para usecase       │
├──────────────────────────────────────────────┤
│  Use Case Layer                              │
│  → orquestra fluxo, valida entrada           │
├──────────────────────────────────────────────┤
│  Service Layer                               │
│  → regras de negócio, cálculos, políticas    │
├──────────────────────────────────────────────┤
│  Port Layer  (Traits async)                  │
│  → contratos sem dependência de infra        │
├──────────────────────────────────────────────┤
│  Repository Layer  (sqlx + PostgreSQL)       │
│  → implementação concreta dos ports          │
├──────────────────────────────────────────────┤
│  Domain Layer  (Models + DomainError)        │
│  → entidades, value objects, enums           │
└──────────────────────────────────────────────┘
```

### 2.1 Domain Layer

`DomainError` tem variantes específicas com contexto suficiente para diagnóstico:

```rust
pub enum DomainError {
    NotFound { entity: &'static str, id: String },
    BusinessRule(String),
    Validation(String),
    Conflict { entity: &'static str, field: String },
    InvalidState { current: String, attempted: String, allowed: Vec<String> },
    Internal(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
```

Isso permite que a camada HTTP mapeie erros para status codes sem fazer pattern matching em strings. `NotFound` → 404, `BusinessRule` → 400, `Conflict` → 409, `Internal` → 500.

### 2.2 Port Layer

São **23 traits** cobrindo todas as entidades do domínio. Exemplo representativo:

```rust
#[async_trait]
pub trait PedidoRepositoryPort: Send + Sync {
    async fn criar(&self, pedido: &Pedido) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Pedido>>;
    async fn buscar_completo(&self, uuid: Uuid) -> DomainResult<Option<Pedido>>;
    async fn buscar_completos_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Pedido>>;
    async fn buscar_completos_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<Pedido>>;
    async fn atualizar_status(&self, uuid: Uuid, novo_status: &str) -> DomainResult<()>;
    async fn atribuir_entregador(&self, pedido_uuid: Uuid, entregador_uuid: Uuid) -> DomainResult<()>;
    // ...
}
```

Nenhuma referência a `sqlx`, `PgPool` ou SQL dentro dos ports. Os services dependem exclusivamente de `Arc<dyn PedidoRepositoryPort>`, o que torna mock e substituição triviais.

### 2.3 Service Layer

Services dependem de traits, não de implementações concretas:

```rust
pub struct PedidoService {
    pedido_repo: Arc<dyn PedidoRepositoryPort>,
    config_repo: Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>,
    cupom_repo: Arc<dyn CupomRepositoryPort>,
    promocao_repo: Arc<dyn PromocaoRepositoryPort>,
    endereco_entrega_repo: Arc<dyn EnderecoEntregaRepositoryPort>,
}
```

Isso é correto e é exatamente o que permite testes unitários sem banco de dados. Em qualquer ponto, é possível criar um `MockPedidoRepository` que implementa `PedidoRepositoryPort` e injetá-lo no `PedidoService` nos testes.

### 2.4 AppState (Injeção de Dependência)

A injeção de dependência é manual e centralizada em `state.rs`:

```rust
pub struct AppState {
    // Services
    pub usuario_service: Arc<UsuarioService>,
    pub loja_service: Arc<LojaService>,
    pub pedido_service: Arc<PedidoService>,
    // ... 14 services no total

    // Repos expostos diretamente para buscas simples
    pub pedido_repo: Arc<PedidoRepository>,
    pub usuario_repo: Arc<UsuarioRepository>,

    // Pool raw para operações administrativas
    pub db: Arc<PgPool>,
}

impl AppState {
    pub fn new(pool: Arc<PgPool>) -> Arc<Self> {
        // 1. Instancia repositories concretos
        let pedido_repo = Arc::new(PedidoRepository::new(pool.clone()));
        // 2. Injeta como trait objects nos services
        let pedido_service = Arc::new(PedidoService::new(
            Arc::clone(&pedido_repo) as Arc<dyn PedidoRepositoryPort>,
            // ...
        ));
        // 3. Monta AppState
        Arc::new(AppState { /* ... */ })
    }
}
```

Funciona bem. A desvantagem é que `state.rs` tem ~294 linhas de wiring manual, mas isso é aceitável — é explícito, rastreável, e fácil de entender sem framework de DI.

---

## 3. Banco de Dados e Migrações

### Migrações

11 arquivos SQL sequenciais numerados com nomes descritivos:

```
0001_criar_tabelas.sql
0002_add_promocao_escopo.sql
0003_add_criado_por_lojas.sql
0004_add_pizza_mode_categorias.sql
0005_add_entregador_uuid_pedidos.sql
0006_consolidar_pedidos_jsonb.sql    ← itens de pedido consolidados em JSONB
0007_add_soft_delete_usuarios.sql
0008_add_bloqueado_usuarios_lojas.sql
0009_unique_celular.sql
0010_pedido_usuario_opcional.sql
0011_categorias_ordem_unique.sql
```

As migrações são aplicadas automaticamente no startup via `sqlx::migrate!()`. O histórico conta uma evolução controlada do schema: nunca houve DROP TABLE desestruturado, soft-deletes foram adicionados sem quebrar dados existentes.

### Decisão JSONB (migração 0006)

A migração `0006` consolidou as tabelas `itens_pedido`, `partes_item_pedido` e `adicionais_item_pedido` em uma única coluna JSONB `itens` na tabela `pedidos`. Isso reduz JOINs e simplifica queries de leitura, mas transfere a responsabilidade de integridade relacional para a aplicação. A troca é aceitável dado o modelo de domínio (um pedido e seus itens são sempre lidos e escritos juntos — são um agregado).

### Bug Corrigido em 2026-04-21

O campo `itens_json` no model `Pedido` tinha `#[sqlx(skip)]`, fazendo com que a coluna JSONB nunca fosse lida do banco. Todos os endpoints de listagem retornavam `"itens": []`. Corrigido para `#[sqlx(rename = "itens")]` com tipo `serde_json::Value`. Feature `json` adicionada ao sqlx no workspace.

---

## 4. Autenticação e Segurança

### JWT

- Token válido por 24 horas
- Assinado com HS256 via `jsonwebtoken`
- A cada request autenticado, o usuário é **rebuscado no banco** para verificar existência, ativação e bloqueio
- Middleware `optional_auth_middleware` para rotas que aceitam usuário anônimo (pedidos balcão)

### Senhas

Hashing com **Argon2id** (padrão atual recomendado para senhas) com salt aleatório por usuário:

```rust
let salt = SaltString::generate(&mut rand::thread_rng());
let argon2 = Argon2::default();
let senha_hash = argon2.hash_password(senha.as_bytes(), &salt)?.to_string();
```

### Sistema de Permissões

Extractors customizados como camada de autorização:

```rust
pub struct AdminPermission(pub Usuario);  // requer classe = "administrador"
pub struct OwnerPermission(pub Usuario);  // requer classe = "owner" ou email == OWNER_EMAIL
```

Falham com `403 Forbidden` antes de o handler ser chamado, sem código repetido em cada handler.

### Problemas de Segurança

| Problema | Severidade | Detalhe |
|---|---|---|
| `JWT_SECRET` com fallback `"secret"` | Crítica em produção | Se `JWT_SECRET` não for definido, qualquer pessoa pode forjar tokens |
| Sem validação estruturada em DTOs | Alta | Campos como `email`, `celular`, `nome` aceitam qualquer string |
| Sem rate limiting | Média | Endpoint de login pode ser alvo de brute force |
| `/api/wipe` em produção | Crítica | Apaga todo o banco; está protegido por `OwnerPermission` mas não deveria existir |

---

## 5. Infraestrutura e Deploy

### Docker

Build multi-stage com imagem final mínima:

```dockerfile
FROM rust:1.88-bookworm AS builder
# ... compila binário release

FROM debian:bookworm-slim AS runtime
# ... copia apenas o binário + migrations
# roda como non-root user (appuser:1000)
```

```yaml
# docker-compose.yml (4 serviços)
services:
  postgres:   # 15.2, com healthcheck
  rabbitmq:   # 3-management-alpine, com healthcheck
  api:        # depende de postgres + rabbitmq saudáveis
  worker:     # depende de postgres + rabbitmq saudáveis
  scheduler:  # depende de postgres + rabbitmq saudáveis
```

### Problemas de Infraestrutura

| Problema | Severidade |
|---|---|
| `network_mode: "host"` no docker-compose | Média — impede isolamento entre serviços |
| Sem CI/CD pipeline | Alta — deploys manuais são propensos a erro |
| Sem monitoramento (Prometheus, Grafana) | Média — invisibilidade em produção |
| Sem healthcheck no endpoint `/api/ok` exposto no load balancer | Baixa |

---

## 6. Problemas Críticos Identificados

### 6.1 Ausência de Testes Automatizados

**Este é o problema mais sério para sustentabilidade a longo prazo.**

Cobertura de testes atual: praticamente zero. O diretório `tests/` existe mas está vazio.

O impacto não é sentido hoje, com 10-15 endpoints ativos e lógica relativamente simples. O impacto é sentido no ano 2 quando:
- O `PedidoService` tem 15 regras de negócio interdependentes
- Uma correção de bug no cálculo de desconto de cupom quebra o cálculo de promoção
- O Claude refatora um service e não tem como saber o que quebrou
- Você descobre o problema em produção, não em desenvolvimento

**Prioridade de cobertura:**

```
1. DomainError e EstadoDePedido (lógica pura — 30 minutos de trabalho)
2. calcular_preco_por_partes, Cupom.calcular_desconto (matemática crítica)
3. PedidoService (maior complexidade de negócio)
4. UsuarioService (autenticação, hashing)
5. Integration tests com testcontainers-rs (PostgreSQL real)
```

### 6.2 `Result<T, String>` nos Services

Os ports retornam `DomainResult<T>` corretamente, mas a maioria dos services e repositories retornam `Result<T, String>` internamente. A conversão acontece na fronteira `impl Port for Repository`:

```rust
// Em usuario_repository.rs
async fn criar(&self, entity: &Usuario) -> DomainResult<Uuid> {
    <Self as Repository<Usuario>>::criar(self, entity)
        .await
        .map_err(|e| DomainError::Internal(e))  // String → DomainError
}
```

O problema: todos os erros de negócio chegam como `DomainError::Internal(string)`, perdendo a semântica. "Email já cadastrado" deveria ser `DomainError::Conflict`, não `Internal`. O `AppError` não consegue distinguir e mapeia tudo para 500.

**Solução:** Migrar services para `DomainResult<T>` incrementalmente. Não é urgente hoje, mas deve ser feito antes de a codebase crescer além de 30-40 endpoints.

### 6.3 Ausência de CI/CD

Sem pipeline automatizado:
- Não há garantia de que `cargo clippy` passa antes de um merge
- Não há garantia de que migrações são válidas antes do deploy
- Deploys são manuais e dependem de disciplina humana

Um pipeline mínimo resolve isso:

```yaml
# .github/workflows/ci.yml
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test --all
- cargo build --release
```

### 6.4 Sem Paginação nos Endpoints de Listagem

Todos os `listar_*` retornam registros ilimitados:

```rust
pub async fn listar_todos(&self) -> DomainResult<Vec<Usuario>>
// SELECT * FROM usuarios → sem LIMIT
```

Com dados reais em produção (milhares de pedidos, centenas de produtos), isso se torna um problema de memória e latência. A solução padrão é adicionar `limit: i32` e `offset: i32` ou cursor-based pagination.

---

## 7. Pontos Fortes para Desenvolvimento Assistido por IA

O projeto tem características que fazem o desenvolvimento com IA funcionar bem de forma sustentável:

### Documentação como Contexto Persistente

`CLAUDE.md`, `API.md`, `CLEAN_ARCHITECTURE_GUIDE.md` e `pendencias.md` formam um corpo de documentação que se comporta como memória de longo prazo. O Claude (ou qualquer IA) consegue ler esses arquivos e entender:
- Onde cada coisa vai
- Quais padrões seguir
- O que está pendente
- O histórico de decisões

Isso é raro e valioso. A maioria dos projetos com desenvolvimento assistido por IA perde consistência ao longo do tempo porque o contexto não é documentado. Aqui está.

### Padrão Arquitetural Sem Ambiguidade

O `CLEAN_ARCHITECTURE_GUIDE.md` tem um tutorial de 11 passos mostrando como criar uma nova entidade do zero. Quando o Claude recebe "adiciona o módulo X", não há dúvida sobre o que criar:

```
1. Model (models/x.rs)
2. Port (ports/x_port.rs)
3. Repository (repositories/x_repository.rs)
4. Service (services/x_service.rs)
5. Usecase (usecases/x.rs)  ← se necessário
6. Handler (api/handlers/x/)
7. Router (api/handlers/routers/x.rs)
8. AppState (injetar o service)
9. Migration (se novo schema)
10. Documentação (API.md, pendencias.md)
```

Cada passo tem um template claro. Isso é o que garante consistência em 3-4 anos.

### Rust como Linguagem para IA

O compilador do Rust funciona como validador das sugestões da IA. Se o Claude gerar código com um campo errado, uma trait não implementada, ou um tipo incompatível, o `cargo check` falha imediatamente com mensagem de erro clara. Isso fecha o loop de feedback muito mais rápido que em linguagens dinâmicas.

---

## 8. Mapa de Prioridades

### Crítico — Deve ser feito antes de ir para produção com carga real

| # | Tarefa | Esforço |
|---|---|---|
| 1 | Escrever testes para `PedidoService` e `UsuarioService` | 2-3 dias |
| 2 | Escrever testes unitários para lógica de domínio (`EstadoDePedido`, cálculos) | 1 dia |
| 3 | Configurar CI com `cargo test + cargo clippy` | 2 horas |
| 4 | Garantir que `JWT_SECRET` não tem fallback em produção | 30 minutos |
| 5 | Remover ou proteger melhor `/api/wipe` | 1 hora |

### Importante — Resolve dívida técnica antes que cresça

| # | Tarefa | Esforço |
|---|---|---|
| 6 | Adicionar paginação aos endpoints de listagem | 1 dia |
| 7 | Migrar services para `DomainResult<T>` | 3-4 dias (incremental) |
| 8 | Adicionar validação estruturada em DTOs (crate `validator`) | 1-2 dias |
| 9 | Integration tests com `testcontainers-rs` | 2-3 dias |
| 10 | Corrigir `network_mode: "host"` no docker-compose | 1 hora |

### Desejável — Qualidade operacional em escala

| # | Tarefa | Esforço |
|---|---|---|
| 11 | Logging estruturado JSON para produção | 1 dia |
| 12 | Rate limiting no endpoint de login | 2 horas |
| 13 | Monitoramento com Prometheus + Grafana | 2 dias |
| 14 | Compressão de resposta HTTP (`tower-http CompressionLayer`) | 1 hora |
| 15 | Cache Redis para cupons e configurações de loja | 2-3 dias |

---

## 9. Conclusão

O projeto Chickie tem uma base arquitetural sólida que sustenta 3-4 anos de desenvolvimento. A separação de camadas é profissional, os contratos de domínio via traits são o padrão correto, e a documentação suporta o ciclo de desenvolvimento assistido por IA de forma que a maioria dos projetos não tem.

O que impede a classificação de "produção-ready hoje" é a combinação de zero testes com zero CI. Com esses dois itens endereçados, a arquitetura está em condição de receber features continuamente sem degradação de qualidade.

A dívida técnica atual (paginação, `Result<String>` nos services, validação de DTOs) é gerenciável e pode ser paga incrementalmente sem reescritas. Não há nenhuma decisão arquitetural que precise ser revertida.

---

*Gerado em 2026-04-21. Reanalisar após mudanças estruturais significativas no projeto.*
