# API Specification — Chickie

> Especificação completa de todos os endpoints, request/response bodies, headers e autenticação.

---

## Interactive API Documentation (Swagger/OpenAPI)

This API provides interactive documentation powered by **utoipa** with automatic OpenAPI specification generation.

### Quick Access

| Resource | URL |
|----------|-----|
| **Swagger UI** | `http://localhost:3000/api/docs/swagger-ui` |
| **OpenAPI JSON Spec** | `http://localhost:3000/api/docs/openapi.json` |

### Features

- **Interactive Testing**: Execute API requests directly from the browser
- **Complete Schema Documentation**: View all request/response models
- **Authentication Support**: Configure JWT tokens for testing protected endpoints
- **Real-time Validation**: See parameter requirements and data constraints

### Using Swagger UI

1. Navigate to `http://localhost:3000/api/docs/swagger-ui`
2. Browse endpoints organized by tags
3. Click on any endpoint to expand details
4. Click **"Try it out"** to enable request testing
5. Fill in required parameters
6. Click **"Execute"** to send the request
7. View response body and headers

> **Note**: For protected endpoints (🔒), authenticate first by clicking the **Authorize** button and entering your JWT token as `Bearer <token>`.

---

## Informações Gerais

| Item | Valor |
|------|-------|
| **Base URL** | `http://localhost:3000/api` |
| **Protocolo** | HTTP/1.1 |
| **Formato** | JSON (`application/json`) |
| **Autenticação** | JWT Bearer Token via header `Authorization: Bearer <token>` |
| **Charset** | UTF-8 |

### Autenticação

Endpoints marcados com 🔒 exigem:
```
Authorization: Bearer <JWT_TOKEN>
```

Endpoints marcados com 👑 exigem além do JWT que o usuário tenha `classe = "administrador"`.

### Classes de Usuário

| Classe | Descrição |
|--------|-----------|
| `cliente` | Padrão. Faz pedidos e avalia. |
| `administrador` | Cria e gerencia lojas, funcionários e entregadores. |
| `funcionario` | Funcionário de uma loja (vinculado via `usuario_uuid`). |
| `entregador` | Entregador de uma loja (vinculado via `usuario_uuid`). |
| `owner` | Dono da plataforma. Acesso total. |

### Erros

Todos os endpoints retornam erros no formato:
```json
{
  "error": "Mensagem de erro descritiva"
}
```

| Status | Significado |
|--------|-------------|
| `400` | Bad Request — dados inválidos ou transição de estado inválida |
| `403` | Forbidden — sem permissão (admin necessário) |
| `404` | Not Found — recurso não encontrado |
| `500` | Internal Server Error — erro interno |
| `201` | Created — recurso criado com sucesso |
| `204` | No Content — operação bem-sucedida sem body |

---

## Máquina de Estados do Pedido

Um pedido passa por uma máquina de estados com transições válidas definidas:

```
Criado
  ↓
AguardandoConfirmacaoDeLoja ←────┐
  ↓                               │
ConfirmadoPelaLoja ───────────────┘
  ↓
EmPreparo ────────────────────────┐
  ↓                               │
ProntoParaRetirada ←──────────────┘
  ↓
SaiuParaEntrega ──────────────────┐
  ↓                               │
Entregue (terminal) ←─────────────┘
```

### Estados e Transições

| Estado Atual | Próximo Válido |
|--------------|----------------|
| `criado` | `aguardando_confirmacao_de_loja` |
| `aguardando_confirmacao_de_loja` | `confirmado_pela_loja` ou voltar para `criado` |
| `confirmado_pela_loja` | `em_preparo` ou voltar para `aguardando_confirmacao_de_loja` |
| `em_preparo` | `pronto_para_retirada` ou voltar para `confirmado_pela_loja` |
| `pronto_para_retirada` | `saiu_para_entrega` ou voltar para `em_preparo` |
| `saiu_para_entrega` | `entregue` ou voltar para `pronto_para_retirada` |
| `entregue` | **(estado terminal — sem transições)** |

### Response ao Avançar Estado

```json
{
  "uuid": "pedido-uuid",
  "status": "em_preparo",
  "transicoes_permitidas": ["pronto_para_retirada", "confirmado_pela_loja"]
}
```

---

## Prefixos de Rota

| Prefixo | Grupo |
|---------|-------|
| `/api/auth/` | Autenticação |
| `/api/usuarios/` | Usuários |
| `/api/lojas/` | Lojas (público) |
| `/api/admin/` | Administração |
| `/api/pedidos/` | Pedidos |
| `/api/marketing/` | Cupons, avaliações, promoções |
| `/api/catalogo/` | Adicionais, categorias |
| `/api/enderecos-entrega/` | Endereços de entrega |
| `/api/enderecos-usuario/` | Endereços de usuário |
| `/api/favoritos/` | Lojas favoritas |
| `/api/produtos/` | Produtos |

---

## 1. Health Check

```
GET /
```

**Response `200`:**
```json
{
  "message": "🚀 Servidor compilado com sucesso!"
}
```

---

## 2. Autenticação (público)

### 2.1 Cadastrar Usuário

```
POST /api/auth/signup
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "string",
  "username": "string",
  "senha": "string",
  "email": "string",
  "telefone": "string",
  "auth_method": "string",
  "classe": "cliente" | "administrador" | "funcionario" | "entregador" | "owner"
}
```

> `classe` é opcional. Default: `"cliente"`.

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "João Silva",
  "username": "joao",
  "email": "joao@email.com",
  "celular": "11999999999",
  "telefone": "11888888888",
  "classe": "cliente",
  "ativo": true,
  "passou_pelo_primeiro_acesso": false,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z",
  "modo_de_cadastro": "email",
  "senha_hash": "$2b$12$..."
}
```

---

### 2.2 Login

```
POST /api/auth/login
Content-Type: application/json
```

**Request Body:**
```json
{
  "email": "string",
  "senha": "string"
}
```

**Response `200`:**
```json
{
  "access_token": "string (JWT)",
  "token_type": "Bearer"
}
```

### 2.3 Me (Obter Usuário Autenticado)

```
GET /api/auth/me
Authorization: Bearer <token>
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "João Silva",
  "username": "joao",
  "email": "joao@email.com",
  "celular": "11999999999",
  "telefone": "11888888888",
  "classe": "cliente",
  "ativo": true,
  "passou_pelo_primeiro_acesso": false,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z",
  "modo_de_cadastro": "email"
}
```

**Response `401`:**
```json
{
  "error": "Token de autenticação não fornecido. Inclua o header: Authorization: Bearer <token>"
}
```

---

## 3. Lojas (público)

### 3.1 Listar Lojas

```
GET /api/lojas/
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "Pizzaria do João",
    "slug": "pizzaria-do-joao",
    "descricao": "A melhor pizza da cidade",
    "email": "contato@pizzaria.com",
    "telefone": "11999999999",
    "ativa": true,
    "logo_url": "https://example.com/logo.png",
    "banner_url": "https://example.com/banner.jpg",
    "horario_abertura": "18:00",
    "horario_fechamento": "23:00",
    "dias_funcionamento": "2,3,4,5,6,0",
    "tempo_preparo_min": 45,
    "taxa_entrega": 5.0,
    "valor_minimo_pedido": 20.0,
    "raio_entrega_km": 5.0,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z"
  }
]
```

### 3.2 Pesquisar Lojas

```
GET /api/lojas/pesquisar?termo={valor}
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "Pizzaria do João",
    "slug": "pizzaria-do-joao",
    "descricao": "A melhor pizza da cidade",
    "email": "contato@pizzaria.com",
    "telefone": "11999999999",
    "ativa": true,
    "logo_url": "https://example.com/logo.png",
    "banner_url": "https://example.com/banner.jpg",
    "horario_abertura": "18:00",
    "horario_fechamento": "23:00",
    "dias_funcionamento": "2,3,4,5,6,0",
    "tempo_preparo_min": 45,
    "taxa_entrega": 5.0,
    "valor_minimo_pedido": 20.0,
    "raio_entrega_km": 5.0,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 3.3 Buscar Loja

```
GET /api/lojas/{uuid}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "Pizzaria do João",
  "slug": "pizzaria-do-joao",
  "descricao": "A melhor pizza da cidade",
  "email": "contato@pizzaria.com",
  "telefone": "11999999999",
  "ativa": true,
  "logo_url": "https://example.com/logo.png",
  "banner_url": "https://example.com/banner.jpg",
  "horario_abertura": "18:00",
  "horario_fechamento": "23:00",
  "dias_funcionamento": "2,3,4,5,6,0",
  "tempo_preparo_min": 45,
  "taxa_entrega": 5.0,
  "valor_minimo_pedido": 20.0,
  "raio_entrega_km": 5.0,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z"
}
```

---

### 3.4 Buscar Loja por Slug

```
GET /api/lojas/slug/{slug}
```

**Response `200`:** (mesmo schema de buscar por UUID)

---

### 3.5 Verificar Disponibilidade de Slug

```
GET /api/lojas/verificar-slug/{slug}
```

**Response `200` (slug disponível):**
```json
{
  "disponivel": true,
  "slug": "pizzaria-do-joao"
}
```

**Response `200` (slug já em uso):**
```json
{
  "disponivel": false,
  "slug": "pizzaria-do-joao"
}
```

---

## 4. Usuários (🔒)

### 4.1 Listar Usuários

```
GET /api/usuarios/
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "João Silva",
    "username": "joao",
    "email": "joao@email.com",
    "celular": "11999999999",
    "telefone": "11888888888",
    "classe": "cliente",
    "ativo": true,
    "passou_pelo_primeiro_acesso": true,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z",
    "modo_de_cadastro": "email",
    "senha_hash": "$2b$12$..."
  }
]
```

---

## 5. Administração (🔒 + 👑 Admin)

### 5.1 Criar Loja

```
POST /api/admin/lojas
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "string",
  "slug": "string",
  "email_contato": "string",
  "descricao": "string | null",
  "telefone": "string | null",
  "hora_abertura": "string | null",
  "hora_fechamento": "string | null",
  "dias_funcionamento": "string | null",
  "tempo_medio": 30,
  "nota_media": 4.5,
  "taxa_entrega_base": 5.0,
  "pedido_minimo": 20.0,
  "max_partes": 4
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "Pizzaria do João",
  "slug": "pizzaria-do-joao",
  "descricao": "A melhor pizza da cidade",
  "email": "contato@pizzaria.com",
  "telefone": "11999999999",
  "ativa": true,
  "logo_url": "https://example.com/logo.png",
  "banner_url": "https://example.com/banner.jpg",
  "horario_abertura": "18:00",
  "horario_fechamento": "23:00",
  "dias_funcionamento": "2,3,4,5,6,0",
  "tempo_preparo_min": 45,
  "taxa_entrega": 5.0,
  "valor_minimo_pedido": 20.0,
  "raio_entrega_km": 5.0,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z"
}
```

---

### 5.2 Listar Todas as Lojas

```
GET /api/admin/lojas/listar
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "Pizzaria do João",
    "slug": "pizzaria-do-joao",
    "descricao": "A melhor pizza da cidade",
    "email": "contato@pizzaria.com",
    "telefone": "11999999999",
    "ativa": true,
    "logo_url": "https://example.com/logo.png",
    "banner_url": "https://example.com/banner.jpg",
    "horario_abertura": "18:00",
    "horario_fechamento": "23:00",
    "dias_funcionamento": "2,3,4,5,6,0",
    "tempo_preparo_min": 45,
    "taxa_entrega": 5.0,
    "valor_minimo_pedido": 20.0,
    "raio_entrega_km": 5.0,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 5.3 Minhas Lojas (Lojas do Admin)

```
GET /api/admin/lojas/minhas-lojas
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "Pizzaria do João",
    "slug": "pizzaria-do-joao"
  }
]
```

---

### 5.4 Adicionar Funcionário

```
POST /api/admin/lojas/{loja_uuid}/funcionarios
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "string",
  "username": "string",
  "email": "string",
  "senha": "string",
  "celular": "string",
  "cargo": "string | null",
  "salario": 2500.0,
  "data_admissao": "2026-04-04"
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440001",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440002",
  "cargo": "Gerente",
  "salario": 2500.0,
  "data_admissao": "2026-04-04",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 5.5 Adicionar Entregador

```
POST /api/admin/lojas/{loja_uuid}/entregadores
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "string",
  "username": "string",
  "email": "string",
  "senha": "string",
  "celular": "string",
  "veiculo": "string | null",
  "placa": "string | null"
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440003",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440004",
  "veiculo": "Moto",
  "placa": "ABC-1234",
  "disponivel": true,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 5.6 Adicionar Cliente

```
POST /api/admin/lojas/{loja_uuid}/clientes
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "string",
  "username": "string",
  "email": "string",
  "senha": "string",
  "celular": "string"
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440005",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440006",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

## 6. Pedidos (🔒)

### 6.1 Criar Pedido

```
POST /api/pedidos/criar
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "taxa_entrega": 5.0,
  "forma_pagamento": "PIX",
  "observacoes": "string | null",
  "codigo_cupom": "string | null",
  "itens": [
    {
      "quantidade": 1,
      "observacoes": "string | null",
      "partes": [
        {
          "produto_uuid": "uuid",
          "posicao": 1
        }
      ]
    }
  ],
  "endereco_entrega": {
    "cep": "string | null",
    "logradouro": "string",
    "numero": "string",
    "complemento": "string | null",
    "bairro": "string",
    "cidade": "string",
    "estado": "string"
  }
}
```

**Response `201`:**
```json
{ "uuid": "uuid" }
```

---

### 6.2 Listar Pedidos

```
GET /api/pedidos/
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440010",
    "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "entregador_uuid": null,
    "status": "criado",
    "total": 65.90,
    "subtotal": 55.90,
    "taxa_entrega": 5.0,
    "desconto": 0.0,
    "forma_pagamento": "PIX",
    "observacoes": "Sem cebola",
    "tempo_estimado_min": 45,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z",
    "itens": [
      {
        "uuid": "550e8400-e29b-41d4-a716-446655440011",
        "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
        "pedido_uuid": "550e8400-e29b-41d4-a716-446655440010",
        "quantidade": 1,
        "observacoes": null,
        "adicionais": [],
        "partes": [
          {
            "uuid": "550e8400-e29b-41d4-a716-446655440012",
            "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
            "item_uuid": "550e8400-e29b-41d4-a716-446655440011",
            "produto_nome": "Pizza Grande",
            "produto_uuid": "550e8400-e29b-41d4-a716-446655440013",
            "preco_unitario": 49.90,
            "posicao": 1,
            "adicionais": [
              {
                "uuid": "550e8400-e29b-41d4-a716-446655440014",
                "item_uuid": "550e8400-e29b-41d4-a716-446655440011",
                "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
                "nome": "Queijo Extra",
                "descricao": "Queijo mussarela adicional",
                "preco": 3.50
              }
            ]
          }
        ]
      }
    ],
    "partes": []
  }
]
```

---

### 6.2.1 Listar Meus Pedidos (Por Usuário)

```
GET /api/pedidos/meus
Authorization: Bearer <token>
```

> Retorna todos os pedidos criados pelo usuário autenticado, ordenados do mais recente ao mais antigo. Inclui itens, partes e adicionais hidratados.

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440010",
    "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "entregador_uuid": null,
    "status": "criado",
    "total": 65.90,
    "subtotal": 55.90,
    "taxa_entrega": 5.0,
    "desconto": 0.0,
    "forma_pagamento": "PIX",
    "observacoes": "Sem cebola",
    "tempo_estimado_min": 45,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z",
    "itens": [
      {
        "uuid": "550e8400-e29b-41d4-a716-446655440011",
        "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
        "pedido_uuid": "550e8400-e29b-41d4-a716-446655440010",
        "quantidade": 1,
        "observacoes": null,
        "adicionais": [],
        "partes": [
          {
            "uuid": "550e8400-e29b-41d4-a716-446655440012",
            "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
            "item_uuid": "550e8400-e29b-41d4-a716-446655440011",
            "produto_nome": "Pizza Grande",
            "produto_uuid": "550e8400-e29b-41d4-a716-446655440013",
            "preco_unitario": 49.90,
            "posicao": 1,
            "adicionais": []
          }
        ]
      }
    ],
    "partes": []
  }
]
```

**Response `200` (vazio — sem pedidos):**
```json
[]
```

---

### 6.3 Buscar Pedido

```
GET /api/pedidos/{uuid}
Authorization: Bearer <token>
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440010",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "entregador_uuid": null,
  "status": "criado",
  "total": 65.90,
  "subtotal": 55.90,
  "taxa_entrega": 5.0,
  "desconto": 0.0,
  "forma_pagamento": "PIX",
  "observacoes": "Sem cebola",
  "tempo_estimado_min": 45,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z",
  "itens": [
    {
      "uuid": "550e8400-e29b-41d4-a716-446655440011",
      "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
      "pedido_uuid": "550e8400-e29b-41d4-a716-446655440010",
      "quantidade": 1,
      "observacoes": null,
      "adicionais": [],
      "partes": [
        {
          "uuid": "550e8400-e29b-41d4-a716-446655440012",
          "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
          "item_uuid": "550e8400-e29b-41d4-a716-446655440011",
          "produto_nome": "Pizza Grande",
          "produto_uuid": "550e8400-e29b-41d4-a716-446655440013",
          "preco_unitario": 49.90,
          "posicao": 1,
          "adicionais": [
            {
              "uuid": "550e8400-e29b-41d4-a716-446655440014",
              "item_uuid": "550e8400-e29b-41d4-a716-446655440011",
              "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
              "nome": "Queijo Extra",
              "descricao": "Queijo mussarela adicional",
              "preco": 3.50
            }
          ]
        }
      ]
    }
  ],
  "partes": []
}
```

---

### 6.4 Listar Pedidos por Loja

```
GET /api/pedidos/criar
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440010",
    "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "entregador_uuid": null,
    "status": "criado",
    "total": 65.90,
    "subtotal": 55.90,
    "taxa_entrega": 5.0,
    "desconto": 0.0,
    "forma_pagamento": "PIX",
    "observacoes": "Sem cebola",
    "tempo_estimado_min": 45,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z",
    "itens": [
      {
        "uuid": "550e8400-e29b-41d4-a716-446655440011",
        "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
        "pedido_uuid": "550e8400-e29b-41d4-a716-446655440010",
        "quantidade": 1,
        "observacoes": null,
        "adicionais": [],
        "partes": [
          {
            "uuid": "550e8400-e29b-41d4-a716-446655440012",
            "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
            "item_uuid": "550e8400-e29b-41d4-a716-446655440011",
            "produto_nome": "Pizza Grande",
            "produto_uuid": "550e8400-e29b-41d4-a716-446655440013",
            "preco_unitario": 49.90,
            "posicao": 1,
            "adicionais": []
          }
        ]
      }
    ],
    "partes": []
  }
]
```

---

### 6.5 Buscar Pedido com Endereço de Entrega

```
GET /api/pedidos/{pedido_uuid}/com-entrega
Authorization: Bearer <token>
```

**Response `200`:**
```json
{
  "pedido": {
    "uuid": "550e8400-e29b-41d4-a716-446655440010",
    "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "entregador_uuid": null,
    "status": "criado",
    "total": 65.90,
    "subtotal": 55.90,
    "taxa_entrega": 5.0,
    "desconto": 0.0,
    "forma_pagamento": "PIX",
    "observacoes": null,
    "tempo_estimado_min": 45,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z",
    "itens": [],
    "partes": []
  },
  "endereco_entrega": {
    "uuid": "550e8400-e29b-41d4-a716-446655440020",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "pedido_uuid": "550e8400-e29b-41d4-a716-446655440010",
    "cep": "01001-000",
    "logradouro": "Rua das Flores",
    "numero": "123",
    "complemento": "Apto 101",
    "bairro": "Centro",
    "cidade": "São Paulo",
    "estado": "SP",
    "latitude": -23.5505,
    "longitude": -46.6333
  }
}
```

---

### 6.6 Atualizar Status do Pedido

```
PUT /api/pedidos/criar/{pedido_uuid}/status
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "novo_status": "em_preparo"
}
```

**Valores válidos para `novo_status`:**
- `criado`
- `aguardando_confirmacao_de_loja`
- `confirmado_pela_loja`
- `em_preparo`
- `pronto_para_retirada`
- `saiu_para_entrega`
- `entregue`

> A transição deve ser válida conforme a máquina de estados. Transições inválidas retornam `400 Bad Request`.

**Response `200`:**
```json
{
  "uuid": "pedido-uuid",
  "status": "em_preparo",
  "transicoes_permitidas": ["pronto_para_retirada", "confirmado_pela_loja"]
}
```

**Response `400` (transição inválida):**
```json
{
  "error": "Transição inválida: Criado -> Entregue. Transições permitidas: [AguardandoConfirmacaoDeLoja]"
}
```

---

### 6.7 Atribuir Entregador ao Pedido

```
PUT /api/pedidos/{pedido_uuid}/entregador/{loja_uuid}
Authorization: Bearer <token>
Content-Type: application/json
```

> Vincula um entregador existente ao pedido. O entregador deve pertencer à mesma loja do pedido.

**Request Body:**
```json
{
  "entregador_uuid": "550e8400-e29b-41d4-a716-446655440099"
}
```

**Response `204`:** No Content

**Response `404` (pedido ou entregador não encontrado):**
```json
{
  "error": "Pedido não encontrado ou não pertence a esta loja"
}
```

---

### 6.8 Remover Entregador do Pedido

```
DELETE /api/pedidos/{pedido_uuid}/entregador/{loja_uuid}
Authorization: Bearer <token>
```

> Remove o entregador vinculado ao pedido (define `entregador_uuid` como NULL).

**Response `204`:** No Content

**Response `404` (pedido não encontrado):**
```json
{
  "error": "Pedido não encontrado ou não pertence a esta loja"
}
```

---

### 6.9 Buscar Pedido com Informações do Entregador

```
GET /api/pedidos/{pedido_uuid}/com-entregador
Authorization: Bearer <token>
```

> Retorna o pedido completo com dados do entregador vinculado (nome, veículo, placa). Se nenhum entregador estiver atribuído, os campos retornam vazios e `entregador_uuid` é `null`.

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440010",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "entregador_uuid": "550e8400-e29b-41d4-a716-446655440099",
  "status": "saiu_para_entrega",
  "total": 65.90,
  "subtotal": 55.90,
  "taxa_entrega": 5.0,
  "desconto": 0.0,
  "forma_pagamento": "PIX",
  "observacoes": "Sem cebola",
  "tempo_estimado_min": 45,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z",
  "entregador_nome": "Carlos Entregador",
  "veiculo": "Moto",
  "placa": "ABC-1234"
}
```

**Response `404`:**
```json
{
  "error": "Pedido não encontrado"
}
```

---

## 7. Marketing: Cupons, Avaliações e Promoções

### 7.1 Criar Cupom

```
POST /api/marketing/{loja_uuid}/cupons
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "codigo": "PROMO10",
  "descricao": "10% off",
  "tipo_desconto": "percentual",
  "valor_desconto": 10.0,
  "valor_minimo": 50.0,
  "data_validade": "2026-12-31T23:59:59Z",
  "limite_uso": 100
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440030",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "codigo": "PROMO10",
  "descricao": "10% off",
  "tipo_desconto": "percentual",
  "valor_desconto": 10.0,
  "valor_minimo": 50.0,
  "data_validade": "2026-12-31T23:59:59Z",
  "limite_uso": 100,
  "uso_atual": 0,
  "status": "Ativo",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 7.2 Listar Cupons

```
GET /api/marketing/cupons
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440030",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "codigo": "PROMO10",
    "descricao": "10% off",
    "tipo_desconto": "percentual",
    "valor_desconto": 10.0,
    "valor_minimo": 50.0,
    "data_validade": "2026-12-31T23:59:59Z",
    "limite_uso": 100,
    "uso_atual": 0,
    "status": "Ativo",
    "criado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 7.3 Validar Cupom

```
GET /api/marketing/cupons/{codigo}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440030",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "codigo": "PROMO10",
  "descricao": "10% off",
  "tipo_desconto": "percentual",
  "valor_desconto": 10.0,
  "valor_minimo": 50.0,
  "data_validade": "2026-12-31T23:59:59Z",
  "limite_uso": 100,
  "uso_atual": 0,
  "status": "Ativo",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 7.4 Avaliar Loja

```
POST /api/marketing/{loja_uuid}/avaliar-loja
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nota": 4.5,
  "comentario": "string | null"
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440040",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nota": 4.5,
  "comentario": "Ótima pizza!",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 7.5 Avaliar Produto

```
POST /api/marketing/{loja_uuid}/avaliar-produto
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "produto_uuid": "uuid",
  "nota": 5.0,
  "descricao": "string",
  "comentario": "string | null"
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440041",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "produto_uuid": "550e8400-e29b-41d4-a716-446655440013",
  "nota": 5.0,
  "descricao": "Pizza deliciosa!",
  "comentario": "Melhor pizza que já comi",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 7.6 Criar Promoção

```
POST /api/marketing/{loja_uuid}/promocoes
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "Promoção Pizza",
  "descricao": "string",
  "tipo_desconto": "percentual",
  "valor_desconto": 20.0,
  "valor_minimo": null,
  "data_inicio": "2026-04-04T00:00:00Z",
  "data_fim": "2026-04-04T23:59:59Z",
  "dias_semana_validos": [5],
  "tipo_escopo": "produto",
  "produto_uuid": "uuid-do-produto",
  "categoria_uuid": null,
  "prioridade": 1
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440050",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "Promoção Pizza",
  "descricao": "Desconto especial em pizzas",
  "tipo_desconto": "percentual",
  "valor_desconto": 20.0,
  "valor_minimo": null,
  "data_inicio": "2026-04-04T00:00:00Z",
  "data_fim": "2026-04-04T23:59:59Z",
  "dias_semana_validos": [5],
  "tipo_escopo": "produto",
  "produto_uuid": "550e8400-e29b-41d4-a716-446655440013",
  "categoria_uuid": null,
  "status": "Ativo",
  "prioridade": 1,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 7.7 Listar Promoções

```
GET /api/marketing/{loja_uuid}/promocoes
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440050",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "Promoção Pizza",
    "descricao": "Desconto especial em pizzas",
    "tipo_desconto": "percentual",
    "valor_desconto": 20.0,
    "valor_minimo": null,
    "data_inicio": "2026-04-04T00:00:00Z",
    "data_fim": "2026-04-04T23:59:59Z",
    "dias_semana_validos": [5],
    "tipo_escopo": "produto",
    "produto_uuid": "550e8400-e29b-41d4-a716-446655440013",
    "categoria_uuid": null,
    "status": "Ativo",
    "prioridade": 1,
    "criado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 7.8 Atualizar Promoção

```
PUT /api/marketing/{loja_uuid}/promocoes/{uuid}
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:** (mesmo schema de criar)

**Response `204`:** No Content

---

### 7.9 Deletar Promoção

```
DELETE /api/marketing/{loja_uuid}/promocoes/{uuid}
Authorization: Bearer <token>
```

**Response `204`:** No Content

---

## 8. Catálogo (🔒)

### 8.1 Criar Adicional

```
POST /api/catalogo/{loja_uuid}/adicionais
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "Queijo Extra",
  "descricao": "string",
  "preco": 3.50
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440060",
  "nome": "Queijo Extra",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "disponivel": true,
  "descricao": "Queijo mussarela adicional",
  "preco": 3.50,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 8.2 Listar Adicionais

```
GET /api/catalogo/{loja_uuid}/adicionais
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440060",
    "nome": "Queijo Extra",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "disponivel": true,
    "descricao": "Queijo mussarela adicional",
    "preco": 3.50,
    "criado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 8.3 Listar Adicionais Disponíveis

```
GET /api/catalogo/{loja_uuid}/adicionais/disponiveis
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440060",
    "nome": "Queijo Extra",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "disponivel": true,
    "descricao": "Queijo mussarela adicional",
    "preco": 3.50,
    "criado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 8.4 Atualizar Adicional

```
PUT /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "Queijo Extra Premium",
  "descricao": "Queijo mussarela especial importado",
  "preco": 5.00
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440060",
  "nome": "Queijo Extra Premium",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "disponivel": true,
  "descricao": "Queijo mussarela especial importado",
  "preco": 5.00,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

**Response `404` (adicional não encontrado):**
```json
{
  "error": "Adicional não encontrado"
}
```

**Response `400` (adicional não pertence à loja):**
```json
{
  "error": "Adicional não pertence a esta loja"
}
```

---

### 8.5 Atualizar Disponibilidade do Adicional

```
PUT /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/disponibilidade
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "disponivel": false
}
```

> Envie `"disponivel": true` para tornar o adicional disponível novamente.

**Response `204`:** No Content

**Response `404` (adicional não encontrado):**
```json
{
  "error": "Adicional não encontrado"
}
```

**Response `400` (adicional não pertence à loja):**
```json
{
  "error": "Adicional não pertence a esta loja"
}
```

---

### 8.6 Deletar Adicional

```
DELETE /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}
Authorization: Bearer <token>
```

**Response `204`:** No Content

**Response `404` (adicional não encontrado):**
```json
{
  "error": "Adicional não encontrado"
}
```

**Response `400` (adicional não pertence à loja):**
```json
{
  "error": "Adicional não pertence a esta loja"
}
```

---

### 8.7 Criar Categoria

```
POST /api/catalogo/{loja_uuid}/categorias
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "Bebidas",
  "descricao": "string | null",
  "ordem": 1,
  "pizza_mode": false
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440070",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "Bebidas",
  "descricao": "Bebidas geladas",
  "ordem": 1,
  "pizza_mode": false,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 8.8 Listar Categorias

```
GET /api/catalogo/{loja_uuid}/categorias
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440070",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "Bebidas",
    "descricao": "Bebidas geladas",
    "ordem": 1,
    "pizza_mode": false,
    "criado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 8.9 Atualizar Categoria

```
PUT /api/catalogo/{loja_uuid}/categorias/{uuid}
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "Bebidas Geladas",
  "descricao": "string | null",
  "ordem": 2,
  "pizza_mode": false
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440070",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "Bebidas Geladas",
  "descricao": "Bebidas bem geladas",
  "ordem": 2,
  "pizza_mode": false,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 8.10 Deletar Categoria

```
DELETE /api/catalogo/{loja_uuid}/categorias/{uuid}
Authorization: Bearer <token>
```

> ⚠️ Apenas funciona se a categoria não tiver produtos vinculados.

**Response `204`:** No Content

**Response `400` (categoria com produtos):**
```json
{
  "error": "Não é possível deletar categoria com 3 produto(s). Remova os produtos primeiro."
}
```

---

## 9. Endereços de Entrega (🔒)

### 9.1 Criar Endereço para Pedido

```
POST /api/enderecos-entrega/{pedido_uuid}/{loja_uuid}
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "cep": "string | null",
  "logradouro": "string",
  "numero": "string",
  "complemento": "string | null",
  "bairro": "string",
  "cidade": "string",
  "estado": "string"
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440080",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "pedido_uuid": "550e8400-e29b-41d4-a716-446655440010",
  "cep": "01001-000",
  "logradouro": "Rua das Flores",
  "numero": "123",
  "complemento": "Apto 101",
  "bairro": "Centro",
  "cidade": "São Paulo",
  "estado": "SP",
  "latitude": -23.5505,
  "longitude": -46.6333
}
```

---

### 9.2 Buscar Endereço por Pedido

```
GET /api/enderecos-entrega/{pedido_uuid}
Authorization: Bearer <token>
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440080",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "pedido_uuid": "550e8400-e29b-41d4-a716-446655440010",
  "cep": "01001-000",
  "logradouro": "Rua das Flores",
  "numero": "123",
  "complemento": "Apto 101",
  "bairro": "Centro",
  "cidade": "São Paulo",
  "estado": "SP",
  "latitude": -23.5505,
  "longitude": -46.6333
}
```

---

### 9.3 Listar Endereços por Loja

```
GET /api/enderecos-entrega/{loja_uuid}/loja
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440080",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "pedido_uuid": "550e8400-e29b-41d4-a716-446655440010",
    "cep": "01001-000",
    "logradouro": "Rua das Flores",
    "numero": "123",
    "complemento": "Apto 101",
    "bairro": "Centro",
    "cidade": "São Paulo",
    "estado": "SP",
    "latitude": -23.5505,
    "longitude": -46.6333
  }
]
```

---

## 10. Endereços de Usuário (🔒)

### 10.1 Criar Endereço

```
POST /api/enderecos-usuario/
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "cep": "string | null",
  "logradouro": "string",
  "numero": "string",
  "complemento": "string | null",
  "bairro": "string",
  "cidade": "string",
  "estado": "string"
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440090",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "cep": "01001-000",
  "logradouro": "Rua das Flores",
  "numero": "123",
  "complemento": "Apto 101",
  "bairro": "Centro",
  "cidade": "São Paulo",
  "estado": "SP",
  "latitude": -23.5505,
  "longitude": -46.6333
}
```

---

### 10.2 Listar Endereços

```
GET /api/enderecos-usuario/
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440090",
    "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "cep": "01001-000",
    "logradouro": "Rua das Flores",
    "numero": "123",
    "complemento": "Apto 101",
    "bairro": "Centro",
    "cidade": "São Paulo",
    "estado": "SP",
    "latitude": -23.5505,
    "longitude": -46.6333
  }
]
```

---

### 10.3 Buscar Endereço

```
GET /api/enderecos-usuario/{uuid}
Authorization: Bearer <token>
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440090",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "cep": "01001-000",
  "logradouro": "Rua das Flores",
  "numero": "123",
  "complemento": "Apto 101",
  "bairro": "Centro",
  "cidade": "São Paulo",
  "estado": "SP",
  "latitude": -23.5505,
  "longitude": -46.6333
}
```

---

### 10.4 Atualizar Endereço

```
PUT /api/enderecos-usuario/{uuid}
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "cep": "string | null",
  "logradouro": "string",
  "numero": "string",
  "complemento": "string | null",
  "bairro": "string",
  "cidade": "string",
  "estado": "string"
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440090",
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "cep": "01001-000",
  "logradouro": "Rua Nova",
  "numero": "456",
  "complemento": "Casa",
  "bairro": "Jardim",
  "cidade": "São Paulo",
  "estado": "SP",
  "latitude": -23.5505,
  "longitude": -46.6333
}
```

---

### 10.5 Deletar Endereço

```
DELETE /api/enderecos-usuario/{uuid}
Authorization: Bearer <token>
```

**Response `204`:** No Content

---

## 11. Lojas Favoritas (🔒)

### 11.1 Adicionar Favorita

```
POST /api/favoritos/{loja_uuid}
Authorization: Bearer <token>
```

**Response `200`:**
```json
{
  "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "uuid": "550e8400-e29b-41d4-a716-446655440100",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 11.2 Remover Favorita

```
DELETE /api/favoritos/{loja_uuid}
Authorization: Bearer <token>
```

**Response `200`:**
```json
{ "message": "Loja removida das favoritas" }
```

---

### 11.3 Listar Minhas Favoritas

```
GET /api/favoritos/minhas
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "usuario_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "uuid": "550e8400-e29b-41d4-a716-446655440100",
    "criado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 11.4 Verificar se é Favorita

```
GET /api/favoritos/{loja_uuid}/verificar
Authorization: Bearer <token>
```

**Response `200`:**
```json
{ "favorita": true }
```

---

## 12. Produtos (🔒)

### 12.1 Criar Produto

```
POST /api/produtos/
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "loja_uuid": "uuid",
  "categoria_uuid": "uuid",
  "nome": "Pizza Grande",
  "descricao": "string | null",
  "preco": 49.90,
  "imagem_url": "string | null",
  "disponivel": true,
  "tempo_preparo_min": 30,
  "destaque": false
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440013",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "categoria_uuid": "550e8400-e29b-41d4-a716-446655440070",
  "nome": "Pizza Grande",
  "descricao": "Pizza grande com até 4 sabores",
  "preco": 49.90,
  "imagem_url": "https://example.com/pizza.jpg",
  "disponivel": true,
  "tempo_preparo_min": 30,
  "destaque": false,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z"
}
```

---

### 12.2 Listar Produtos

```
GET /api/produtos/
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440013",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "categoria_uuid": "550e8400-e29b-41d4-a716-446655440070",
    "nome": "Pizza Grande",
    "descricao": "Pizza grande com até 4 sabores",
    "preco": 49.90,
    "imagem_url": "https://example.com/pizza.jpg",
    "disponivel": true,
    "tempo_preparo_min": 30,
    "destaque": false,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 12.3 Listar Produtos por Categoria

```
GET /api/produtos/categoria/{categoria_uuid}
Authorization: Bearer <token>
```

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440013",
    "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
    "categoria_uuid": "550e8400-e29b-41d4-a716-446655440070",
    "nome": "Pizza Grande",
    "descricao": "Pizza grande com até 4 sabores",
    "preco": 49.90,
    "imagem_url": "https://example.com/pizza.jpg",
    "disponivel": true,
    "tempo_preparo_min": 30,
    "destaque": false,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### 12.4 Buscar Produto por UUID

```
GET /api/produtos/{uuid}
Authorization: Bearer <token>
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440013",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "categoria_uuid": "550e8400-e29b-41d4-a716-446655440070",
  "nome": "Pizza Grande",
  "descricao": "Pizza grande com até 4 sabores",
  "preco": 49.90,
  "imagem_url": "https://example.com/pizza.jpg",
  "disponivel": true,
  "tempo_preparo_min": 30,
  "destaque": false,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z"
}
```

---

### 12.5 Atualizar Produto

```
PUT /api/produtos/{uuid}
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "string",
  "descricao": "string | null",
  "preco": 59.90,
  "categoria_uuid": "uuid",
  "tempo_preparo_min": 35
}
```

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440013",
  "loja_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "categoria_uuid": "550e8400-e29b-41d4-a716-446655440070",
  "nome": "Pizza Grande Especial",
  "descricao": "Pizza grande premium",
  "preco": 59.90,
  "imagem_url": "https://example.com/pizza.jpg",
  "disponivel": true,
  "tempo_preparo_min": 35,
  "destaque": true,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z"
}
```

---

### 12.6 Deletar Produto

```
DELETE /api/produtos/{uuid}
Authorization: Bearer <token>
```

**Response `204`:** No Content

---

### 12.7 Subir Imagem do Produto

```
POST /api/produtos/{uuid}/imagem
Authorization: Bearer <token>
Content-Type: multipart/form-data
```

**Request Body:** (multipart form-data)
- `file`: Image file (JPEG, PNG, WebP, etc.)

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440013",
  "imagem_url": "https://s3.example.com/bucket/produtos/550e8400-...-pizza.jpg",
  "message": "Imagem enviada com sucesso"
}
```

**Environment Variables Required:**
```bash
S3_BUCKET_NAME=your-bucket-name
S3_REGION=us-east-1                 # Optional, default: us-east-1
S3_ENDPOINT=https://s3.example.com  # Optional, for custom endpoints (rust-fs)
S3_ACCESS_KEY_ID=your-access-key    # Optional, for custom endpoints
S3_SECRET_ACCESS_KEY=your-secret-key # Optional, for custom endpoints
```

---

## 13. Utilitários

### 13.1 OK

```
GET /api/ok
```

**Response `200`:**
```json
{ "msg": "ok" }
```

---

### 13.2 Wipe Database (⚠️ Dev Only)

```
DELETE /api/wipe
```

> ⚠️ Apenas com `MODE=development`. Apaga todas as tabelas e reaplica migrações.

**Response `200`:**
```json
{
  "message": "Database wiped successfully",
  "warning": "⚠️ All data has been permanently deleted"
}
```

---

## Sumário Completo de Endpoints

| # | Método | Rota | Auth | Admin |
|---|--------|------|------|-------|
| 1 | `GET` | `/` | — | — |
| 2 | `POST` | `/api/auth/signup` | — | — |
| 3 | `POST` | `/api/auth/login` | — | — |
| 4 | `GET` | `/api/auth/me` | 🔒 | — |
| 5 | `GET` | `/api/lojas/` | — | — |
| 6 | `GET` | `/api/lojas/pesquisar` | — | — |
| 7 | `GET` | `/api/lojas/{uuid}` | — | — |
| 8 | `GET` | `/api/lojas/slug/{slug}` | — | — |
| 9 | `GET` | `/api/lojas/verificar-slug/{slug}` | — | — |
| 10 | `GET` | `/api/usuarios/` | 🔒 | — |
| 11 | `POST` | `/api/admin/lojas` | 🔒 | 👑 |
| 12 | `GET` | `/api/admin/lojas/listar` | 🔒 | — |
| 13 | `POST` | `/api/admin/lojas/{loja_uuid}/funcionarios` | 🔒 | 👑 |
| 14 | `POST` | `/api/admin/lojas/{loja_uuid}/entregadores` | 🔒 | 👑 |
| 15 | `POST` | `/api/admin/lojas/{loja_uuid}/clientes` | 🔒 | 👑 |
| 16 | `POST` | `/api/pedidos/criar` | 🔒 | — |
| 17 | `GET` | `/api/pedidos/listar` | 🔒 | — |
| 17.1 | `GET` | `/api/pedidos/meus` | 🔒 | — |
| 18 | `GET` | `/api/pedidos/por-loja/{loja_uuid}` | 🔒 | — |
| 19 | `GET` | `/api/pedidos/{uuid}` | 🔒 | — |
| 20 | `GET` | `/api/pedidos/{uuid}/com-entrega` | 🔒 | — |
| 21 | `PUT` | `/api/pedidos/{uuid}/status` | 🔒 | — |
| 22 | `PUT` | `/api/pedidos/{pedido_uuid}/entregador/{loja_uuid}` | 🔒 | — |
| 23 | `DELETE` | `/api/pedidos/{pedido_uuid}/entregador/{loja_uuid}` | 🔒 | — |
| 24 | `GET` | `/api/pedidos/{pedido_uuid}/com-entregador` | 🔒 | — |
| 25 | `POST` | `/api/marketing/{loja_uuid}/cupons` | 🔒 | — |
| 26 | `GET` | `/api/marketing/cupons/{codigo}` | — | — |
| 27 | `GET` | `/api/marketing/cupons` | 🔒 | — |
| 28 | `POST` | `/api/marketing/{loja_uuid}/avaliar-loja` | 🔒 | — |
| 29 | `POST` | `/api/marketing/{loja_uuid}/avaliar-produto` | 🔒 | — |
| 30 | `POST` | `/api/marketing/{loja_uuid}/promocoes` | 🔒 | — |
| 31 | `GET` | `/api/marketing/{loja_uuid}/promocoes` | 🔒 | — |
| 32 | `PUT` | `/api/marketing/{loja_uuid}/promocoes/{uuid}` | 🔒 | — |
| 33 | `DELETE` | `/api/marketing/{loja_uuid}/promocoes/{uuid}` | 🔒 | — |
| 34 | `POST` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — |
| 35 | `GET` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — |
| 36 | `GET` | `/api/catalogo/{loja_uuid}/adicionais/disponiveis` | 🔒 | — |
| 37 | `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}` | 🔒 | — |
| 38 | `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/disponibilidade` | 🔒 | — |
| 39 | `DELETE` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}` | 🔒 | — |
| 40 | `POST` | `/api/catalogo/{loja_uuid}/categorias` | 🔒 | — |
| 41 | `GET` | `/api/catalogo/{loja_uuid}/categorias` | 🔒 | — |
| 42 | `PUT` | `/api/catalogo/{loja_uuid}/categorias/{uuid}` | 🔒 | — |
| 43 | `DELETE` | `/api/catalogo/{loja_uuid}/categorias/{uuid}` | 🔒 | — |
| 44 | `POST` | `/api/enderecos-entrega/{pedido_uuid}/{loja_uuid}` | 🔒 | — |
| 45 | `GET` | `/api/enderecos-entrega/{pedido_uuid}` | 🔒 | — |
| 46 | `GET` | `/api/enderecos-entrega/{loja_uuid}/loja` | 🔒 | — |
| 47 | `POST` | `/api/enderecos-usuario/` | 🔒 | — |
| 48 | `GET` | `/api/enderecos-usuario/` | 🔒 | — |
| 49 | `GET` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 50 | `PUT` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 51 | `DELETE` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 52 | `POST` | `/api/favoritos/{loja_uuid}` | 🔒 | — |
| 53 | `DELETE` | `/api/favoritos/{loja_uuid}` | 🔒 | — |
| 54 | `GET` | `/api/favoritos/minhas` | 🔒 | — |
| 55 | `GET` | `/api/favoritos/{loja_uuid}/verificar` | 🔒 | — |
| 56 | `POST` | `/api/produtos/` | 🔒 | — |
| 57 | `GET` | `/api/produtos/` | 🔒 | — |
| 58 | `GET` | `/api/produtos/categoria/{categoria_uuid}` | 🔒 | — |
| 59 | `GET` | `/api/produtos/{uuid}` | 🔒 | — |
| 60 | `PUT` | `/api/produtos/{uuid}` | 🔒 | — |
| 61 | `DELETE` | `/api/produtos/{uuid}` | 🔒 | — |
| 62 | `POST` | `/api/produtos/{uuid}/imagem` | 🔒 | — |
| 63 | `GET` | `/api/ok` | — | — |
| 64 | `DELETE` | `/api/wipe` ⚠️ | — | — |

**Total: 65 endpoints**
