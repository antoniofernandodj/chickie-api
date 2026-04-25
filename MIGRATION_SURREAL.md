# Plano de Migração — PostgreSQL → SurrealDB

> Documento de planejamento para migração do banco de dados do projeto Chickie de PostgreSQL (sqlx) para SurrealDB (SDK Rust oficial). Nenhuma alteração de código foi feita ainda.

---

## Sumário

1. [Visão Geral](#1-visão-geral)
2. [Análise do Estado Atual](#2-análise-do-estado-atual)
3. [Decisões de Design para SurrealDB](#3-decisões-de-design-para-surrealdb)
4. [Schema SurrealDB por Tabela](#4-schema-surrealdb-por-tabela)
5. [Tabela `pedidos` — SCHEMALESS](#5-tabela-pedidos--schemaless)
6. [Mudanças nas Dependências Rust](#6-mudanças-nas-dependências-rust)
7. [Refatoração das Camadas](#7-refatoração-das-camadas)
8. [Estratégia de Migração de Dados](#8-estratégia-de-migração-de-dados)
9. [Fases de Execução](#9-fases-de-execução)
10. [Riscos e Mitigações](#10-riscos-e-mitigações)

---

## 1. Visão Geral

### Objetivo

Migrar o banco de dados do Chickie de PostgreSQL + sqlx para SurrealDB, utilizando o SDK oficial Rust do SurrealDB. Todas as tabelas devem ser **SCHEMAFULL** (schema rígido), exceto `pedidos`, que será **SCHEMALESS** para acomodar o JSONB complexo dos itens de pedido.

### Motivações

| Aspecto | PostgreSQL atual | SurrealDB proposto |
|---------|----------------|--------------------|
| Modelo de dados | Relacional + JSONB híbrido | Multi-model nativo (grafo + documento) |
| Schema | DDL SQL separado (migrations) | Schema definido em SurrealQL embutido |
| Pedidos | JSONB na coluna `itens` | Documento schemaless nativo |
| IDs de registros | UUIDs externos explícitos | `RecordId` nativo auto-gerado pelo DB |
| Relacionamentos | FKs UUID explícitas | `record<table>` — referências tipadas |
| Arrays | `Vec<i32>` via sqlx | `array<int>` nativo |

### Escopo

- **24 tabelas** a migrar
- **23 repositórios** a reescrever
- **8 serviços** a adaptar
- **5 usecases** a adaptar
- **97 endpoints** a verificar (handlers permanecem inalterados na lógica)

---

## 2. Análise do Estado Atual

### Tabelas Existentes (PostgreSQL)

| Tabela | Tipo atual | Linhas estimadas (dev) | Observações |
|--------|-----------|----------------------|-------------|
| `usuarios` | Relacional | baixo | Soft-delete, enum classe |
| `lojas` | Relacional | baixo | Soft-delete, arrays |
| `clientes` | Relacional (join) | baixo | Relação loja ↔ usuário |
| `lojas_favoritas` | Relacional (join) | baixo | Relação inversa |
| `categorias_produtos` | Relacional | baixo | `loja_uuid` nullable (global) |
| `ordem_categorias_de_produtos` | Relacional | baixo | Tabela auxiliar de ordenação |
| `produtos` | Relacional | baixo | |
| `adicionais` | Relacional | baixo | |
| `ingredientes` | Relacional | baixo | |
| `entregadores` | Relacional | baixo | FK para usuarios |
| `funcionarios` | Relacional | baixo | FK para usuarios |
| `horarios_funcionamento` | Relacional | baixo | NaiveTime como string |
| `configuracoes_pedidos_loja` | Relacional | baixo | Enum tipo_calculo |
| `pedidos` | Relacional + JSONB | médio | **SCHEMALESS alvo** |
| `enderecos_entrega` | Relacional | médio | |
| `enderecos_loja` | Relacional | baixo | |
| `enderecos_usuario` | Relacional | baixo | |
| `avaliacoes_loja` | Relacional | baixo | |
| `avaliacoes_produto` | Relacional | baixo | |
| `cupons` | Relacional | baixo | Enum status |
| `uso_cupons` | Relacional | baixo | |
| `promocoes` | Relacional | baixo | Array dias_semana, enum escopo |

### Tipos Especiais a Converter

| Tipo PostgreSQL | Tipo SurrealDB | Rust |
|----------------|----------------|------|
| `UUID` (PK) | ID nativo auto-gerado | `surrealdb::RecordId` |
| `TIMESTAMPTZ` | `datetime` | `chrono::DateTime<Utc>` |
| `NUMERIC` / `DECIMAL` | `decimal` | `rust_decimal::Decimal` |
| `TIME` (NaiveTime) | `string` (`HH:MM`) | `String` |
| `DATE` (NaiveDate) | `string` (`YYYY-MM-DD`) | `String` |
| `BOOLEAN` | `bool` | `bool` |
| `INTEGER` | `int` | `i64` |
| `TEXT` | `string` | `String` |
| `JSONB` | objeto/array nativo | `serde_json::Value` |
| `int[]` / `Vec<i32>` | `array<int>` | `Vec<i64>` |
| `VARCHAR(n)` | `string` | `String` |

### Enums PostgreSQL → SurrealDB

SurrealDB não tem enum nativo. A estratégia é usar `string` com `ASSERT` de validação:

```sql
DEFINE FIELD classe ON TABLE usuarios TYPE string
  ASSERT $value IN ["cliente", "administrador", "funcionario", "entregador", "owner"];
```

---

## 3. Decisões de Design para SurrealDB

### 3.1 Estratégia de IDs

**Decisão: usar IDs nativos do SurrealDB — auto-gerados no `CREATE`, expostos como `RecordId` nos models.**

SurrealDB gera automaticamente um ID aleatório ao criar um registro sem especificar `id`. No SDK Rust v2, esse ID é representado pelo tipo `surrealdb::RecordId`, que serializa para JSON como a string `"tabela:id_gerado"`.

Não há campo `uuid` separado. O `id` nativo é o único identificador.

```sql
-- SurrealDB gera automaticamente, ex: usuarios:2h5k7m9pqr...
-- Não é necessário DEFINE FIELD id (é implícito e gerenciado pelo DB)
DEFINE TABLE usuarios SCHEMAFULL;
```

No Rust, cada model expõe apenas:

```rust
pub id: RecordId,  // ex: serializa como "usuarios:2h5k7m9pqr..."
```

Os handlers recebem o ID como `String` no path param e constroem o `RecordId` no repositório:

```rust
// Handler:
Path(id): Path<String>

// Repositório:
let record_id = RecordId::from(("usuarios", id.as_str()));
```

### 3.2 Referências entre Tabelas

Substituir `loja_uuid: Uuid` por `loja_id: record<lojas>` no schema SurrealDB. No Rust, o campo é `loja_id: RecordId` nos models.

```sql
-- Schema SurrealDB
DEFINE FIELD loja_id ON TABLE produtos TYPE record<lojas>;

-- Query com graph traversal
SELECT *, loja_id.nome AS loja_nome FROM produtos;
```

### 3.3 Schema vs Schemaless

| Tabela | Tipo | Justificativa |
|--------|------|--------------|
| Todas exceto pedidos | `SCHEMAFULL` | Estrutura fixa e conhecida |
| `pedidos` | `SCHEMALESS` | Itens são JSONB com estrutura variável (partes, adicionais aninhados) |

### 3.4 Substituição do `Repository<T>` Trait

O trait genérico atual usa `sqlx::FromRow` e `PgPool`. Será substituído por um novo trait usando `surrealdb::Surreal` e `serde`:

```rust
pub trait Repository<T>: Send + Sync
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    fn table_name(&self) -> &'static str;
    fn entity_name(&self) -> &'static str;
    fn db(&self) -> &Surreal<impl Connection>;
    // métodos assíncronos...
}
```

### 3.5 Migrations

SurrealDB possui ferramenta nativa de migrations via CLI (`surreal migrate`), funcionando de forma similar ao `sqlx migrate`:

```bash
# Criar nova migration
surreal migrate create <nome_da_migration>

# Aplicar migrations pendentes
surreal migrate apply --endpoint ws://localhost:8000 \
  --username root --password root \
  --namespace chickie --database production
```

Cada migration é um arquivo `.surql` com seções `-- migration:up` e `-- migration:down`, versionadas em ordem:

```
migrations/
├── 20260421_000000_initial.surql
├── 20260422_000000_add_ordem_categorias.surql
└── ...
```

Exemplo de arquivo de migration:

```sql
-- migration:up
DEFINE TABLE usuarios SCHEMAFULL;
DEFINE FIELD nome ON TABLE usuarios TYPE string;
-- ...

-- migration:down
REMOVE TABLE usuarios;
```

---

## 4. Schema SurrealDB por Tabela

### 4.1 `usuarios` — SCHEMAFULL

```sql
DEFINE TABLE usuarios SCHEMAFULL;

DEFINE FIELD nome                     ON TABLE usuarios TYPE string;
DEFINE FIELD username                 ON TABLE usuarios TYPE string;
DEFINE FIELD email                    ON TABLE usuarios TYPE string;
DEFINE FIELD celular                  ON TABLE usuarios TYPE option<string>;
DEFINE FIELD senha_hash               ON TABLE usuarios TYPE string;
DEFINE FIELD classe                   ON TABLE usuarios TYPE string
  ASSERT $value IN ["cliente", "administrador", "funcionario", "entregador", "owner"];
DEFINE FIELD ativo                    ON TABLE usuarios TYPE bool DEFAULT true;
DEFINE FIELD bloqueado                ON TABLE usuarios TYPE bool DEFAULT false;
DEFINE FIELD passou_pelo_primeiro_acesso ON TABLE usuarios TYPE bool DEFAULT false;
DEFINE FIELD modo_de_cadastro         ON TABLE usuarios TYPE string;
DEFINE FIELD marcado_para_remocao     ON TABLE usuarios TYPE option<datetime>;
DEFINE FIELD deletado                 ON TABLE usuarios TYPE bool DEFAULT false;
DEFINE FIELD criado_em                ON TABLE usuarios TYPE datetime DEFAULT time::now();
DEFINE FIELD atualizado_em            ON TABLE usuarios TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_usuarios_email    ON TABLE usuarios COLUMNS email UNIQUE;
DEFINE INDEX idx_usuarios_username ON TABLE usuarios COLUMNS username UNIQUE;
DEFINE INDEX idx_usuarios_celular  ON TABLE usuarios COLUMNS celular UNIQUE;
```

### 4.2 `lojas` — SCHEMAFULL

```sql
DEFINE TABLE lojas SCHEMAFULL;

DEFINE FIELD nome                 ON TABLE lojas TYPE string;
DEFINE FIELD slug                 ON TABLE lojas TYPE string;
DEFINE FIELD descricao            ON TABLE lojas TYPE option<string>;
DEFINE FIELD email                ON TABLE lojas TYPE string;
DEFINE FIELD celular              ON TABLE lojas TYPE option<string>;
DEFINE FIELD ativa                ON TABLE lojas TYPE bool DEFAULT true;
DEFINE FIELD bloqueado            ON TABLE lojas TYPE bool DEFAULT false;
DEFINE FIELD logo_url             ON TABLE lojas TYPE option<string>;
DEFINE FIELD banner_url           ON TABLE lojas TYPE option<string>;
DEFINE FIELD horario_abertura     ON TABLE lojas TYPE option<string>;
DEFINE FIELD horario_fechamento   ON TABLE lojas TYPE option<string>;
DEFINE FIELD dias_funcionamento   ON TABLE lojas TYPE option<array<int>>;
DEFINE FIELD tempo_preparo_min    ON TABLE lojas TYPE option<int>;
DEFINE FIELD taxa_entrega         ON TABLE lojas TYPE option<decimal>;
DEFINE FIELD valor_minimo_pedido  ON TABLE lojas TYPE option<decimal>;
DEFINE FIELD raio_entrega_km      ON TABLE lojas TYPE option<decimal>;
DEFINE FIELD criado_por           ON TABLE lojas TYPE option<record<usuarios>>;
DEFINE FIELD marcado_para_remocao ON TABLE lojas TYPE option<datetime>;
DEFINE FIELD deletado             ON TABLE lojas TYPE bool DEFAULT false;
DEFINE FIELD criado_em            ON TABLE lojas TYPE datetime DEFAULT time::now();
DEFINE FIELD atualizado_em        ON TABLE lojas TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_lojas_slug ON TABLE lojas COLUMNS slug UNIQUE;
```

### 4.3 `clientes` — SCHEMAFULL

```sql
DEFINE TABLE clientes SCHEMAFULL;

DEFINE FIELD usuario_id ON TABLE clientes TYPE record<usuarios>;
DEFINE FIELD loja_id    ON TABLE clientes TYPE record<lojas>;
DEFINE FIELD criado_em  ON TABLE clientes TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_clientes_unique ON TABLE clientes COLUMNS usuario_id, loja_id UNIQUE;
```

### 4.4 `lojas_favoritas` — SCHEMAFULL

```sql
DEFINE TABLE lojas_favoritas SCHEMAFULL;

DEFINE FIELD usuario_id ON TABLE lojas_favoritas TYPE record<usuarios>;
DEFINE FIELD loja_id    ON TABLE lojas_favoritas TYPE record<lojas>;
DEFINE FIELD criado_em  ON TABLE lojas_favoritas TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_lojas_favoritas_unique ON TABLE lojas_favoritas COLUMNS usuario_id, loja_id UNIQUE;
```

### 4.5 `categorias_produtos` — SCHEMAFULL

```sql
DEFINE TABLE categorias_produtos SCHEMAFULL;

DEFINE FIELD loja_id    ON TABLE categorias_produtos TYPE option<record<lojas>>;
DEFINE FIELD nome       ON TABLE categorias_produtos TYPE string;
DEFINE FIELD descricao  ON TABLE categorias_produtos TYPE option<string>;
DEFINE FIELD pizza_mode ON TABLE categorias_produtos TYPE bool DEFAULT false;
DEFINE FIELD drink_mode ON TABLE categorias_produtos TYPE bool DEFAULT false;
DEFINE FIELD criado_em  ON TABLE categorias_produtos TYPE datetime DEFAULT time::now();
```

> `loja_id` nullable = categoria global (sem loja associada).

### 4.6 `ordem_categorias_de_produtos` — SCHEMAFULL

```sql
DEFINE TABLE ordem_categorias_de_produtos SCHEMAFULL;

DEFINE FIELD loja_id      ON TABLE ordem_categorias_de_produtos TYPE record<lojas>;
DEFINE FIELD categoria_id ON TABLE ordem_categorias_de_produtos TYPE record<categorias_produtos>;
DEFINE FIELD ordem        ON TABLE ordem_categorias_de_produtos TYPE int;
DEFINE FIELD criado_em    ON TABLE ordem_categorias_de_produtos TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_ordem_categorias_unique
  ON TABLE ordem_categorias_de_produtos COLUMNS loja_id, categoria_id UNIQUE;
```

### 4.7 `produtos` — SCHEMAFULL

```sql
DEFINE TABLE produtos SCHEMAFULL;

DEFINE FIELD loja_id          ON TABLE produtos TYPE record<lojas>;
DEFINE FIELD categoria_id     ON TABLE produtos TYPE record<categorias_produtos>;
DEFINE FIELD nome             ON TABLE produtos TYPE string;
DEFINE FIELD descricao        ON TABLE produtos TYPE option<string>;
DEFINE FIELD preco            ON TABLE produtos TYPE decimal;
DEFINE FIELD imagem_url       ON TABLE produtos TYPE option<string>;
DEFINE FIELD disponivel       ON TABLE produtos TYPE bool DEFAULT true;
DEFINE FIELD tempo_preparo_min ON TABLE produtos TYPE option<int>;
DEFINE FIELD destaque         ON TABLE produtos TYPE bool DEFAULT false;
DEFINE FIELD criado_em        ON TABLE produtos TYPE datetime DEFAULT time::now();
DEFINE FIELD atualizado_em    ON TABLE produtos TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_produtos_loja ON TABLE produtos COLUMNS loja_id;
```

### 4.8 `adicionais` — SCHEMAFULL

```sql
DEFINE TABLE adicionais SCHEMAFULL;

DEFINE FIELD loja_id    ON TABLE adicionais TYPE record<lojas>;
DEFINE FIELD nome       ON TABLE adicionais TYPE string;
DEFINE FIELD descricao  ON TABLE adicionais TYPE string;
DEFINE FIELD preco      ON TABLE adicionais TYPE decimal;
DEFINE FIELD disponivel ON TABLE adicionais TYPE bool DEFAULT true;
DEFINE FIELD criado_em  ON TABLE adicionais TYPE datetime DEFAULT time::now();
```

### 4.9 `ingredientes` — SCHEMAFULL

```sql
DEFINE TABLE ingredientes SCHEMAFULL;

DEFINE FIELD loja_id         ON TABLE ingredientes TYPE record<lojas>;
DEFINE FIELD nome            ON TABLE ingredientes TYPE string;
DEFINE FIELD unidade_medida  ON TABLE ingredientes TYPE option<string>;
DEFINE FIELD quantidade      ON TABLE ingredientes TYPE decimal;
DEFINE FIELD preco_unitario  ON TABLE ingredientes TYPE decimal;
DEFINE FIELD criado_em       ON TABLE ingredientes TYPE datetime DEFAULT time::now();
DEFINE FIELD atualizado_em   ON TABLE ingredientes TYPE datetime DEFAULT time::now();
```

### 4.10 `entregadores` — SCHEMAFULL

```sql
DEFINE TABLE entregadores SCHEMAFULL;

DEFINE FIELD loja_id     ON TABLE entregadores TYPE record<lojas>;
DEFINE FIELD usuario_id  ON TABLE entregadores TYPE record<usuarios>;
DEFINE FIELD veiculo     ON TABLE entregadores TYPE option<string>;
DEFINE FIELD placa       ON TABLE entregadores TYPE option<string>;
DEFINE FIELD disponivel  ON TABLE entregadores TYPE bool DEFAULT true;
DEFINE FIELD criado_em   ON TABLE entregadores TYPE datetime DEFAULT time::now();
```

### 4.11 `funcionarios` — SCHEMAFULL

```sql
DEFINE TABLE funcionarios SCHEMAFULL;

DEFINE FIELD loja_id        ON TABLE funcionarios TYPE record<lojas>;
DEFINE FIELD usuario_id     ON TABLE funcionarios TYPE record<usuarios>;
DEFINE FIELD cargo          ON TABLE funcionarios TYPE option<string>;
DEFINE FIELD salario        ON TABLE funcionarios TYPE option<decimal>;
DEFINE FIELD data_admissao  ON TABLE funcionarios TYPE string; -- "YYYY-MM-DD"
DEFINE FIELD criado_em      ON TABLE funcionarios TYPE datetime DEFAULT time::now();
```

### 4.12 `horarios_funcionamento` — SCHEMAFULL

```sql
DEFINE TABLE horarios_funcionamento SCHEMAFULL;

DEFINE FIELD loja_id     ON TABLE horarios_funcionamento TYPE record<lojas>;
DEFINE FIELD dia_semana  ON TABLE horarios_funcionamento TYPE int
  ASSERT $value >= 0 AND $value <= 6;
DEFINE FIELD abertura    ON TABLE horarios_funcionamento TYPE string; -- "HH:MM"
DEFINE FIELD fechamento  ON TABLE horarios_funcionamento TYPE string; -- "HH:MM"
DEFINE FIELD ativo       ON TABLE horarios_funcionamento TYPE bool DEFAULT true;
DEFINE FIELD criado_em   ON TABLE horarios_funcionamento TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_horarios_loja_dia
  ON TABLE horarios_funcionamento COLUMNS loja_id, dia_semana UNIQUE;
```

### 4.13 `configuracoes_pedidos_loja` — SCHEMAFULL

```sql
DEFINE TABLE configuracoes_pedidos_loja SCHEMAFULL;

DEFINE FIELD loja_id       ON TABLE configuracoes_pedidos_loja TYPE record<lojas>;
DEFINE FIELD max_partes    ON TABLE configuracoes_pedidos_loja TYPE int;
DEFINE FIELD tipo_calculo  ON TABLE configuracoes_pedidos_loja TYPE string
  ASSERT $value IN ["media_ponderada", "mais_caro"];
DEFINE FIELD criado_em     ON TABLE configuracoes_pedidos_loja TYPE datetime DEFAULT time::now();
DEFINE FIELD atualizado_em ON TABLE configuracoes_pedidos_loja TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_config_pedidos_loja ON TABLE configuracoes_pedidos_loja COLUMNS loja_id UNIQUE;
```

### 4.14 `enderecos_loja` — SCHEMAFULL

```sql
DEFINE TABLE enderecos_loja SCHEMAFULL;

DEFINE FIELD loja_id     ON TABLE enderecos_loja TYPE record<lojas>;
DEFINE FIELD cep         ON TABLE enderecos_loja TYPE option<string>;
DEFINE FIELD logradouro  ON TABLE enderecos_loja TYPE string;
DEFINE FIELD numero      ON TABLE enderecos_loja TYPE string;
DEFINE FIELD complemento ON TABLE enderecos_loja TYPE option<string>;
DEFINE FIELD bairro      ON TABLE enderecos_loja TYPE string;
DEFINE FIELD cidade      ON TABLE enderecos_loja TYPE string;
DEFINE FIELD estado      ON TABLE enderecos_loja TYPE string;
DEFINE FIELD latitude    ON TABLE enderecos_loja TYPE option<decimal>;
DEFINE FIELD longitude   ON TABLE enderecos_loja TYPE option<decimal>;
```

### 4.15 `enderecos_entrega` — SCHEMAFULL

```sql
DEFINE TABLE enderecos_entrega SCHEMAFULL;

DEFINE FIELD loja_id     ON TABLE enderecos_entrega TYPE record<lojas>;
DEFINE FIELD pedido_id   ON TABLE enderecos_entrega TYPE record<pedidos>;
DEFINE FIELD cep         ON TABLE enderecos_entrega TYPE option<string>;
DEFINE FIELD logradouro  ON TABLE enderecos_entrega TYPE string;
DEFINE FIELD numero      ON TABLE enderecos_entrega TYPE string;
DEFINE FIELD complemento ON TABLE enderecos_entrega TYPE option<string>;
DEFINE FIELD bairro      ON TABLE enderecos_entrega TYPE string;
DEFINE FIELD cidade      ON TABLE enderecos_entrega TYPE string;
DEFINE FIELD estado      ON TABLE enderecos_entrega TYPE string;
DEFINE FIELD latitude    ON TABLE enderecos_entrega TYPE option<decimal>;
DEFINE FIELD longitude   ON TABLE enderecos_entrega TYPE option<decimal>;
```

### 4.16 `enderecos_usuario` — SCHEMAFULL

```sql
DEFINE TABLE enderecos_usuario SCHEMAFULL;

DEFINE FIELD usuario_id  ON TABLE enderecos_usuario TYPE record<usuarios>;
DEFINE FIELD cep         ON TABLE enderecos_usuario TYPE option<string>;
DEFINE FIELD logradouro  ON TABLE enderecos_usuario TYPE string;
DEFINE FIELD numero      ON TABLE enderecos_usuario TYPE string;
DEFINE FIELD complemento ON TABLE enderecos_usuario TYPE option<string>;
DEFINE FIELD bairro      ON TABLE enderecos_usuario TYPE string;
DEFINE FIELD cidade      ON TABLE enderecos_usuario TYPE string;
DEFINE FIELD estado      ON TABLE enderecos_usuario TYPE string;
DEFINE FIELD latitude    ON TABLE enderecos_usuario TYPE option<decimal>;
DEFINE FIELD longitude   ON TABLE enderecos_usuario TYPE option<decimal>;
```

### 4.17 `avaliacoes_loja` — SCHEMAFULL

```sql
DEFINE TABLE avaliacoes_loja SCHEMAFULL;

DEFINE FIELD loja_id     ON TABLE avaliacoes_loja TYPE record<lojas>;
DEFINE FIELD usuario_id  ON TABLE avaliacoes_loja TYPE record<usuarios>;
DEFINE FIELD nota        ON TABLE avaliacoes_loja TYPE decimal
  ASSERT $value >= 0 AND $value <= 5;
DEFINE FIELD comentario  ON TABLE avaliacoes_loja TYPE option<string>;
DEFINE FIELD criado_em   ON TABLE avaliacoes_loja TYPE datetime DEFAULT time::now();
```

### 4.18 `avaliacoes_produto` — SCHEMAFULL

```sql
DEFINE TABLE avaliacoes_produto SCHEMAFULL;

DEFINE FIELD usuario_id  ON TABLE avaliacoes_produto TYPE record<usuarios>;
DEFINE FIELD loja_id     ON TABLE avaliacoes_produto TYPE record<lojas>;
DEFINE FIELD produto_id  ON TABLE avaliacoes_produto TYPE record<produtos>;
DEFINE FIELD nota        ON TABLE avaliacoes_produto TYPE decimal
  ASSERT $value >= 0 AND $value <= 5;
DEFINE FIELD descricao   ON TABLE avaliacoes_produto TYPE string;
DEFINE FIELD comentario  ON TABLE avaliacoes_produto TYPE option<string>;
DEFINE FIELD criado_em   ON TABLE avaliacoes_produto TYPE datetime DEFAULT time::now();
```

### 4.19 `cupons` — SCHEMAFULL

```sql
DEFINE TABLE cupons SCHEMAFULL;

DEFINE FIELD loja_id        ON TABLE cupons TYPE record<lojas>;
DEFINE FIELD codigo         ON TABLE cupons TYPE string;
DEFINE FIELD descricao      ON TABLE cupons TYPE option<string>;
DEFINE FIELD tipo_desconto  ON TABLE cupons TYPE string
  ASSERT $value IN ["percentual", "fixo"];
DEFINE FIELD valor_desconto ON TABLE cupons TYPE decimal;
DEFINE FIELD valor_minimo   ON TABLE cupons TYPE option<decimal>;
DEFINE FIELD data_validade  ON TABLE cupons TYPE option<datetime>;
DEFINE FIELD limite_uso     ON TABLE cupons TYPE option<int>;
DEFINE FIELD uso_atual      ON TABLE cupons TYPE int DEFAULT 0;
DEFINE FIELD status         ON TABLE cupons TYPE string
  ASSERT $value IN ["Ativo", "Inativo", "Expirado", "Esgotado"];
DEFINE FIELD criado_em      ON TABLE cupons TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_cupons_codigo ON TABLE cupons COLUMNS loja_id, codigo UNIQUE;
```

### 4.20 `uso_cupons` — SCHEMAFULL

```sql
DEFINE TABLE uso_cupons SCHEMAFULL;

DEFINE FIELD cupom_id        ON TABLE uso_cupons TYPE record<cupons>;
DEFINE FIELD usuario_id      ON TABLE uso_cupons TYPE option<record<usuarios>>;
DEFINE FIELD pedido_id       ON TABLE uso_cupons TYPE record<pedidos>;
DEFINE FIELD valor_desconto  ON TABLE uso_cupons TYPE decimal;
DEFINE FIELD usado_em        ON TABLE uso_cupons TYPE datetime DEFAULT time::now();
```

### 4.21 `promocoes` — SCHEMAFULL

```sql
DEFINE TABLE promocoes SCHEMAFULL;

DEFINE FIELD loja_id              ON TABLE promocoes TYPE record<lojas>;
DEFINE FIELD nome                 ON TABLE promocoes TYPE string;
DEFINE FIELD descricao            ON TABLE promocoes TYPE option<string>;
DEFINE FIELD tipo_desconto        ON TABLE promocoes TYPE string
  ASSERT $value IN ["percentual", "fixo"];
DEFINE FIELD valor_desconto       ON TABLE promocoes TYPE decimal;
DEFINE FIELD valor_minimo         ON TABLE promocoes TYPE option<decimal>;
DEFINE FIELD data_inicio          ON TABLE promocoes TYPE datetime;
DEFINE FIELD data_fim             ON TABLE promocoes TYPE option<datetime>;
DEFINE FIELD dias_semana_validos  ON TABLE promocoes TYPE option<array<int>>;
DEFINE FIELD tipo_escopo          ON TABLE promocoes TYPE string
  ASSERT $value IN ["loja", "produto", "categoria"];
DEFINE FIELD produto_id           ON TABLE promocoes TYPE option<record<produtos>>;
DEFINE FIELD categoria_id         ON TABLE promocoes TYPE option<record<categorias_produtos>>;
DEFINE FIELD status               ON TABLE promocoes TYPE string
  ASSERT $value IN ["Ativo", "Inativo", "Expirado"];
DEFINE FIELD prioridade           ON TABLE promocoes TYPE int DEFAULT 0;
DEFINE FIELD criado_em            ON TABLE promocoes TYPE datetime DEFAULT time::now();
```

---

## 5. Tabela `pedidos` — SCHEMALESS

A tabela `pedidos` é a única **SCHEMALESS**, pois seus itens têm estrutura aninhada variável (partes com adicionais por parte). No PostgreSQL atual já está armazenada como JSONB após a migration 0006.

```sql
DEFINE TABLE pedidos SCHEMALESS;
```

### Estrutura esperada de um documento `pedidos`

```json
{
  "id": "pedidos:2h5k7m9pqr...",
  "codigo": "A1B2C3",
  "usuario_id": "usuarios:3j6l8n0qst...",
  "loja_id": "lojas:4k7m9p1ruv...",
  "entregador_id": null,
  "status": "criado",
  "total": "65.90",
  "subtotal": "60.90",
  "taxa_entrega": "5.00",
  "desconto": "0.00",
  "forma_pagamento": "PIX",
  "observacoes": null,
  "tempo_estimado_min": 45,
  "criado_em": "2026-04-23T00:00:00Z",
  "atualizado_em": "2026-04-23T00:00:00Z",
  "itens": [
    {
      "id": "item:5l8n0q2svw...",
      "quantidade": 1,
      "observacoes": null,
      "partes": [
        {
          "id": "parte:6m9p1r3twx...",
          "produto_id": "produtos:7n0q2s4uxy...",
          "produto_nome": "Pizza Grande",
          "preco_unitario": "49.90",
          "posicao": 1,
          "adicionais": [
            {
              "id": "adicionais:8o1r3t5vyz...",
              "nome": "Queijo Extra",
              "descricao": "Mussarela adicional",
              "preco": "3.50"
            }
          ]
        }
      ]
    }
  ]
}
```

### Queries de Pedido (SurrealQL)

```sql
-- Criar pedido
CREATE pedidos CONTENT {
  codigo: "A1B2C3",
  loja_id: lojas:uuid,
  status: "criado",
  itens: [...],
  -- ...
};

-- Listar por loja
SELECT * FROM pedidos WHERE loja_id = lojas:uuid ORDER BY criado_em DESC;

-- Buscar por codigo
SELECT * FROM pedidos WHERE codigo = "A1B2C3";

-- Buscar por usuario
SELECT * FROM pedidos WHERE usuario_id = usuarios:uuid ORDER BY criado_em DESC;
```

---

## 6. Mudanças nas Dependências Rust

### Cargo.toml — workspace (diff conceitual)

```toml
# REMOVER
sqlx = { version = "0.8.6", features = [...] }

# ADICIONAR
surrealdb = { version = "2", features = ["kv-rocksdb"] }
# ou para conexão remota:
surrealdb = "2"
```

### Dependências a manter

Todas as outras permanecem: `serde`, `serde_json`, `uuid`, `chrono`, `rust_decimal`, `axum`, `tokio`, `tracing`, `jsonwebtoken`, etc.

### Remoção de derives sqlx dos models

Cada model atualmente possui derives como:
- `sqlx::FromRow` — **remover**
- `sqlx::Type` — **remover** (usado nos enums)
- `sqlx::Encode` / `sqlx::Decode` — **remover**

Adicionar:
- `serde::Serialize` / `serde::Deserialize` — já presentes na maioria
- Implementações manuais de conversão `Thing ↔ Uuid` onde necessário

---

## 7. Refatoração das Camadas

### 7.1 Conexão — `crates/core/src/infrastructure/database.rs`

**Atual:**
```rust
pub async fn criar_pool(database_url: &str) -> PgPool {
    PgPool::connect(database_url).await.unwrap()
}
```

**Novo:**
```rust
use surrealdb::{Surreal, engine::remote::ws::Ws};
use surrealdb::opt::auth::Root;

pub async fn criar_conexao(url: &str, ns: &str, db: &str) -> Surreal<surrealdb::engine::remote::ws::Client> {
    let client = Surreal::new::<Ws>(url).await.unwrap();
    client.signin(Root { username: "root", password: "root" }).await.unwrap();
    client.use_ns(ns).use_db(db).await.unwrap();
    client
}
```

**Variáveis de ambiente necessárias:**

| Variável | Descrição | Exemplo |
|----------|-----------|---------|
| `SURREAL_URL` | Endereço WebSocket do SurrealDB | `ws://localhost:8000` |
| `SURREAL_NS` | Namespace | `chickie` |
| `SURREAL_DB` | Database | `production` |
| `SURREAL_USER` | Usuário root | `root` |
| `SURREAL_PASS` | Senha root | `senha_segura` |

### 7.2 AppState — `src/api/state.rs`

**Atual:** recebe `Arc<PgPool>`

**Novo:** recebe `Arc<Surreal<Client>>` onde `Client` é o tipo de conexão WebSocket.

```rust
pub struct AppState {
    pub db: Arc<Surreal<surrealdb::engine::remote::ws::Client>>,
    // services continuam iguais na interface pública
    pub usuario_service: UsuarioService,
    // ...
}
```

### 7.3 Trait `Repository<T>` — Novo design

```rust
use surrealdb::{Surreal, Connection, RecordId};
use serde::{Serialize, de::DeserializeOwned};

#[async_trait]
pub trait Repository<T>: Send + Sync
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn table_name(&self) -> &'static str;
    fn entity_name(&self) -> &'static str;
    fn db(&self) -> &Surreal<impl Connection>;

    async fn buscar_por_id(&self, id: &str) -> Result<Option<T>, String> {
        let record_id = RecordId::from((self.table_name(), id));
        let result: Option<T> = self.db()
            .select(record_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result)
    }

    async fn listar_todos(&self) -> Result<Vec<T>, String> {
        let result: Vec<T> = self.db()
            .select(self.table_name())
            .await
            .map_err(|e| e.to_string())?;
        Ok(result)
    }

    async fn deletar(&self, id: &str) -> Result<(), String> {
        let record_id = RecordId::from((self.table_name(), id));
        let _: Option<T> = self.db()
            .delete(record_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // criar retorna o RecordId gerado pelo SurrealDB
    async fn criar(&self, item: impl Serialize + Send) -> Result<RecordId, String>;
    async fn atualizar(&self, id: &str, item: impl Serialize + Send) -> Result<(), String>;
    async fn listar_todos_por_loja(&self, loja_id: &str) -> Result<Vec<T>, String>;
}
```

### 7.4 Exemplo de Repositório — `usuario_repository.rs`

```rust
use surrealdb::RecordId;

pub struct UsuarioRepository {
    db: Arc<Surreal<Client>>,
}

impl UsuarioRepository {
    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Usuario>, String> {
        let mut result = self.db
            .query("SELECT * FROM usuarios WHERE email = $email LIMIT 1")
            .bind(("email", email))
            .await
            .map_err(|e| e.to_string())?;
        let usuario: Option<Usuario> = result.take(0).map_err(|e| e.to_string())?;
        Ok(usuario)
    }
}

#[async_trait]
impl Repository<Usuario> for UsuarioRepository {
    fn table_name(&self) -> &'static str { "usuarios" }
    fn entity_name(&self) -> &'static str { "Usuário" }
    fn db(&self) -> &Surreal<impl Connection> { &self.db }

    async fn criar(&self, novo: impl Serialize + Send) -> Result<RecordId, String> {
        // SurrealDB gera o ID automaticamente
        let created: Option<Usuario> = self.db
            .create("usuarios")
            .content(novo)
            .await
            .map_err(|e| e.to_string())?;
        let record = created.ok_or_else(|| "Falha ao criar usuário".to_string())?;
        Ok(record.id)
    }

    async fn atualizar(&self, id: &str, dados: impl Serialize + Send) -> Result<(), String> {
        let record_id = RecordId::from(("usuarios", id));
        let _: Option<Usuario> = self.db
            .update(record_id)
            .merge(dados)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn listar_todos_por_loja(&self, _loja_id: &str) -> Result<Vec<Usuario>, String> {
        Err("Usuários não pertencem a lojas".to_string())
    }
}
```

### 7.5 Repositório de Pedidos — Schemaless

O repositório de pedidos usa queries SurrealQL manuais para lidar com a estrutura schemaless. O ID é auto-gerado pelo SurrealDB:

```rust
use surrealdb::RecordId;

impl PedidoRepository {
    pub async fn criar_pedido(&self, pedido: impl Serialize + Send) -> Result<RecordId, String> {
        // SurrealDB gera o ID automaticamente
        let created: Option<serde_json::Value> = self.db
            .create("pedidos")
            .content(pedido)
            .await
            .map_err(|e| e.to_string())?;
        let record = created.ok_or_else(|| "Falha ao criar pedido".to_string())?;
        let id: RecordId = serde_json::from_value(record["id"].clone())
            .map_err(|e| e.to_string())?;
        Ok(id)
    }

    pub async fn buscar_completo(&self, id: &str) -> Result<Option<Pedido>, String> {
        let record_id = RecordId::from(("pedidos", id));
        let result: Option<Pedido> = self.db
            .select(record_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result)
    }

    pub async fn buscar_por_codigo(&self, codigo: &str) -> Result<Option<Pedido>, String> {
        let mut result = self.db
            .query("SELECT * FROM pedidos WHERE codigo = $codigo LIMIT 1")
            .bind(("codigo", codigo))
            .await
            .map_err(|e| e.to_string())?;
        let pedido: Option<Pedido> = result.take(0).map_err(|e| e.to_string())?;
        Ok(pedido)
    }

    pub async fn buscar_completos_por_loja(&self, loja_id: &str) -> Result<Vec<Pedido>, String> {
        let record_id = RecordId::from(("lojas", loja_id));
        let mut result = self.db
            .query("SELECT * FROM pedidos WHERE loja_id = $loja_id ORDER BY criado_em DESC")
            .bind(("loja_id", record_id))
            .await
            .map_err(|e| e.to_string())?;
        let pedidos: Vec<Pedido> = result.take(0).map_err(|e| e.to_string())?;
        Ok(pedidos)
    }
}
```

### 7.6 Models — Ajustes necessários

#### Remover de todos os models:
- `#[derive(sqlx::FromRow)]`
- `#[sqlx(rename_all = "snake_case")]`
- `#[sqlx(type_name = "...")]`
- Campo `uuid: Uuid` (substituído por `id: RecordId`)

#### Manter / Adicionar:
- `#[derive(Serialize, Deserialize, Clone, Debug)]`
- Nos enums: `#[serde(rename_all = "snake_case")]`
- Campo `id: RecordId` (ID nativo do SurrealDB, auto-gerado)

#### Campo `id` nos models:

```rust
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usuario {
    pub id: RecordId,  // auto-gerado pelo SurrealDB, serializa como "usuarios:abc123..."
    pub nome: String,
    pub username: String,
    pub email: String,
    // sem campo uuid separado
    // ...
}
```

`RecordId` serializa para JSON como a string `"usuarios:2h5k7m9pqr..."`, portanto os responses da API passarão a ter `"id": "usuarios:..."` em vez de `"uuid": "550e8400-..."`.

#### Impacto nos Handlers (app não foi ao ar — alterações permitidas):

| Antes | Depois |
|-------|--------|
| `Path(uuid): Path<Uuid>` | `Path(id): Path<String>` |
| `usuario.uuid` | `usuario.id` (RecordId) |
| Response: `"uuid": "550e8400-..."` | Response: `"id": "usuarios:abc123..."` |
| `buscar_por_uuid(uuid)` | `buscar_por_id(&id)` |

Referências entre entidades nos models Rust também usam `RecordId`:

```rust
pub struct Produto {
    pub id: RecordId,
    pub loja_id: RecordId,       // antes: loja_uuid: Uuid
    pub categoria_id: RecordId,  // antes: categoria_uuid: Uuid
    // ...
}
```

---

## 8. Estratégia de Migração de Dados

### 8.1 Script de Migração (ambiente de produção futura)

Criar um binário separado em `crates/migrator/`:

```
crates/
└── migrator/
    ├── Cargo.toml
    └── src/
        ├── main.rs      -- orchestração
        ├── pg_reader.rs -- lê do PostgreSQL (sqlx)
        └── surreal_writer.rs -- escreve no SurrealDB
```

### 8.2 Ordem de Migração (respeitar FKs)

A ordem deve respeitar dependências entre tabelas:

```
1. usuarios
2. lojas
3. clientes
4. lojas_favoritas
5. categorias_produtos
6. ordem_categorias_de_produtos
7. produtos
8. adicionais
9. ingredientes
10. entregadores
11. funcionarios
12. horarios_funcionamento
13. configuracoes_pedidos_loja
14. enderecos_loja
15. enderecos_usuario
16. cupons
17. pedidos           ← schemaless, inclui itens embutidos
18. uso_cupons
19. enderecos_entrega
20. avaliacoes_loja
21. avaliacoes_produto
22. promocoes
```

### 8.3 Estratégia para `pedidos`

O JSONB atual da coluna `itens` em PostgreSQL já está no formato aninhado. O migrator deve:

1. Ler cada linha de `pedidos` com `itens_json`
2. Fazer parse do JSON
3. Construir o documento SurrealDB com `itens` embutidos (sem as tabelas relacionais separadas)

### 8.4 Script de Schema Inicial

Criar a primeira migration com o CLI do SurrealDB:

```bash
surreal migrate create initial_schema
```

O arquivo gerado (ex: `migrations/20260421_000000_initial_schema.surql`) receberá todas as definições de tabela da seção 4. Aplicar com:

```bash
surreal migrate apply --endpoint ws://localhost:8000 \
  --username root --password root \
  --namespace chickie --database production
```

Não há necessidade de aplicar schema manualmente no boot do Rust — as migrations são gerenciadas pelo CLI, igual ao fluxo do `sqlx migrate`.

---

## 9. Fases de Execução

### Fase 1 — Preparação (sem quebrar nada)

- [ ] Adicionar `surrealdb` ao `Cargo.toml` (sem remover `sqlx`)
- [ ] Criar `crates/core/src/infrastructure/surreal.rs` com função de conexão
- [ ] Subir instância SurrealDB local para desenvolvimento (Docker)
- [ ] Criar migration inicial via `surreal migrate create initial_schema`
- [ ] Preencher o arquivo gerado com o schema da seção 4
- [ ] Validar aplicando com `surreal migrate apply` contra o SurrealDB local

**Docker para dev:**
```yaml
surreal:
  image: surrealdb/surrealdb:latest
  command: start --log trace --user root --pass root memory
  ports:
    - "8000:8000"
```

### Fase 2 — Modelos

- [ ] Remover derives `sqlx::FromRow`, `sqlx::Type`, `sqlx::Encode`, `sqlx::Decode` de todos os models
- [ ] Garantir `Serialize + Deserialize` em todos os models e enums
- [ ] Adicionar campo `id: Option<Thing>` nos models onde necessário
- [ ] Ajustar enums para usar `#[serde(rename_all = "snake_case")]` em vez de sqlx

### Fase 3 — Repositórios (um por vez)

Reescrever cada repositório em ordem de menor dependência:

- [ ] `usuario_repository.rs`
- [ ] `loja_repository.rs`
- [ ] `produto_repository.rs`
- [ ] `categoria_produtos_repository.rs`
- [ ] `adicional_repository.rs`
- [ ] `ingrediente_repository.rs`
- [ ] `entregador_repository.rs`
- [ ] `funcionario_repository.rs`
- [ ] `cliente_repository.rs`
- [ ] `loja_favorita_repository.rs`
- [ ] `horario_funcionamento_repository.rs`
- [ ] `configuracao_pedidos_loja_repository.rs`
- [ ] `endereco_loja_repository.rs`
- [ ] `endereco_usuario_repository.rs`
- [ ] `endereco_entrega_repository.rs`
- [ ] `cupom_repository.rs`
- [ ] `uso_cupom_repository.rs`
- [ ] `pedido_repository.rs` (schemaless — maior cuidado)
- [ ] `avaliacao_de_loja_repository.rs`
- [ ] `avaliacao_de_produto_repository.rs`
- [ ] `promocao_repository.rs`
- [ ] `categoria_ordem_repository.rs`
- [ ] `parte_de_item_pedido_repository.rs`

### Fase 4 — AppState e Bootstrap

- [ ] Substituir `PgPool` por `Surreal<Client>` no `AppState`
- [ ] Atualizar `main.rs` para conectar ao SurrealDB e aplicar schema
- [ ] Remover `database.rs` PostgreSQL
- [ ] Atualizar variáveis de ambiente

### Fase 5 — Serviços e Usecases

Os serviços e usecases provavelmente não precisam de mudanças (interface dos repositórios mantida). Verificar:
- [ ] `usuario_service.rs`
- [ ] `loja_service.rs`
- [ ] `catalogo_service.rs`
- [ ] `pedido_service.rs`
- [ ] `marketing_service.rs`
- [ ] `endereco_entrega_service.rs`
- [ ] `endereco_usuario_service.rs`
- [ ] `loja_favorita_service.rs`

### Fase 6 — Migrador de Dados

- [ ] Criar `crates/migrator/` com binário independente
- [ ] Implementar leitura do PostgreSQL (mantendo sqlx temporariamente neste crate)
- [ ] Implementar escrita no SurrealDB
- [ ] Testar com dados de dev
- [ ] Executar migração completa em staging

### Fase 7 — Limpeza

- [ ] Remover `sqlx` do `Cargo.toml` (exceto do `crates/migrator`)
- [ ] Remover arquivos de migration PostgreSQL
- [ ] Remover `DATABASE_URL` das variáveis de ambiente
- [ ] Atualizar `CLAUDE.md`, `API.md`, `README.md` e `pendencias.md`
- [ ] Atualizar `docker-compose.yml` para incluir SurrealDB

---

## 10. Riscos e Mitigações

| Risco | Impacto | Probabilidade | Mitigação |
|-------|---------|--------------|-----------|
| SurrealDB SDK instável / breaking changes | Alto | Médio | Fixar versão `surrealdb = "2.x.x"` exata |
| Performance de queries complexas (ex: JOIN equivalente) | Médio | Médio | Testar com `SELECT *, field.* FROM tabela` (graph traversal) |
| Transações distribuídas (ex: criar pedido + uso_cupom) | Alto | Alto | Usar `BEGIN TRANSACTION / COMMIT` do SurrealDB |
| Perda de dados na migração | Crítico | Baixo | Manter PostgreSQL em paralelo durante migração; só desligar após validação |
| Compatibilidade de `Decimal` com SurrealDB SDK | Médio | Médio | Serializar como `String` se necessário e converter na camada de serviço |
| Serialização de `RecordId` na API | Médio | Baixo | `RecordId` serializa como `"table:id"` — clientes devem enviar apenas a parte `id` no path param (sem prefixo de tabela) |
| Queries N+1 sem FK enforcement | Médio | Médio | Usar `FETCH` do SurrealDB para eager loading |

### Queries SurrealDB equivalentes a JOINs PostgreSQL

```sql
-- PostgreSQL (JOIN):
SELECT al.*, u.nome AS usuario_nome FROM avaliacoes_loja al
JOIN usuarios u ON u.uuid = al.usuario_uuid
WHERE al.loja_uuid = $1;

-- SurrealDB equivalente:
SELECT *, usuario_id.nome AS usuario_nome
FROM avaliacoes_loja
WHERE loja_id = $loja_id;
```

### Transações em SurrealDB

```rust
// Criar pedido com uso de cupom em transação
db.query("
    BEGIN TRANSACTION;
    CREATE pedidos:$uuid CONTENT $pedido;
    CREATE uso_cupons:$uso_uuid CONTENT $uso;
    UPDATE cupons:$cupom_uuid SET uso_atual += 1;
    COMMIT TRANSACTION;
")
.bind(("uuid", pedido_uuid))
.bind(("pedido", pedido_content))
// ...
.await?;
```

---

## Apêndice — Variáveis de Ambiente Finais

| Variável | Atual (PostgreSQL) | Novo (SurrealDB) |
|----------|-------------------|-----------------|
| `DATABASE_URL` | Obrigatório | **Remover** |
| `SURREAL_URL` | — | `ws://localhost:8000` |
| `SURREAL_NS` | — | `chickie` |
| `SURREAL_DB` | — | `production` |
| `SURREAL_USER` | — | `root` |
| `SURREAL_PASS` | — | `senha_segura` |
| `APP_PORT` | Mantido | Mantido |
| `RUST_LOG` | Mantido | Mantido |
| `JWT_SECRET` | Mantido | Mantido |
| `MODE` | Mantido (ajustar lógica de wipe) | Mantido |

---

*Documento criado em 2026-04-23. Nenhuma alteração de código foi realizada.*
