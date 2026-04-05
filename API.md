# API Specification — Chickie

> Especificação completa de todos os endpoints, request/response bodies, headers e autenticação.

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

### 5.3 Adicionar Funcionário

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

### 5.4 Adicionar Entregador

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

### 5.5 Adicionar Cliente

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

### 8.4 Marcar Adicional como Indisponível

```
PUT /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/indisponivel
Authorization: Bearer <token>
```

**Response `204`:** No Content

---

### 8.5 Criar Categoria

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
  "ordem": 1
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
  "criado_em": "2026-04-04T00:00:00Z"
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
  "uuid": "uuid",
  "categoria_uuid": "uuid",
  "nome": "Pizza Grande",
  "descricao": "string | null",
  "preco": 49.90,
  "imagem_url": "string | null",
  "disponivel": true,
  "tempo_preparo_min": 30,
  "destaque": false,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z"
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

### 12.3 Atualizar Produto

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
| 9 | `GET` | `/api/usuarios/` | 🔒 | — |
| 10 | `POST` | `/api/admin/lojas` | 🔒 | 👑 |
| 11 | `GET` | `/api/admin/lojas/listar` | 🔒 | — |
| 12 | `POST` | `/api/admin/lojas/{loja_uuid}/funcionarios` | 🔒 | 👑 |
| 13 | `POST` | `/api/admin/lojas/{loja_uuid}/entregadores` | 🔒 | 👑 |
| 14 | `POST` | `/api/admin/lojas/{loja_uuid}/clientes` | 🔒 | 👑 |
| 15 | `POST` | `/api/pedidos/criar` | 🔒 | — |
| 16 | `GET` | `/api/pedidos/` | 🔒 | — |
| 17 | `GET` | `/api/pedidos/{uuid}` | 🔒 | — |
| 18 | `GET` | `/api/pedidos/criar` | 🔒 | — |
| 19 | `GET` | `/api/pedidos/criar/{pedido_uuid}/com-entrega` | 🔒 | — |
| 20 | `PUT` | `/api/pedidos/criar/{pedido_uuid}/status` | 🔒 | — |
| 21 | `POST` | `/api/marketing/{loja_uuid}/cupons` | 🔒 | — |
| 22 | `GET` | `/api/marketing/cupons/{codigo}` | — | — |
| 23 | `GET` | `/api/marketing/cupons` | 🔒 | — |
| 24 | `POST` | `/api/marketing/{loja_uuid}/avaliar-loja` | 🔒 | — |
| 25 | `POST` | `/api/marketing/{loja_uuid}/avaliar-produto` | 🔒 | — |
| 26 | `POST` | `/api/marketing/{loja_uuid}/promocoes` | 🔒 | — |
| 27 | `GET` | `/api/marketing/{loja_uuid}/promocoes` | 🔒 | — |
| 28 | `PUT` | `/api/marketing/{loja_uuid}/promocoes/{uuid}` | 🔒 | — |
| 29 | `DELETE` | `/api/marketing/{loja_uuid}/promocoes/{uuid}` | 🔒 | — |
| 30 | `POST` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — |
| 31 | `GET` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — |
| 32 | `GET` | `/api/catalogo/{loja_uuid}/adicionais/disponiveis` | 🔒 | — |
| 33 | `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/indisponivel` | 🔒 | — |
| 34 | `POST` | `/api/catalogo/{loja_uuid}/categorias` | 🔒 | — |
| 35 | `POST` | `/api/enderecos-entrega/{pedido_uuid}/{loja_uuid}` | 🔒 | — |
| 36 | `GET` | `/api/enderecos-entrega/{pedido_uuid}` | 🔒 | — |
| 37 | `GET` | `/api/enderecos-entrega/{loja_uuid}/loja` | 🔒 | — |
| 38 | `POST` | `/api/enderecos-usuario/` | 🔒 | — |
| 39 | `GET` | `/api/enderecos-usuario/` | 🔒 | — |
| 40 | `GET` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 41 | `PUT` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 42 | `DELETE` | `/api/enderecos-usuario/{uuid}` | 🔒 | — |
| 43 | `POST` | `/api/favoritos/{loja_uuid}` | 🔒 | — |
| 44 | `DELETE` | `/api/favoritos/{loja_uuid}` | 🔒 | — |
| 45 | `GET` | `/api/favoritos/minhas` | 🔒 | — |
| 46 | `GET` | `/api/favoritos/{loja_uuid}/verificar` | 🔒 | — |
| 47 | `POST` | `/api/produtos/` | 🔒 | — |
| 48 | `GET` | `/api/produtos/` | 🔒 | — |
| 49 | `PUT` | `/api/produtos/{uuid}` | 🔒 | — |
| 50 | `GET` | `/api/ok` | — | — |
| 51 | `DELETE` | `/api/wipe` ⚠️ | — | — |

**Total: 51 endpoints**
