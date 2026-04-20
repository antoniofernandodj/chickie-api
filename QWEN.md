# Chickie

> Documento de referência para desenvolvimento com Qwen Code no projeto Chickie.

---

## Visão Geral

API REST em Rust (Axum + Tokio) para o sistema de pedidos e entregas **Chickie**.

- **Banco de dados:** PostgreSQL via `sqlx`
- **Deploy:** Docker no Dokploy
- **Arquitetura:** Hexagonal / Clean Architecture com camadas bem definidas

---

## Stack Técnica

| Componente          | Tecnologia              | Versão   |
|---------------------|-------------------------|----------|
| Linguagem           | Rust                    | 1.88     |
| Edição              | Rust 2024               |          |
| HTTP Framework      | Axum                    | 0.8      |
| Runtime Async       | Tokio                   |          |
| Database            | sqlx (PostgreSQL)       |          |
| Logging             | tracing + tracing-subscriber |     |
| Serialização        | serde / serde_json      |          |
| JWT                 | jsonwebtoken            |          |
| Timestamps          | chrono::DateTime<Utc>   |          |
| Horários            | chrono::NaiveTime       |          |
| Decimais (NUMERIC)  | rust_decimal::Decimal   |          |

---

## Estrutura de Módulos

### Core (crates/core/src/)

```
crates/core/src/
├── domain/                     # Camada de domínio puro
│   ├── errors/mod.rs           # DomainError enum + DomainResult<T>
│   └── enums/                  # Enums puros (sem sqlx Encode/Decode)
│       ├── classe_usuario.rs
│       ├── estado_pedido.rs
│       ├── status_cupom.rs
│       ├── tipo_calculo.rs
│       └── tipo_escopo.rs
│
├── ports/                      # Contratos (traits sem sqlx)
│   ├── mod.rs                  # Re-exports de todos os ports
│   ├── repository.rs           # RepositoryPort<T> genérico
│   ├── storage.rs              # ImageStoragePort
│   ├── usuario_port.rs
│   ├── loja_port.rs
│   ├── pedido_port.rs
│   ├── produto_port.rs
│   ├── categoria_port.rs
│   ├── adicional_port.rs
│   ├── cupom_port.rs
│   ├── promocao_port.rs
│   ├── endereco_entrega_port.rs
│   ├── endereco_usuario_port.rs
│   ├── endereco_loja_port.rs
│   ├── avaliacao_loja_port.rs
│   ├── avaliacao_produto_port.rs
│   ├── config_pedido_port.rs
│   ├── horario_port.rs
│   ├── funcionario_port.rs
│   ├── entregador_port.rs
│   ├── cliente_port.rs
│   ├── ingrediente_port.rs
│   └── loja_favorita_port.rs
│
├── adapters/                   # Implementações sqlx dos ports
│   ├── mod.rs
│   ├── usuario_adapter.rs
│   └── loja_adapter.rs
│
├── models/                     # Entidades (FromRow + Serialize/Deserialize)
│   ├── mod.rs
│   ├── usuario.rs
│   ├── loja.rs
│   ├── pedido.rs
│   ├── produto.rs
│   └── ...                     # Uma por entidade
│
├── repositories/               # Queries SQL, implementam ports
│   ├── mod.rs                  # Trait Repository<T> com defaults
│   ├── usuario_repository.rs   # impl UsuarioRepositoryPort
│   ├── loja_repository.rs      # impl LojaRepositoryPort
│   └── ...                     # impl PortTrait para cada repo
│
├── services/                   # Regras de negócio, dependem de PORTS
│   ├── mod.rs
│   ├── usuario_service.rs      # usa Arc<dyn UsuarioRepositoryPort>
│   ├── loja_service.rs         # usa Arc<dyn ...Port> para 7 repos
│   ├── pedido_service.rs
│   └── ...                     # 15 services no total
│
├── usecases/                   # Orquestradores para API
│   ├── mod.rs
│   ├── pedido.rs               # PedidoUsecase → PedidoService
│   ├── catalogo.rs             # CatalogoUsecase → CatalogoService
│   ├── marketing.rs
│   ├── admin.rs
│   └── ...                     # 9 usecases
│
└── utils/                      # Utilitários
```

### API (crates/api/src/)

```
crates/api/src/
├── main.rs                     # Bootstrap, tracing, bind
├── infrastructure/
│   └── database.rs             # Pool PostgreSQL + migrations
│
└── handlers/
    ├── mod.rs                  # Declaração de módulos
    ├── routers.rs              # Definição de todas as rotas
    ├── state.rs                # AppState (injeção de dependência)
    ├── auth.rs                 # JWT middleware
    ├── dto/mod.rs              # Request DTOs + AppError + Claims
    ├── openapi.rs              # Swagger/OpenAPI
    │
    ├── usuario/                # Handlers de usuário
    ├── loja/                   # Handlers de loja
    ├── pedido/                 # Handlers de pedido
    ├── produto/                # Handlers de produto
    ├── cupom/                  # Handlers de cupom
    ├── catalogo/               # Handlers de catálogo
    ├── endereco_entrega/       # Handlers de endereço de entrega
    ├── endereco_usuario/       # Handlers de endereço de usuário
    ├── endereco_loja/          # Handlers de endereço de loja
    ├── loja_favorita/          # Handlers de favoritos
    ├── marketing/              # Handlers de avaliação
    ├── funcionario/            # Handlers de funcionário
    ├── entregador/             # Handlers de entregador
    ├── ingrediente/            # Handlers de ingrediente
    ├── horario/                # Handlers de horário
    ├── config_pedido/          # Handlers de config de pedido
    └── routers/                # Arquivos de rotas individuais
```

---

## Observações importantes

- Todo endpoint que por pedido para ser feito nunca vai conter logica alguma.
vai usar na verdade algum usecase, que vai usar algum service, que vai usar
os repositórios. então sempre que for pedido um endpoint, deve-se observar
esta pilha.
- Logo após ser editados documentos de projeto, toda a documentação deve
ser atualizada logo em seguida, @QWEN.md, @CLAUDE.md, @pendencias.md e @API.md
- Sempre que eu mencionar documentação completa estou falando de @API.md,
@QWEN.md, @CLAUDE.md, @README.md e @pendencias.md

### Regra Arquitetural Obrigatória

**TODO endpoint segue estritamente:**

```
Handler → Usecase → Service → Port (trait) → Repository → Database
```

- Handlers **NUNCA** contêm lógica de negócio, queries SQL, ou chamadas diretas a repositórios
- Services dependem de **port traits** (`Arc<dyn XPort>`), não de repositórios concretos
- Repositories implementam seus respectivos ports (`impl XPort for YRepository`)
- `DomainError` é o tipo de erro do domínio; `AppError` na API faz o mapeamento para HTTP
- Para criar novas entidades do zero, consulte `CLEAN_ARCHITECTURE_GUIDE.md`

## Microserviços (Visão Futura)

| Microserviço              | Responsabilidade                        |
|---------------------------|-----------------------------------------|
| **Chickie**               | Sistema core de pedidos e entregas      |
| **ChickieSupplyChain**    | Relacionamento com fornecedores         |
| **ChickieAnalytics**      | Análise de dados e métricas             |
| **ChickieAdmin**          | Administração e gerenciamento           |
| **ChickieAuth**           | Autenticação e autorização centralizada |
| **ChickiePayment**        | Processamento de pagamentos             |
| **ChickiePushNotification** | Notificações push                     |
| **ChickieWorker**         | Tarefas assíncronas em background       |
| **ChickieUI**             | Interface do usuário (frontend)         |

---

## Arquitetura

### Princípios Adotados

- **Hexagonal (Ports & Adapters):** domínio isolado de infraestrutura
- **Clean Architecture:** camadas com dependência unidirecional
- **Domain-Driven Design:** agregados, value objects, repositórios
- **Repository Pattern:** trait genérica `Repository<T>` + traits específicas por entidade (ports)

### Camadas

```
┌─────────────────────────────────────────────────────────┐
│           API Layer (Axum)                              │
│  Handlers → DTOs → chamam usecases → retornam response   │
├─────────────────────────────────────────────────────────┤
│        Use Case Layer                                   │
│  Orquestram services, carregam contexto (usuario, loja)  │
├─────────────────────────────────────────────────────────┤
│        Service Layer                                    │
│  Regras de negócio, dependem de PORT TRAITS              │
├─────────────────────────────────────────────────────────┤
│        Port Layer (Traits)                              │
│  23 traits sem sqlx — contratos de interface             │
├─────────────────────────────────────────────────────────┤
│     Repository Layer (sqlx)                             │
│  20 repositórios implementando port traits               │
├─────────────────────────────────────────────────────────┤
│        Domain Layer                                     │
│  DomainError, enums puros, models (FromRow + serde)      │
└─────────────────────────────────────────────────────────┘
```

### Port Traits

Definidos em `ports/`, são contratos que services e usecases dependem. **Nenhum port menciona sqlx.**

| Port | Métodos principais |
|------|-------------------|
| `UsuarioRepositoryPort` | criar, buscar_por_uuid, buscar_por_email, listar_todos, atualizar |
| `LojaRepositoryPort` | criar, buscar_por_uuid, buscar_por_slug, pesquisar, listar_todos |
| `PedidoRepositoryPort` | criar, buscar_completo, buscar_completos_por_loja, atualizar_status |
| `ProdutoRepositoryPort` | criar, buscar_por_uuid, listar_por_loja, atualizar, deletar |
| `... + 19 outros` | cada entidade tem seu port |

### Trait `Repository<T>`

Definida em `repositories/mod.rs`, fornece métodos default para eliminar repetição:

| Método                   | Default? | Descrição                        |
|--------------------------|----------|----------------------------------|
| `buscar_por_uuid`        | ✅ Sim   | Busca entidade por UUID          |
| `listar_todos`           | ✅ Sim   | Lista todas as entidades         |
| `deletar`                | ✅ Sim   | Deleta por UUID com msg de erro  |
| `criar`                  | ❌ Não   | Insert específico por entidade   |
| `atualizar`              | ❌ Não   | Update específico por entidade   |

Cada repositório implementa também:
- `fn table_name(&self) -> &'static str` — nome da tabela
- `fn entity_name(&self) -> &'static str` — nome da entidade (para erros)
- `fn pool(&self) -> &PgPool` — acesso ao pool
- `impl XPort for YRepository` — conecta ao layer de ports

### DomainError

Definido em `domain/errors/mod.rs`:

```rust
pub enum DomainError {
    NotFound { entity, id },
    BusinessRule(String),
    Validation(String),
    Conflict { entity, field },
    InvalidState { current, attempted, allowed },
    Internal(String),
}
```

`AppError` na API converte `DomainError` para status HTTP corretos (404, 400, 403, 409, 422, 500).

---

## Convenções de Desenvolvimento

### Logging

| Nível    | Uso                                    |
|----------|----------------------------------------|
| `info!`  | Fluxo normal da aplicação              |
| `warn!`  | Situações recuperáveis                 |
| `error!` | Falhas                                 |
| `debug!` | Detalhes de desenvolvimento            |

> **Nunca usar** `println!` ou `eprintln!` fora do `main.rs`. Sempre usar `tracing`.

### Tratamento de Erros

- Handlers retornam `Result<impl IntoResponse, AppError>`
- `AppError` enum em `api/dto/mod.rs`: `NotFound`, `Internal`, `BadRequest`, `Unauthorized`
- **Nunca** usar `.unwrap()` fora do bootstrap do `main.rs`
- Usar `?` com `From<String> for AppError` para conversão automática

### Rotas

| Padrão                          | Exemplo                                    |
|---------------------------------|--------------------------------------------|
| Todas sob `/api`                | `POST /api/pedidos`                        |
| Health check em `/`             | `GET /` → `handler_ok`                     |
| Fallback 404 genérico           | qualquer rota não mapeada                  |
| Auth via middleware             | Aplicado em `/pedidos` (parcial), `/usuarios`, `/produtos`, `/marketing` (parcial), `/catalogo`, `/enderecos-entrega`, `/enderecos-usuario`, `/favoritos`, `/admin` |
| Sem auth                        | `/auth/*`, `/ok`, `GET /api/lojas/`, `GET /api/marketing/cupons/{codigo}`, `/wipe` (dev only) |

### Referência Completa de Endpoints

#### Autenticação (sem auth)

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/auth/signup` | Cadastro de usuário |
| `POST` | `/api/auth/login` | Login (gera JWT) |
| `GET` | `/api/auth/me` | Obter usuário autenticado | 🔒 |

> **Bloqueio de login:** Usuários com `bloqueado = true` são rejeitados no login E no middleware JWT.

#### Sistema de Permissões

| Extractor | Permissão | Uso |
|-----------|-----------|-----|
| `AdminPermission` | `classe = "administrador"` | Criar lojas, funcionários, entregadores |
| `OwnerPermission` | `classe = "owner"` OU `email == OWNER_EMAIL` | God mode — acesso total |
| `is_self_or_owner` | Próprio usuário OU owner | Marcar/desmarcar remoção |

**Variável de ambiente:** `OWNER_EMAIL=seu@email.com` — define o dono da plataforma sem necessidade de seed no banco.

#### Usuários (auth required)

| Método | Rota | Descrição |
|--------|------|-----------|
| `GET` | `/api/usuarios/` | Listar usuários |

#### Lojas Públicas (sem auth para listagem)

| Método | Rota | Descrição |
|--------|------|-----------|
| `GET` | `/api/lojas/` | Listar lojas |

#### Administração (auth required, apenas admin)

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/admin/lojas` | Criar loja |
| `GET` | `/api/admin/lojas/listar` | Listar todas as lojas |
| `POST` | `/api/admin/lojas/{loja_uuid}/funcionarios` | Adicionar funcionário |
| `POST` | `/api/admin/lojas/{loja_uuid}/entregadores` | Adicionar entregador |

#### Produtos (GETs públicos, mutações requerem auth)

| Método | Rota | Auth | Descrição |
|--------|------|------|-----------|
| `GET` | `/api/produtos/listar/{loja_uuid}` | — | Listar produtos da loja |
| `GET` | `/api/produtos/categoria/{categoria_uuid}` | — | Listar produtos por categoria |
| `GET` | `/api/produtos/{uuid}` | — | Buscar produto por UUID |
| `POST` | `/api/produtos/` | 🔒 | Criar produto |
| `PUT` | `/api/produtos/{uuid}` | 🔒 | Atualizar produto |
| `DELETE` | `/api/produtos/{uuid}` | 🔒 | Deletar produto |
| `PUT` | `/api/produtos/{loja_uuid}/{produto_uuid}/disponibilidade` | 🔒 | Atualizar disponibilidade |
| `POST` | `/api/produtos/{uuid}/imagem` | 🔒 | Subir imagem do produto (S3) |

#### Horários de Funcionamento (GET público, mutações requerem auth)

| Método | Rota | Auth | Descrição |
|--------|------|------|-----------|
| `GET` | `/api/horarios/{loja_uuid}` | — | Listar horários |
| `POST` | `/api/horarios/{loja_uuid}` | 🔒 | Criar ou atualizar horário |
| `PUT` | `/api/horarios/{loja_uuid}/dia/{dia_semana}/ativo` | 🔒 | Ativar/desativar dia |
| `DELETE` | `/api/horarios/{loja_uuid}/dia/{dia_semana}` | 🔒 | Deletar horário do dia |

#### Configurações de Pedido (🔒)

| Método | Rota | Descrição |
|--------|------|-----------|
| `GET` | `/api/config-pedido/{loja_uuid}` | Buscar configuração |
| `PUT` | `/api/config-pedido/{loja_uuid}` | Salvar configuração |

#### Cupons Admin (🔒)

| Método | Rota | Descrição |
|--------|------|-----------|
| `PUT` | `/api/cupons/admin/{loja_uuid}/{uuid}` | Atualizar cupom |
| `DELETE` | `/api/cupons/admin/{loja_uuid}/{uuid}` | Deletar cupom |

#### Ingredientes (🔒)

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/ingredientes/{loja_uuid}` | Criar ingrediente |
| `GET` | `/api/ingredientes/{loja_uuid}` | Listar ingredientes |
| `PUT` | `/api/ingredientes/{loja_uuid}/{uuid}` | Atualizar ingrediente |
| `DELETE` | `/api/ingredientes/{loja_uuid}/{uuid}` | Deletar ingrediente |

#### Funcionários (🔒)

| Método | Rota | Descrição |
|--------|------|-----------|
| `GET` | `/api/funcionarios/{loja_uuid}` | Listar funcionários |
| `PUT` | `/api/funcionarios/{loja_uuid}/{uuid}` | Atualizar funcionário (inclui campos de usuário opcionais) |
| `PUT` | `/api/funcionarios/{loja_uuid}/usuarios/{usuario_uuid}/credenciais` | Trocar email/senha |

#### Entregadores (🔒)

| Método | Rota | Descrição |
|--------|------|-----------|
| `GET` | `/api/entregadores/{loja_uuid}` | Listar entregadores |
| `PUT` | `/api/entregadores/{loja_uuid}/{uuid}` | Atualizar entregador (inclui campos de usuário opcionais) |
| `PUT` | `/api/entregadores/{loja_uuid}/usuarios/{usuario_uuid}/credenciais` | Trocar email/senha |

#### Catálogo (GETs públicos, mutações requerem auth)

| Método | Rota | Auth | Descrição |
|--------|------|------|-----------|
| `GET` | `/api/catalogo/{loja_uuid}/adicionais` | — | Listar todos os adicionais |
| `GET` | `/api/catalogo/{loja_uuid}/adicionais/disponiveis` | — | Listar adicionais disponíveis |
| `GET` | `/api/catalogo/{loja_uuid}/categorias` | — | Listar categorias |
| `POST` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | Criar adicional |
| `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}` | 🔒 | Atualizar adicional |
| `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/disponibilidade` | 🔒 | Atualizar disponibilidade (true/false) |
| `DELETE` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}` | 🔒 | Deletar adicional |
| `POST` | `/api/catalogo/{loja_uuid}/categorias` | 🔒 | Criar categoria |
| `PUT` | `/api/catalogo/{loja_uuid}/categorias/{uuid}` | 🔒 | Atualizar categoria |
| `DELETE` | `/api/catalogo/{loja_uuid}/categorias/{uuid}` | 🔒 | Deletar categoria (apenas se vazia) |

#### Pedidos (auth required)

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/pedidos/criar` | Criar pedido (`loja_uuid` no body) |
| `GET` | `/api/pedidos/listar` | Listar todos pedidos |
| `GET` | `/api/pedidos/por-loja/{loja_uuid}` | Listar pedidos por loja |
| `GET` | `/api/pedidos/{uuid}` | Buscar pedido por UUID |
| `GET` | `/api/pedidos/{uuid}/com-entrega` | Buscar pedido com endereço de entrega |
| `PUT` | `/api/pedidos/{uuid}/status` | Atualizar status (máquina de estados) |

#### Cupons & Avaliações (auth required, exceto validar cupom)

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/marketing/{loja_uuid}/cupons` | Criar cupom |
| `GET` | `/api/marketing/cupons` | Listar cupons da loja |
| `GET` | `/api/marketing/cupons/{codigo}` | Validar cupom |
| `POST` | `/api/marketing/{loja_uuid}/avaliar-loja` | Avaliar loja |
| `POST` | `/api/marketing/{loja_uuid}/avaliar-produto` | Avaliar produto |
| `POST` | `/api/marketing/{loja_uuid}/promocoes` | Criar promoção (escopo: loja, produto ou categoria) |
| `GET` | `/api/marketing/{loja_uuid}/promocoes` | Listar promoções |
| `PUT` | `/api/marketing/{loja_uuid}/promocoes/{uuid}` | Atualizar promoção |
| `DELETE` | `/api/marketing/{loja_uuid}/promocoes/{uuid}` | Deletar promoção |

#### Endereços de Entrega (auth required)

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/enderecos-entrega/{pedido_uuid}/{loja_uuid}` | Criar endereço para pedido |
| `GET` | `/api/enderecos-entrega/{pedido_uuid}` | Buscar endereço do pedido |

#### Endereços de Usuário (auth required)

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/enderecos-usuario/` | Criar endereço |
| `GET` | `/api/enderecos-usuario/` | Listar endereços do usuário |
| `GET` | `/api/enderecos-usuario/{uuid}` | Buscar endereço |
| `PUT` | `/api/enderecos-usuario/{uuid}` | Atualizar endereço |
| `DELETE` | `/api/enderecos-usuario/{uuid}` | Deletar endereço |

#### Lojas Favoritas (auth required)

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/favoritos/{loja_uuid}` | Adicionar loja às favoritas |
| `DELETE` | `/api/favoritos/{loja_uuid}` | Remover loja das favoritas |
| `GET` | `/api/favoritos/minhas` | Listar minhas lojas favoritas |
| `GET` | `/api/favoritos/{loja_uuid}/verificar` | Verificar se loja é favorita |

#### Administração

| Método | Rota | Descrição |
|--------|------|-----------|
| `DELETE` | `/api/wipe` | ⚠️ Limpar todo o banco (dev only) |

### AppState

```rust
pub struct AppState {
    // Services (alta abstração)
    pub usuario_service: UsuarioService,
    pub loja_service: LojaService,
    pub catalogo_service: CatalogoService,
    pub pedido_service: PedidoService,
    pub marketing_service: MarketingService,
    pub endereco_entrega_service: EnderecoEntregaService,
    pub endereco_usuario_service: EnderecoUsuarioService,
    pub loja_favorita_service: LojaFavoritaService,

    // Repositórios brutos (buscas simples em handlers)
    pub pedido_repo: Arc<PedidoRepository>,
    pub cupom_repo: Arc<CupomRepository>,
    pub usuario_repo: Arc<UsuarioRepository>,
    pub loja_repo: Arc<LojaRepository>,
    pub produto_repo: Arc<ProdutoRepository>,

    // Pool raw para operações administrativas
    pub db: Arc<PgPool>,
}
```

Injetado via `State(state): State<Arc<AppState>>`.

---

## Variáveis de Ambiente

| Variável      | Padrão  | Descrição                                      |
|---------------|---------|------------------------------------------------|
| `APP_PORT`    | `3000`  | Porta do servidor                              |
| `DATABASE_URL`| —       | String de conexão PostgreSQL (Dokploy internal) |
| `RUST_LOG`    | `info`  | Nível de log (`debug` em desenvolvimento)       |
| `JWT_SECRET`  | `secret`| Chave de assinatura JWT (fallback)              |
| `MODE`        | —       | Se `development`, dropa o banco e reaplica migrações no startup |

---

## Comandos Úteis

```bash
cargo run                        # Rodar localmente
cargo test                       # Executar testes
cargo build --release            # Build de produção
docker build -t chickie .        # Build da imagem Docker
cargo check                      # Verificar compilação sem gerar binário
```

---

## Documentos de Referência

| Documento | Descrição |
|-----------|-----------|
| [`API.md`](./API.md) | Especificação completa de todos os 46 endpoints |
| [`pendencias.md`](./pendencias.md) | Lista de pendências (bugs, melhorias, features faltando) |

---

## Regras — O Que Evitar

- ❌ Não adicionar estado mutável global fora do `AppState`
- ❌ Não expor rotas sem passar pelo nest `/api` (exceto `/` e fallback)
- ❌ Não usar `.unwrap()` em código de produção fora do bootstrap
- ❌ Não usar `println!` / `eprintln!` — usar `tracing`
- ❌ Não criar handlers sem tratamento de erro adequado
- ❌ Não bypassar o `Repository<T>` trait para queries genéricas

---

## Domínio da Aplicação

Sistema de pedidos e entregas de comida, com evolução futura para supply chain.

### Entidades

#### Serviços Disponíveis

| Serviço | Responsabilidade |
|---------|-----------------|
| `UsuarioService` | Registro, autenticação, listagem de usuários |
| `LojaService` | Criação de loja, funcionários, entregadores, clientes |
| `CatalogoService` | Produtos, categorias, adicionais |
| `PedidoService` | Criação, busca, listagem de pedidos |
| `MarketingService` | Cupons, promoções, avaliações |
| `EnderecoEntregaService` | Endereços de entrega vinculados a pedidos |
| `EnderecoUsuarioService` | CRUD de endereços de usuários |
| `LojaFavoritaService` | Favoritar/desfavoritar lojas, listar favoritas |

#### Usuários & Autenticação

| Entidade         | Descrição                                                                                         |
|------------------|---------------------------------------------------------------------------------------------------|
| `Usuario`        | Usuário do sistema. Possui campo `classe`: **`cliente`**, **`administrador`**, **`funcionario`**, **`entregador`** ou **`owner`**. |
| `ClasseUsuario`  | Enum com cinco variantes: `Cliente` (padrão), `Administrador`, `Funcionario`, `Entregador`, `Owner`. `Owner` é o dono da plataforma. Apenas admins podem criar lojas. |

#### Lojas & Catálogo

| Entidade              | Descrição                                                        |
|-----------------------|------------------------------------------------------------------|
| `Loja`                | Tenant que vende produtos. Possui slug, logo, horários, etc.    |
| `CategoriaProdutos`   | Categorias como bebidas, pizzas, hambúrgueres.                  |
| `Produto`             | Produto vendável (pizza, hambúrguer, etc.). Pode ser inativado. |
| `Ingrediente`         | Ingrediente usado na descrição/composição de produtos.          |
| `Adicional`           | Ingrediente opcional adicionável a produtos (queijo, cebola).   |

#### Pedidos

| Entidade                    | Descrição                                                            |
|-----------------------------|----------------------------------------------------------------------|
| `Pedido`                    | Pedido de cliente para uma loja. Possui status, total, forma de pagamento. |
| `ItemPedido`                | Item dentro do pedido (ex: 1 pizza grande). Pode ter várias partes.  |
| `ParteDeItemPedido`         | Parte de um item (ex: fatia de pizza de um sabor específico).        |
| `AdicionalDeItemDePedido`   | Adicional vinculado a uma parte específica do item.                  |
| `EnderecoEntrega`           | Endereço de entrega do pedido (snapshot no momento do pedido).       |

#### Marketing & Avaliações

| Entidade              | Descrição                                              |
|-----------------------|--------------------------------------------------------|
| `Cupom`               | Cupom de desconto aplicável a pedidos.                 |
| `UsoCupom`            | Registro de uso de um cupom em um pedido.              |
| `Promocao`            | Promoção aplicável à loja, produto ou categoria (escopo via `tipo_escopo`). |
| `AvaliacaoDeLoja`     | Avaliação de loja feita por usuário (nota 0-5 + comentário). |
| `AvaliacaoDeLojaComUsuario` | Avaliação de loja enriquecida com `usuario_nome` e `usuario_email` via JOIN. Usada no endpoint de listagem. |
| `AvaliacaoDeProduto`  | Avaliação de produto feita por usuário (só via pedido autenticado). |

#### Operacional

| Entidade                      | Descrição                                            |
|-------------------------------|------------------------------------------------------|
| `Cliente`                     | Usuário que é favorito de uma loja (usuário favorito da loja). |
| `LojaFavorita`                | Loja que é favorita de um usuário (loja favorita do usuário). |
| `Entregador`                  | Entregador vinculado a uma loja. Referencia `Usuario` via FK (`usuario_uuid`). |
| `Funcionario`                 | Funcionário vinculado a uma loja. Referencia `Usuario` via FK (`usuario_uuid`). |
| `HorarioFuncionamento`        | Horário de funcionamento por dia da semana.          |
| `ConfiguracaoDePedidosLoja`   | Configurações de pedido da loja (max partes, tipo de cálculo). |
| `EnderecoLoja`                | Endereço físico de uma loja.                         |
| `EnderecoUsuario`             | Endereços salvos de um usuário.                      |

---

### Estrutura Esperada de um Pedido

```
Pedido {
    observacoes: String,
    itens: [
        ItemPedido {
            quantidade: i32,
            observacoes: Option<String>,
            partes: [
                ParteDeItemPedido {
                    produto_nome: String,
                    preco_unitario: f64,
                    posicao: i32,
                    adicionais: [
                        AdicionalDeItemDePedido { nome, descricao, preco }
                    ]
                }
            ]
        }
    ]
}
```

### Fluxo de Uso

#### 1. Cadastro de Loja (Admin)

```
Admin → cadastra-se como usuário (classe: "administrador")
     → cria sua loja via POST /api/admin/lojas (requer JWT de admin)
     → configura catálogo (categorias, produtos, ingredientes, adicionais)
     → define horários de funcionamento
     → cria promoções
     → configura entregadores e funcionários
```

#### 2. Navegação e Pedido (Cliente)

```
Cliente → cadastra-se como usuário (classe: "cliente", padrão)
        → segue lojas preferidas (cria Cliente)
        → acessa página da loja
        → navega catálogo (apenas produtos/adicional ativos)
        → monta pedido (seleciona partes, adicionais, observações)
        → aplica cupom de desconto (opcional)
        → informa endereço de entrega
        → finaliza pedido
```

#### 3. Lifecycle do Pedido

| Status                          | Descrição                                      |
|---------------------------------|------------------------------------------------|
| `criado`                        | Pedido recebido, aguardando confirmação        |
| `aguardando_confirmacao_de_loja`| Loja ainda não confirmou                       |
| `confirmado_pela_loja`          | Loja confirmou o pedido                        |
| `em_preparo`                    | Pedido sendo preparado na cozinha              |
| `pronto_para_retirada`          | Pedido pronto para o cliente                   |
| `saiu_para_entrega`             | Entregador saiu com o pedido                   |
| `entregue`                      | Pedido entregue ao cliente                     |

#### 4. Pós-Entrega

```
Entregador entrega → pedido status → ENTREGUE
                   → cliente avalia a loja
                   → cliente avalia os produtos (só se teve pedido)
```

### Regras de Negócio

| Regra | Detalhe |
|-------|---------|
| Classe de usuário | `cliente` (padrão), `administrador`, `funcionario` ou `entregador`. Definida no signup via campo `classe`. |
| Criar loja | Apenas usuários com `classe = "administrador"` podem criar lojas (`POST /api/admin/lojas`). |
| Produtos/Adicionais inativos | Não são exibidos no catálogo |
| Avaliação de produto | Só via pedido autenticado (evite avaliação fraudulenta) |
| Ingredientes | Não são adicionáveis pelo usuário; servem para descrever o produto |
| Adicionais | Aplicáveis a partes específicas (ex: cebola na fatia portuguesa, não na mussarela) |
| Cupom de desconto | Aplicado no ato da criação do pedido, computa desconto numérico |
| Endereço de entrega | Solicitado no cadastro, mas pode ser sobrescrito no pedido |
| Soft-delete de conta | Marca `a_remover = now() + 1 mês`; scheduler marca `excluída = true` após 30 dias |
| Soft-delete de loja | Mesmo mecanismo de conta |
| Tipo de cálculo de partes | Configuração da loja: `mais_caro` ou `media_ponderada` |

### A Corrigir / Melhorar

- [ ] Endpoint `/wipe`: remover antes de deploy em produção

---

## Histórico de Mudanças Recentes

| Data        | Mudança                                            |
|-------------|----------------------------------------------------|
| 2026-04-20  | **Endpoints públicos de leitura**: GETs de horários, adicionais, categorias e produtos movidos para fora do middleware JWT. Handlers `listar_horarios`, `listar_adicionais`, `listar_adicionais_disponiveis`, `listar_categorias`, `listar_produtos` e `listar_produtos_por_categoria` refatorados para chamar o service diretamente, sem `Extension<Usuario>`. |
| 2026-04-07  | **Campo pizza_mode na categoria**: `categorias_produtos` ganhou campo `pizza_mode BOOLEAN DEFAULT FALSE`. Migration `0004` criada. Stack completa atualizada: model `CategoriaProdutos`, repository SQL, `CatalogoService`, handlers criar/atualizar categoria com DTOs. |
| 2026-04-07  | **Listar pedidos por usuário**: Novo endpoint `GET /api/pedidos/meus` retorna todos os pedidos do usuário autenticado com hidratação completa (itens, partes, adicionais). Pipeline: Handler → Usecase → Service → Repository (`buscar_completos_por_usuario`). |
| 2026-04-07  | **Atribuir entregador ao pedido**: Endpoints `PUT /api/pedidos/{pedido_uuid}/entregador/{loja_uuid}` e `DELETE` para vincular/remover entregador. Migration `0005` adicionou `entregador_uuid UUID FK` à tabela `pedidos`. `Pedido` model, repository (criar/atualizar/novas queries), service, usecase e handlers atualizados. Novo DTO `PedidoComEntregador` com JOIN para dados do entregador. |
| 2026-04-07  | **Buscar pedido com entregador**: Novo endpoint `GET /api/pedidos/{pedido_uuid}/com-entregador` retorna pedido com dados do entregador (nome, veiculo, placa) via LEFT JOIN. |
| 2026-04-05  | **NUMERIC com tipo correto**: Todos os campos `f64`/`Option<f64>` mapeados para `NUMERIC` migrados para `rust_decimal::Decimal`. Afeta `taxa_entrega`, `valor_minimo_pedido`, `raio_entrega_km` (loja), `preco` (produto, adicional), `nota` (avaliação), `latitude/longitude` (endereco), `quantidade/preco_unitario` (ingrediente), `salario` (funcionario), `valor_desconto/valor_minimo` (cupom, promocao), `total/subtotal/desconto` (pedido). |
| 2026-04-05  | **Endpoint minhas lojas**: `GET /api/admin/minhas-lojas` lista lojas criadas pelo admin logado. Tabela `lojas` ganhou campo `criado_por UUID` (FK para `usuarios`). Migration `0003` criada. |
| 2026-04-05  | **Campos TIME corrigidos**: `horario_abertura`, `horario_fechamento` (`loja`) e `abertura`, `fechamento` (`horarios_funcionamento`) migrados de `String` para `chrono::NaiveTime`. `loja_service` agora converte `String → NaiveTime::parse_from_str("%H:%M")`. |
| 2026-04-05  | **Timestamps corrigidos**: Models migrados de `String` para `chrono::DateTime<Utc>`. Repositórios agora omitem `criado_em`/`atualizado_em` em INSERTs/UPDATEs (usam DEFAULT/TRIGGER do PostgreSQL). |
| 2026-04-05  | Máquina de estados do pedido + endpoint `PUT /status` |
| 2026-04-05  | `Promocao` suporta escopo por loja, produto ou categoria |
| 2026-04-05  | Bug fix: `dias_semana_validos` agora usa `Vec<i32>` (era `String` com bug de serialização) |
| 2026-04-05  | Migration `0002` aplicada no boot |
| 2026-04-04  | Endpoints de pedidos: listar_por_loja, buscar_pedido_com_entrega |
| 2026-04-04  | CRUD completo de promoções (listar, atualizar, deletar) |
| 2026-04-04  | Endpoint `GET /api/cupons/` para listar cupons por loja |
| 2026-04-04  | `ClasseUsuario.Owner` adicionado (dono da plataforma) |
| 2026-04-04  | Endpoint `POST /api/cupons/{loja_uuid}/promocoes` criado via MarketingUsecase |
| 2026-04-04  | Endpoints de adicionais (listar, disponíveis, indisponível) criados |
| 2026-04-03  | Tabela `lojas_favoritas` e endpoints de favoritos criados |
| 2026-04-04  | `Funcionario` e `Entregador` agora referenciam `Usuario` via FK (`usuario_uuid`) |
| 2026-04-04  | `ClasseUsuario` expandido com `Funcionario` e `Entregador` |
| 2026-04-03  | `LojaService::adicionar_cliente` agora cria usuário cliente automaticamente |
| 2026-04-03  | `EnderecoEntregaService` e `EnderecoUsuarioService` injetados no AppState |
| 2026-04-03  | `ClasseUsuario` adicionada ao `Usuario` (cliente/admin) |
| 2026-04-03  | Endpoint `POST /api/admin/lojas` protegido por auth + verificação de admin |
| 2026-04-03  | Pasta `src/api/usecases/` movida para `src/usecases/` |
| 2026-04-03  | Repositórios extraídos para módulo com trait defaults |
| 2026-04-03  | SQL queries otimizadas (indentação compacta)       |
| 2026-04-03  | Endpoint `DELETE /api/wipe` criado (dev only)      |
| 2026-04-03  | Endpoints de avaliação de loja e produto criados   |
| 2026-04-03  | `MarketingUsecase` implementado                    |
| 2026-04-03  | `MarketingService` agora deriva `Clone`            |

## AI Added Memories
- Regra arquitetural obrigatória: TODO endpoint deve seguir a pilha Handler → Usecase → Service → Repository → Database. Handlers NUNCA podem conter lógica de negócio, queries SQL, ou chamadas diretas a repositories. Handlers apenas extraem dados da request, instanciam o usecase, chamam seu método e retornam a response. Isso vale para qualquer novo endpoint criado no projeto Chickie API.
- O projeto Chickie API usa o comando `make export-docs` para sincronizar API.md, CLAUDE.md e README.md para o projeto chickie-ui em ../../TS/chickie-ui/api_docs/
