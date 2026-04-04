# Chickie — QWEN.md

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

---

## Estrutura de Módulos

```
src/
├── main.rs                 # Bootstrap, tracing, bind do servidor
├── database.rs             # Criação do pool PostgreSQL
├── utils.rs                # Utilitários gerais (ex: agora())
│
├── models/                 # Structs de domínio (Serialize/Deserialize)
│   ├── mod.rs
│   ├── usuario.rs
│   ├── loja.rs
│   ├── pedido.rs
│   ├── produto.rs
│   ├── avaliacao.rs
│   ├── cupom.rs
│   └── ...
│
├── repositories/           # Acesso direto ao banco (queries SQL)
│   ├── mod.rs              # Trait Repository<T> com defaults
│   ├── usuario_repository.rs
│   ├── loja_repository.rs
│   ├── pedido_repository.rs
│   └── ...                 # Um arquivo por entidade
│
├── services/               # Regras de negócio, orquestra repositories
│   ├── mod.rs
│   ├── usuario_service.rs
│   ├── loja_service.rs
│   ├── pedido_service.rs
│   ├── catalogo_service.rs
│   └── marketing_service.rs
│
└── api/                    # Handlers Axum, rotas, AppState
    ├── mod.rs              # Declaração de módulos e re-exports
    ├── routers.rs          # Definição de todas as rotas
    ├── state.rs            # AppState (estado global compartilhado)
    ├── auth.rs             # JWT middleware + criação de token
    ├── dto/mod.rs          # Request DTOs + AppError + Claims
    ├── wipe.rs             # ⚠️ Endpoint de wipe do banco (dev only)
    │
    ├── usuario/            # Handlers de usuário
    ├── loja/               # Handlers de loja
    ├── pedido/             # Handlers de pedido
    ├── produto/            # Handlers de produto
    ├── cupom/              # Handlers de cupom
    ├── marketing/          # Handlers de avaliação (loja/produto)
    └── usecases/           # Casos de uso (orquestram services + usuário)
        ├── catalogo.rs
        ├── pedido.rs
        └── marketing.rs
```

---

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
- **Repository Pattern:** trait genérica `Repository<T>` com defaults

### Camadas

```
┌─────────────────────────────────────────┐
│           API Layer (Axum)              │  ← Handlers, rotas, DTOs
├─────────────────────────────────────────┤
│        Use Case Layer                   │  ← Casos de uso orquestradores
├─────────────────────────────────────────┤
│        Service Layer                    │  ← Regras de negócio
├─────────────────────────────────────────┤
│     Repository Layer (sqlx)             │  ← Acesso ao banco
├─────────────────────────────────────────┤
│        Domain Layer (models)            │  ← Entidades, value objects
└─────────────────────────────────────────┘
```

### Trait `Repository<T>`

Definida em `repositories/mod.rs`, fornece métodos default para eliminar repetição:

| Método                   | Default? | Descrição                        |
|--------------------------|----------|----------------------------------|
| `buscar_por_uuid`        | ✅ Sim   | Busca entidade por UUID          |
| `listar_todos`           | ✅ Sim   | Lista todas as entidades         |
| `deletar`                | ✅ Sim   | Deleta por UUID com msg de erro  |
| `criar`                  | ❌ Não   | Insert específico por entidade   |
| `atualizar`              | ❌ Não   | Update específico por entidade   |
| `listar_todos_por_loja`  | ❌ Não   | Filtra por loja (varia por repo) |

Cada repositório implementa também:
- `fn table_name(&self) -> &'static str` — nome da tabela
- `fn entity_name(&self) -> &'static str` — nome da entidade (para erros)
- `fn pool(&self) -> &PgPool` — acesso ao pool

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
- `AppError` enum em `api/dto/mod.rs`: `NotFound`, `Internal`, `BadRequest`
- **Nunca** usar `.unwrap()` fora do bootstrap do `main.rs`
- Usar `?` com `From<String> for AppError` para conversão automática

### Rotas

| Padrão                          | Exemplo                                    |
|---------------------------------|--------------------------------------------|
| Todas sob `/api`                | `POST /api/pedidos`                        |
| Health check em `/`             | `GET /` → `handler_ok`                     |
| Fallback 404 genérico           | qualquer rota não mapeada                  |
| Auth via middleware             | Aplicado em `/pedidos`, `/usuarios`, `/lojas`, `/produtos`, `/cupons` |
| Sem auth                        | `/auth/*`, `/wipe`, `/ok`                  |

### AppState

```rust
pub struct AppState {
    // Services (alta abstração)
    pub usuario_service: UsuarioService,
    pub loja_service: LojaService,
    pub catalogo_service: CatalogoService,
    pub pedido_service: PedidoService,
    pub marketing_service: MarketingService,

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

#### Usuários & Autenticação

| Entidade   | Descrição                                              |
|------------|--------------------------------------------------------|
| `Usuario`  | Usuário do sistema (cliente, entregador ou admin). Admin pode cadastrar uma ou mais lojas. |

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
| `Promocao`            | Promoção aplicável à loja (corrigir: atualmente aplica para toda a loja, não por produto/categoria). |
| `AvaliacaoDeLoja`     | Avaliação de loja feita por usuário (nota 0-5 + comentário). |
| `AvaliacaoDeProduto`  | Avaliação de produto feita por usuário (só via pedido autenticado). |

#### Operacional

| Entidade                      | Descrição                                            |
|-------------------------------|------------------------------------------------------|
| `Cliente`                     | Usuário que segue uma loja (relacionamento user-loja). |
| `Entregador`                  | Entregador vinculado a uma loja.                     |
| `Funcionario`                 | Funcionário vinculado a uma loja.                    |
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
Admin → cadastra-se como usuário
     → cadastra sua loja (dados, logo, slug, etc.)
     → configura catálogo (categorias, produtos, ingredientes, adicionais)
     → define horários de funcionamento
     → cria promoções
     → configura entregadores e funcionários
```

#### 2. Navegação e Pedido (Cliente)

```
Cliente → cadastra-se como usuário
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

- [ ] `Promocao`: modelo atual aplica para toda a loja; deveria ser por produto/categoria
- [ ] `ClienteRepository::listar_todos_por_loja`: retorna `Vec<Produto>` ao invés de `Vec<Cliente>` (bug)
- [ ] Endpoint `/wipe`: remover antes de deploy em produção

---

## Histórico de Mudanças Recentes

| Data        | Mudança                                            |
|-------------|----------------------------------------------------|
| 2026-04-03  | Repositórios extraídos para módulo com trait defaults |
| 2026-04-03  | SQL queries otimizadas (indentação compacta)       |
| 2026-04-03  | Endpoint `DELETE /api/wipe` criado (dev only)      |
| 2026-04-03  | Endpoints de avaliação de loja e produto criados   |
| 2026-04-03  | `MarketingUsecase` implementado                    |
| 2026-04-03  | `MarketingService` agora deriva `Clone`            |
