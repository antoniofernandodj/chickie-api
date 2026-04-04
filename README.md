# 🐔 Chickie API

API REST para sistema de pedidos e entregas de comida, construída em **Rust** com **Axum**.

[![Rust](https://img.shields.io/badge/Rust-1.88-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15+-blue.svg)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

---

## 📋 Visão Geral

Chickie é um sistema de delivery que permite:

- 🏪 Lojas (tenants) gerenciarem seus catálogos, produtos e equipes
- 🛒 Clientes navegarem catálogos, montarem pedidos com partes personalizáveis (ex: pizza meio-a-meio) e aplicarem cupons de desconto
- 🛵 Entregadores serem atribuídos a pedidos para entrega
- ⭐ Avaliações de lojas e produtos por clientes autenticados
- 🎫 Cupons de desconto e promoções por dia da semana

---

## 🚀 Quick Start

### Pré-requisitos

- **Rust 1.88+** (edição 2024)
- **PostgreSQL 15+**
- **Docker** (opcional, para rodar o banco localmente)

### Rodando Localmente

1. **Clone o repositório:**
   ```bash
   git clone <repo-url>
   cd chickie-api
   ```

2. **Suba o banco de dados (Docker):**
   ```bash
   docker compose up -d
   ```

3. **Configure o banco:**
   ```bash
   export DATABASE_URL="postgres://myuser:mypassword@localhost:5432/mydatabase"
   sqlx migrate run
   ```

4. **Rode a API:**
   ```bash
   cargo run
   ```

   A API estará disponível em `http://localhost:3000`.

### Build de Produção

```bash
cargo build --release
```

Ou via Docker:

```bash
docker build -t chickie .
docker run -p 3000:3000 \
  -e DATABASE_URL="postgres://..." \
  chickie
```

---

## 📡 Endpoints da API

Todos os endpoints vivem sob `/api`.

### Autenticação

| Método | Rota              | Descrição        | Auth |
|--------|-------------------|------------------|------|
| `POST` | `/api/auth/signup`| Cadastro de usuário | ❌  |
| `POST` | `/api/auth/login` | Login (gera JWT)    | ❌  |

### Lojas

| Método  | Rota           | Descrição        | Auth |
|---------|----------------|------------------|------|
| `POST`  | `/api/lojas/`  | Criar loja       | ✅   |
| `GET`   | `/api/lojas/`  | Listar lojas     | ❌   |

### Produtos

| Método | Rota                | Descrição           | Auth |
|--------|---------------------|---------------------|------|
| `POST` | `/api/produtos/`    | Criar produto       | ✅   |
| `GET`  | `/api/produtos/`    | Listar produtos     | ❌   |
| `PUT`  | `/api/produtos/{uuid}` | Atualizar produto | ✅   |

### Pedidos

| Método | Rota                | Descrição           | Auth |
|--------|---------------------|---------------------|------|
| `POST` | `/api/pedidos/`     | Criar pedido        | ✅   |
| `GET`  | `/api/pedidos/`     | Listar pedidos      | ✅   |
| `GET`  | `/api/pedidos/{uuid}` | Buscar pedido    | ✅   |

### Cupons & Avaliações

| Método | Rota                                    | Descrição              | Auth |
|--------|-----------------------------------------|------------------------|------|
| `POST` | `/api/cupons/`                          | Criar cupom            | ✅   |
| `GET`  | `/api/cupons/{codigo}`                  | Validar cupom          | ❌   |
| `POST` | `/api/cupons/{loja_uuid}/avaliar-loja`  | Avaliar loja           | ✅   |
| `POST` | `/api/cupons/{loja_uuid}/avaliar-produto` | Avaliar produto      | ✅   |

### Administração

| Método   | Rota         | Descrição                      | Auth |
|----------|--------------|--------------------------------|------|
| `DELETE` | `/api/wipe`  | ⚠️ Limpar todo o banco (dev)  | ❌   |

> **⚠️ O endpoint `/api/wipe` é apenas para desenvolvimento. Deve ser removido antes do deploy em produção.**

---

## 🏗️ Arquitetura

### Camadas

```
┌─────────────────────────────────────────┐
│         API Layer (Axum Handlers)       │  ← Rotas, DTOs, validação
├─────────────────────────────────────────┤
│       Use Case Layer                    │  ← Casos de uso orquestradores
├─────────────────────────────────────────┤
│       Service Layer                     │  ← Regras de negócio
├─────────────────────────────────────────┤
│    Repository Layer (sqlx)              │  ← Acesso ao banco
├─────────────────────────────────────────┤
│       Domain Layer (models)             │  ← Entidades, value objects
└─────────────────────────────────────────┘
```

### Estrutura do Projeto

```
src/
├── main.rs                 # Bootstrap, tracing, servidor
├── database.rs             # Pool PostgreSQL
├── utils.rs                # Utilitários (ex: agora())
│
├── models/                 # Entidades de domínio
├── repositories/           # Queries SQL (trait Repository<T>)
├── services/               # Regras de negócio
│
└── api/
    ├── routers.rs          # Definição de rotas
    ├── state.rs            # AppState (estado global)
    ├── auth.rs             # JWT middleware
    ├── dto/                # Request DTOs + AppError
    ├── wipe.rs             # ⚠️ Wipe DB (dev only)
    ├── usuario/            # Handlers de usuário
    ├── loja/               # Handlers de loja
    ├── pedido/             # Handlers de pedido
    ├── produto/            # Handlers de produto
    ├── cupom/              # Handlers de cupom
    ├── marketing/          # Handlers de avaliação
    └── usecases/           # Casos de uso
```

### Stack

| Componente   | Tecnologia                    |
|--------------|-------------------------------|
| Linguagem    | Rust 1.88 (edição 2024)       |
| HTTP         | Axum 0.8                      |
| Runtime      | Tokio                         |
| Database     | sqlx + PostgreSQL 15          |
| Logging      | tracing + tracing-subscriber  |
| Auth         | JWT (jsonwebtoken)            |
| Serialização | serde / serde_json            |

---

## 🔑 Autenticação

A API usa **JWT (JSON Web Token)** para autenticação:

1. `POST /api/auth/signup` — cria usuário e retorna token
2. `POST /api/auth/login` — autentica com email/senha, retorna token
3. Inclua o token no header: `Authorization: Bearer <token>`

Rotas protegidas: pedidos, criação de produtos, cupons, avaliações.

---

## 📦 Estrutura de um Pedido

Pedidos suportam partes personalizáveis (ex: pizza meio-a-meio):

```
Pedido
├── observacoes
├── itens[]
│   ├── quantidade
│   ├── observacoes
│   └── partes[]
│       ├── produto_nome
│       ├── preco_unitario
│       ├── posicao
│       └── adicionais[]
│           ├── nome
│           ├── descricao
│           └── preco
└── endereco_entrega
    ├── logradouro, numero, bairro, cidade, estado
    └── cep
```

### Lifecycle do Pedido

```
criado → aguardando_confirmacao → confirmado → em_preparo
       → pronto_para_retirada → saiu_para_entrega → entregue
```

---

## 🔧 Comandos Úteis

```bash
cargo run                  # Rodar localmente
cargo test                 # Executar testes
cargo check                # Verificar compilação
cargo build --release      # Build de produção
cargo clippy               # Lint do projeto
docker compose up -d       # Subir PostgreSQL local
docker build -t chickie .  # Build da imagem
```

---

## 🌍 Variáveis de Ambiente

| Variável       | Padrão     | Descrição                                  |
|----------------|------------|--------------------------------------------|
| `APP_PORT`     | `3000`     | Porta do servidor                          |
| `DATABASE_URL` | —          | String de conexão PostgreSQL               |
| `RUST_LOG`     | `info`     | Nível de log (`debug` em dev)              |
| `JWT_SECRET`   | `secret`   | Chave de assinatura JWT                    |

---

## 🗄️ Banco de Dados

Migrações são gerenciadas pelo `sqlx-cli`.

```bash
# Criar nova migração
sqlx migrate add <nome_da_migracao>

# Aplicar migrações
sqlx migrate run

# Reverter última migração
sqlx migrate revert
```

Migrações existentes estão em `migrations/0001_criar_tabelas.sql`.

---

## 📝 Convenções de Desenvolvimento

- **Logging:** usar `tracing` — nunca `println!` fora do `main`
- **Erros:** handlers retornam `Result<impl IntoResponse, AppError>`
- **Sem `.unwrap()`:** em código de produção, fora do bootstrap
- **Rotas:** todas sob `/api`, exceto health check (`/`) e fallback 404
- **AppState:** único ponto de estado global compartilhado via `Arc<AppState>`

---

## 🗺️ Roadmap

- [ ] **Pagamentos:** métodos de pagamento, transações, histórico
- [ ] **Endereços:** múltiplos endereços por cliente, validação de área de entrega
- [ ] **Notificações:** alertas de status do pedido, promoções
- [ ] **Cardápio avançado:** variações de produto (P/M/G), produtos em destaque
- [ ] **Rastreamento:** tempo estimado, localização em tempo real
- [ ] **Promoções por produto/categoria** (atualmente aplica para toda a loja)
- [ ] **CI/CD:** pipeline de testes, lint e deploy automatizado
- [ ] **Remover endpoint `/api/wipe`** antes de produção

---

## 🐳 Docker

### Multi-stage Build

O `Dockerfile` usa dois estágios para minimizar a imagem final:

1. **Builder:** Rust 1.88 com compilação otimizada
2. **Runtime:** Debian slim com apenas o binário e migrações

### Deploy

Deploy via **Dokploy** com Docker. A imagem é construída e publicada no registry interno.

---

## 📄 Licença

MIT
