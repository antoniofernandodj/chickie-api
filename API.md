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

## 1. Health Check (sem auth)

### 1.1 OK

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

## 2. Autenticação (sem auth)

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

**Response `200`:** `Usuario` (objeto completo)

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

---

## 3. Lojas (público)

### 3.1 Listar Lojas

```
GET /api/lojas/
```

**Response `200`:** `Vec<Loja>`

---

## 4. Usuários (🔒)

### 4.1 Listar Usuários

```
GET /api/usuarios/
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Usuario>`

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

**Response `200`:** `Loja`

---

### 5.2 Listar Todas as Lojas (Admin)

```
GET /api/admin/lojas/listar
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Loja>`

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

**Response `200`:** `Funcionario`

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

**Response `200`:** `Entregador`

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

**Response `200`:** `Cliente`

---

## 6. Pedidos (🔒)

### 6.1 Criar Pedido

```
POST /api/pedidos/{loja_uuid}
Authorization: Bearer <token>
Content-Type: application/json
```

> O `usuario_uuid` é extraído automaticamente do JWT. O `loja_uuid` vem do path.

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

**Response `200`:**
```json
{
  "uuid": "uuid"
}
```

---

### 6.2 Listar Pedidos

```
GET /api/pedidos/
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Pedido>`

---

### 6.3 Buscar Pedido

```
GET /api/pedidos/{uuid}
Authorization: Bearer <token>
```

**Response `200`:** `Pedido` (com itens, partes e adicionais hidratados)

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

**Response `200`:** `Cupom`

---

### 7.2 Listar Cupons

```
GET /api/marketing/cupons
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Cupom>`

---

### 7.3 Validar Cupom

```
GET /api/marketing/cupons/{codigo}
```

**Response `200`:** `Cupom`

**Response `404`:**
```json
{ "error": "Cupom não encontrado" }
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

**Response `200`:** `AvaliacaoDeLoja`

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

**Response `200`:** `AvaliacaoDeProduto`

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

**Response `200`:** `Adicional`

---

### 8.2 Listar Adicionais

```
GET /api/catalogo/{loja_uuid}/adicionais
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Adicional>`

---

### 8.3 Listar Adicionais Disponíveis

```
GET /api/catalogo/{loja_uuid}/adicionais/disponiveis
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Adicional>` (apenas onde `disponivel = true`)

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

**Response `200`:** `CategoriaProdutos`

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

**Response `200`:** `EnderecoEntrega`

---

### 9.2 Buscar Endereço por Pedido

```
GET /api/enderecos-entrega/{pedido_uuid}
Authorization: Bearer <token>
```

**Response `200`:** `EnderecoEntrega`

---

### 9.3 Listar Endereços por Loja

```
GET /api/enderecos-entrega/{loja_uuid}/loja
Authorization: Bearer <token>
```

**Response `200`:** `Vec<EnderecoEntrega>`

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

**Response `200`:** `EnderecoUsuario`

---

### 10.2 Listar Endereços

```
GET /api/enderecos-usuario/
Authorization: Bearer <token>
```

**Response `200`:** `Vec<EnderecoUsuario>`

---

### 10.3 Buscar Endereço

```
GET /api/enderecos-usuario/{uuid}
Authorization: Bearer <token>
```

**Response `200`:** `EnderecoUsuario`

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

**Response `200`:** `EnderecoUsuario` (objeto atualizado)

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

**Request Body:** _(nenhum)_

**Response `200`:** `LojaFavorita`

---

### 11.2 Remover Favorita

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

### 11.3 Listar Minhas Favoritas

```
GET /api/favoritos/minhas
Authorization: Bearer <token>
```

**Request Body:** _(nenhum)_

**Response `200`:** `Vec<LojaFavorita>`

---

### 11.4 Verificar se é Favorita

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

**Response `200`:** `Produto`

---

### 12.2 Listar Produtos

```
GET /api/produtos/
Authorization: Bearer <token>
```

**Response `200`:** `Vec<Produto>`

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

**Response `200`:** `Produto` (objeto atualizado)

---

## 13. Utilitários

### 13.1 Wipe Database (⚠️ Dev Only)

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

## Sumário Completo de Endpoints

| # | Método | Rota | Auth | Admin | Body |
|---|--------|------|------|-------|------|
| 1 | `GET` | `/` | — | — | — |
| 2 | `POST` | `/api/auth/signup` | — | — | `CreateUsuarioRequest` |
| 3 | `POST` | `/api/auth/login` | — | — | `LoginRequest` |
| 4 | `GET` | `/api/lojas/` | — | — | — |
| 5 | `GET` | `/api/usuarios/` | 🔒 | — | — |
| 6 | `POST` | `/api/admin/lojas` | 🔒 | 👑 | `CreateLojaRequest` |
| 7 | `GET` | `/api/admin/lojas/listar` | 🔒 | — | — |
| 8 | `POST` | `/api/admin/lojas/{loja_uuid}/funcionarios` | 🔒 | 👑 | `AdicionarFuncionarioRequest` |
| 9 | `POST` | `/api/admin/lojas/{loja_uuid}/entregadores` | 🔒 | 👑 | `AdicionarEntregadorRequest` |
| 10 | `POST` | `/api/admin/lojas/{loja_uuid}/clientes` | 🔒 | 👑 | `AdicionarClienteRequest` |
| 11 | `POST` | `/api/pedidos/{loja_uuid}` | 🔒 | — | `CreatePedidoRequest` |
| 12 | `GET` | `/api/pedidos/` | 🔒 | — | — |
| 13 | `GET` | `/api/pedidos/{uuid}` | 🔒 | — | — |
| 14 | `POST` | `/api/marketing/{loja_uuid}/cupons` | 🔒 | — | `CriarCupomRequest` |
| 15 | `GET` | `/api/marketing/cupons` | 🔒 | — | — |
| 16 | `GET` | `/api/marketing/cupons/{codigo}` | — | — | — |
| 17 | `POST` | `/api/marketing/{loja_uuid}/avaliar-loja` | 🔒 | — | `AvaliarLojaRequest` |
| 18 | `POST` | `/api/marketing/{loja_uuid}/avaliar-produto` | 🔒 | — | `AvaliarProdutoRequest` |
| 19 | `POST` | `/api/marketing/{loja_uuid}/promocoes` | 🔒 | — | `CriarPromocaoRequest` |
| 20 | `POST` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — | `CreateAdicionalRequest` |
| 21 | `GET` | `/api/catalogo/{loja_uuid}/adicionais` | 🔒 | — | — |
| 22 | `GET` | `/api/catalogo/{loja_uuid}/adicionais/disponiveis` | 🔒 | — | — |
| 23 | `PUT` | `/api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/indisponivel` | 🔒 | — | — |
| 24 | `POST` | `/api/catalogo/{loja_uuid}/categorias` | 🔒 | — | `CreateCategoriaRequest` |
| 25 | `POST` | `/api/enderecos-entrega/{pedido_uuid}/{loja_uuid}` | 🔒 | — | `CreateEnderecoEntregaRequest` |
| 26 | `GET` | `/api/enderecos-entrega/{pedido_uuid}` | 🔒 | — | — |
| 27 | `GET` | `/api/enderecos-entrega/{loja_uuid}/loja` | 🔒 | — | — |
| 28 | `POST` | `/api/enderecos-usuario/` | 🔒 | — | `CreateEnderecoUsuarioRequest` |
| 29 | `GET` | `/api/enderecos-usuario/` | 🔒 | — | — |
| 30 | `GET` | `/api/enderecos-usuario/{uuid}` | 🔒 | — | — |
| 31 | `PUT` | `/api/enderecos-usuario/{uuid}` | 🔒 | — | `UpdateEnderecoUsuarioRequest` |
| 32 | `DELETE` | `/api/enderecos-usuario/{uuid}` | 🔒 | — | — |
| 33 | `POST` | `/api/favoritos/{loja_uuid}` | 🔒 | — | — |
| 34 | `DELETE` | `/api/favoritos/{loja_uuid}` | 🔒 | — | — |
| 35 | `GET` | `/api/favoritos/minhas` | 🔒 | — | — |
| 36 | `GET` | `/api/favoritos/{loja_uuid}/verificar` | 🔒 | — | — |
| 37 | `POST` | `/api/produtos/` | 🔒 | — | `CreateProdutoRequest` |
| 38 | `GET` | `/api/produtos/` | 🔒 | — | — |
| 39 | `PUT` | `/api/produtos/{uuid}` | 🔒 | — | `AtualizarProdutoRequest` |
| 40 | `DELETE` | `/api/wipe` ⚠️ | — | — | — |

**Total: 40 endpoints**
