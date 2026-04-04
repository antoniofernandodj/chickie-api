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

Endpoints marcados com 🔒 exigem header:
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
| `400` | Bad Request — dados inválidos |
| `403` | Forbidden — sem permissão (admin necessário) |
| `404` | Not Found — recurso não encontrado |
| `500` | Internal Server Error — erro interno |

---

## 1. Autenticação (sem auth)

### 1.1 Cadastrar Usuário

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
  "classe": "cliente" | "administrador" | "funcionario" | "entregador" | "owner"  // opcional, default: "cliente"
}
```

**Response `200`:**
```json
{
  "uuid": "uuid",
  "nome": "string",
  "username": "string",
  "email": "string",
  "celular": "string",
  "telefone": null,
  "classe": "cliente",
  "ativo": true,
  "passou_pelo_primeiro_acesso": false,
  "modo_de_cadastro": "email",
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z"
}
```

---

### 1.2 Login

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

---

## 2. Lojas (público)

### 2.1 Listar Lojas

```
GET /api/lojas/
```

**Response `200`:**
```json
[
  {
    "uuid": "uuid",
    "nome": "string",
    "slug": "string",
    "descricao": "string | null",
    "email": "string",
    "ativa": true,
    "logo_url": null,
    "banner_url": null,
    "horario_abertura": null,
    "horario_fechamento": null,
    "taxa_entrega": null,
    "valor_minimo_pedido": null,
    "raio_entrega_km": null,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

## 3. Administração (🔒 + 👑 Admin)

### 3.1 Criar Loja

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

**Response `200`:** `Loja` (objeto completo)

---

### 3.2 Listar Todas as Lojas (Admin)

```
GET /api/admin/lojas/listar
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Loja>`

---

### 3.3 Adicionar Funcionário

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
  "uuid": "uuid",
  "loja_uuid": "uuid",
  "usuario_uuid": "uuid",
  "cargo": "Caixa",
  "salario": 2500.0,
  "data_admissao": "2026-04-04",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 3.4 Adicionar Entregador

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
  "uuid": "uuid",
  "loja_uuid": "uuid",
  "usuario_uuid": "uuid",
  "veiculo": "Motocicleta",
  "placa": "ABC-1234",
  "disponivel": false,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### 3.5 Adicionar Cliente

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
  "uuid": "uuid",
  "usuario_uuid": "uuid",
  "loja_uuid": "uuid",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

## 4. Produtos (🔒)

### 4.1 Criar Produto

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

**Response `200`:** `Produto` (objeto completo)

---

### 4.2 Listar Produtos

```
GET /api/produtos/
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Produto>`

---

### 4.3 Atualizar Produto

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

**Response `200`:** `Produto` (objeto atualizado)

---

## 5. Pedidos (🔒)

### 5.1 Criar Pedido

```
POST /api/pedidos/
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "usuario_uuid": "uuid",
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

**Response `200`:**
```json
{
  "uuid": "uuid"
}
```

---

### 5.2 Listar Pedidos

```
GET /api/pedidos/
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Pedido>`

---

### 5.3 Buscar Pedido

```
GET /api/pedidos/{uuid}
Authorization: Bearer <token>
```

**Response `200`:** `Pedido` (com itens, partes e adicionais hidratados)

---

## 6. Cupons & Marketing (🔒 exceto validar)

### 6.1 Criar Cupom

```
POST /api/cupons/
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

**Response `200`:** `Cupom` (objeto completo)

---

### 6.2 Listar Cupons

```
GET /api/cupons/
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Cupom>`

---

### 6.3 Validar Cupom

```
GET /api/cupons/{codigo}
```

**Response `200`:** `Cupom`

**Response `404`:**
```json
{ "error": "Cupom não encontrado" }
```

---

### 6.4 Avaliar Loja

```
POST /api/cupons/{loja_uuid}/avaliar-loja
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

**Response `200`:** `AvaliacaoDeLoja`

---

### 6.5 Avaliar Produto

```
POST /api/cupons/{loja_uuid}/avaliar-produto
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

**Response `200`:** `AvaliacaoDeProduto`

---

### 6.6 Criar Promoção

```
POST /api/cupons/{loja_uuid}/promocoes
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "nome": "Black Friday",
  "descricao": "string",
  "tipo_desconto": "percentual",
  "valor_desconto": 50.0,
  "valor_minimo": 100.0,
  "data_inicio": "2026-11-25T00:00:00Z",
  "data_fim": "2026-11-25T23:59:59Z",
  "dias_semana_validos": [5],
  "prioridade": 1
}
```

**Response `200`:** `Promocao`

---

## 7. Catálogo (🔒)

### 7.1 Criar Adicional

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

**Response `200`:** `Adicional`

---

### 7.2 Listar Adicionais

```
GET /api/catalogo/{loja_uuid}/adicionais
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Adicional>`

---

### 7.3 Listar Adicionais Disponíveis

```
GET /api/catalogo/{loja_uuid}/adicionais/disponiveis
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Adicional>` (apenas onde `disponivel = true`)

---

### 7.4 Marcar Adicional como Indisponível

```
PUT /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/indisponivel
Authorization: Bearer <token>
```

**Response `204`:** No Content

---

### 7.5 Criar Categoria

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

**Response `200`:** `CategoriaProdutos`

---

## 8. Endereços de Entrega (🔒)

### 8.1 Criar Endereço para Pedido

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

**Response `200`:** `EnderecoEntrega`

---

### 8.2 Buscar Endereço por Pedido

```
GET /api/enderecos-entrega/{pedido_uuid}
Authorization: Bearer <token>
```

**Response `200`:** `EnderecoEntrega`

---

### 8.3 Listar Endereços por Loja

```
GET /api/enderecos-entrega/{loja_uuid}/loja
Authorization: Bearer <token>
```

**Response `200`:** `Vec<EnderecoEntrega>`

---

## 9. Endereços de Usuário (🔒)

### 9.1 Criar Endereço

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

**Response `200`:** `EnderecoUsuario`

---

### 9.2 Listar Endereços

```
GET /api/enderecos-usuario/
Authorization: Bearer <token>
```

**Response `200`:** `Vec<EnderecoUsuario>`

---

### 9.3 Buscar Endereço

```
GET /api/enderecos-usuario/{uuid}
Authorization: Bearer <token>
```

**Response `200`:** `EnderecoUsuario`

---

### 9.4 Atualizar Endereço

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

**Response `200`:** `EnderecoUsuario` (objeto atualizado)

---

### 9.5 Deletar Endereço

```
DELETE /api/enderecos-usuario/{uuid}
Authorization: Bearer <token>
```

**Response `204`:** No Content

---

## 10. Lojas Favoritas (🔒)

### 10.1 Adicionar Favorita

```
POST /api/favoritos/{loja_uuid}
Authorization: Bearer <token>
```

**Request Body:** _(nenhum)_

**Response `200`:** `LojaFavorita`

---

### 10.2 Remover Favorita

```
DELETE /api/favoritos/{loja_uuid}
Authorization: Bearer <token>
```

**Request Body:** _(nenhum)_

**Response `200`:**
```json
{
  "message": "Loja removida das favoritas"
}
```

---

### 10.3 Listar Minhas Favoritas

```
GET /api/favoritos/minhas
Authorization: Bearer <token>
```

**Request Body:** _(nenhum)_

**Response `200`:** `Vec<LojaFavorita>`

---

### 10.4 Verificar se é Favorita

```
GET /api/favoritos/{loja_uuid}/verificar
Authorization: Bearer <token>
```

**Request Body:** _(nenhum)_

**Response `200`:**
```json
{
  "favorita": true
}
```

---

## 11. Utilitários

### 11.1 Health Check

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

### 11.2 Wipe Database (⚠️ Dev Only)

```
DELETE /api/wipe
```

> ⚠️ **Apenas para desenvolvimento.** Requer `MODE=development`. Apaga todas as tabelas e reaplica migrações.

**Response `200`:**
```json
{
  "message": "Database wiped successfully",
  "warning": "⚠️ All data has been permanently deleted"
}
```

---

## Sumário de Endpoints

| # | Método | Rota | Auth | Admin | Body |
|---|--------|------|------|-------|------|
| 1 | `POST` | `/api/auth/signup` | — | — | `CreateUsuarioRequest` |
| 2 | `POST` | `/api/auth/login` | — | — | `LoginRequest` |
| 3 | `GET` | `/api/lojas/` | — | — | — |
| 4 | `POST` | `/api/admin/lojas` | 🔒 | 👑 | `CreateLojaRequest` |
| 5 | `GET` | `/api/admin/lojas/listar` | 🔒 | — | — |
| 6 | `POST` | `/api/admin/lojas/{loja_uuid}/funcionarios` | 🔒 | 👑 | `AdicionarFuncionarioRequest` |
| 7 | `POST` | `/api/admin/lojas/{loja_uuid}/entregadores` | 🔒 | 👑 | `AdicionarEntregadorRequest` |
| 8 | `POST` | `/api/admin/lojas/{loja_uuid}/clientes` | 🔒 | 👑 | `AdicionarClienteRequest` |
| 9 | `POST` | `/api/produtos/` | 🔒 | — | `CreateProdutoRequest` |
| 10 | `GET` | `/api/produtos/` | 🔒 | — | — |
| 11 | `PUT` | `/api/produtos/{uuid}` | 🔒 | — | `AtualizarProdutoRequest` |
| 12 | `POST` | `/api/pedidos/` | 🔒 | — | `CreatePedidoRequest` |
| 13 | `GET` | `/api/pedidos/` | 🔒 | — | — |
| 14 | `GET` | `/api/pedidos/{uuid}` | 🔒 | — | — |
| 15 | `POST` | `/api/cupons/` | — | — | `CriarCupomRequest` |
| 16 | `GET` | `/api/cupons/` | 🔒 | — | — |
| 17 | `GET` | `/api/cupons/{codigo}` | — | — | — |
| 18 | `POST` | `/api/cupons/{loja_uuid}/avaliar-loja` | 🔒 | — | `AvaliarLojaRequest` |
| 19 | `POST` | `/api/cupons/{loja_uuid}/avaliar-produto` | 🔒 | — | `AvaliarProdutoRequest` |
| 20 | `POST` | `/api/cupons/{loja_uuid}/promocoes` | 🔒 | — | `CriarPromocaoRequest` |
| 21 | `POST` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — | `CreateAdicionalRequest` |
| 22 | `GET` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — | — |
| 23 | `GET` | `/api/catalogo/{loja_uuid}/adicionais/disponiveis` | 🔒 | — | — |
| 24 | `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/indisponivel` | 🔒 | — | — |
| 25 | `POST` | `/api/catalogo/{loja_uuid}/categorias` | 🔒 | — | `CreateCategoriaRequest` |
| 26 | `POST` | `/api/enderecos-entrega/{pedido_uuid}/{loja_uuid}` | 🔒 | — | `CreateEnderecoEntregaRequest` |
| 27 | `GET` | `/api/enderecos-entrega/{pedido_uuid}` | 🔒 | — | — |
| 28 | `GET` | `/api/enderecos-entrega/{loja_uuid}/loja` | 🔒 | — | — |
| 29 | `POST` | `/api/enderecos-usuario/` | 🔒 | — | `CreateEnderecoUsuarioRequest` |
| 30 | `GET` | `/api/enderecos-usuario/` | 🔒 | — | — |
| 31 | `GET` | `/api/enderecos-usuario/{uuid}` | 🔒 | — | — |
| 32 | `PUT` | `/api/enderecos-usuario/{uuid}` | 🔒 | — | `UpdateEnderecoUsuarioRequest` |
| 33 | `DELETE` | `/api/enderecos-usuario/{uuid}` | 🔒 | — | — |
| 34 | `POST` | `/api/favoritos/{loja_uuid}` | 🔒 | — | — |
| 35 | `DELETE` | `/api/favoritos/{loja_uuid}` | 🔒 | — | — |
| 36 | `GET` | `/api/favoritos/minhas` | 🔒 | — | — |
| 37 | `GET` | `/api/favoritos/{loja_uuid}/verificar` | 🔒 | — | — |
| 38 | `DELETE` | `/api/wipe` | — | — | — |

**Total: 38 endpoints**
