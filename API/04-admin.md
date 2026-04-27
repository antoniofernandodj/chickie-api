# Administração — `/api/admin/`

> Todos os endpoints requerem autenticação. Criação de lojas exige `classe = "administrador"`.

---

## POST /api/admin/lojas 🔒 👑 Admin

**Request Body:**
```json
{
  "nome": "string",
  "slug": "string",
  "email_contato": "string",
  "descricao": "string | null",
  "celular": "string | null",
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

**Response `200`:** objeto loja completo.

---

## GET /api/admin/lojas/listar 🔒

Lista todas as lojas (admin).

**Response `200`:** array de objetos loja.

---

## GET /api/admin/lojas/minhas-lojas 🔒

Lista as lojas criadas pelo admin autenticado.

**Response `200`:**
```json
[
  { "uuid": "uuid", "nome": "Pizzaria do João", "slug": "pizzaria-do-joao" }
]
```

---

## POST /api/admin/lojas/{loja_uuid}/funcionarios 🔒 👑 Admin

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
  "cargo": "Gerente",
  "salario": 2500.0,
  "data_admissao": "2026-04-04",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

## POST /api/admin/lojas/{loja_uuid}/entregadores 🔒 👑 Admin

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
  "veiculo": "Moto",
  "placa": "ABC-1234",
  "disponivel": true,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

## POST /api/admin/lojas/{loja_uuid}/clientes 🔒 👑 Admin

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

## POST /api/admin/categorias/globais 🔒 👑 Owner

Cria uma categoria sem vínculo com loja específica.

**Request Body:**
```json
{
  "nome": "Quentinhas",
  "descricao": "Refeições completas",
  "pizza_mode": false,
  "drink_mode": false
}
```

**Response `200`:**
```json
{
  "uuid": "uuid",
  "loja_uuid": null,
  "nome": "Quentinhas",
  "descricao": "Refeições completas",
  "ordem": 11,
  "pizza_mode": false,
  "drink_mode": false,
  "criado_em": "2026-04-21T00:00:00Z"
}
```

---

## PUT /api/admin/categorias/globais/{uuid} 🔒 👑 Owner

**Request Body:**
```json
{
  "nome": "Pizzas Especiais",
  "descricao": "Pizzas premium de diversos sabores",
  "pizza_mode": true,
  "drink_mode": false
}
```

**Response `200`:** objeto categoria global atualizado.

**Response `404`:** `{ "error": "Categoria não encontrada" }`

**Response `400`:** `{ "error": "Categoria não é global" }`

---

## DELETE /api/admin/categorias/globais/{uuid} 🔒 👑 Owner

> ⚠️ Apenas funciona se a categoria não tiver produtos vinculados.

**Response `204`:** No Content
