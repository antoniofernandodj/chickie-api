# Catálogo — `/api/catalogo/`

> GETs são públicos. POST, PUT e DELETE requerem 🔒.

---

## Adicionais

### POST /api/catalogo/{loja_uuid}/adicionais 🔒

**Request Body:**
```json
{ "nome": "Queijo Extra", "descricao": "string", "preco": 3.50 }
```

**Response `200`:**
```json
{
  "uuid": "uuid",
  "nome": "Queijo Extra",
  "loja_uuid": "uuid",
  "disponivel": true,
  "descricao": "Queijo mussarela adicional",
  "preco": 3.50,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### GET /api/catalogo/{loja_uuid}/adicionais

**Response `200`:** array de adicionais.

---

### GET /api/catalogo/{loja_uuid}/adicionais/disponiveis

**Response `200`:** array de adicionais com `disponivel: true`.

---

### PUT /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid} 🔒

**Request Body:**
```json
{ "nome": "Queijo Extra Premium", "descricao": "...", "preco": 5.00 }
```

**Response `200`:** objeto adicional atualizado.

**Response `404`:** `{ "error": "Adicional não encontrado" }`

**Response `400`:** `{ "error": "Adicional não pertence a esta loja" }`

---

### PUT /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}/disponibilidade 🔒

**Request Body:**
```json
{ "disponivel": false }
```

**Response `204`:** No Content

---

### DELETE /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid} 🔒

**Response `204`:** No Content

---

## Categorias

### POST /api/catalogo/{loja_uuid}/categorias 🔒

> `ordem` não é definida na criação. Use `PUT /{loja_uuid}/categorias/reordenar` separadamente.

**Request Body:**
```json
{
  "nome": "Bebidas",
  "descricao": "string | null",
  "pizza_mode": false,
  "drink_mode": false
}
```

**Response `200`:**
```json
{
  "uuid": "uuid",
  "loja_uuid": "uuid",
  "nome": "Bebidas",
  "descricao": "Bebidas geladas",
  "pizza_mode": false,
  "drink_mode": false,
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### GET /api/catalogo/categorias/globais

Retorna categorias sem vínculo com loja.

**Response `200`:**
```json
[
  {
    "uuid": "uuid",
    "loja_uuid": null,
    "nome": "Pizzas",
    "descricao": "Pizzas de diversos sabores e tamanhos",
    "ordem": 1,
    "pizza_mode": true,
    "drink_mode": false,
    "criado_em": "2026-04-21T00:00:00Z"
  }
]
```

---

### GET /api/catalogo/{loja_uuid}/categorias

Retorna categorias próprias + globais com ordem da loja.

**Response `200`:**
```json
[
  {
    "uuid": "uuid",
    "loja_uuid": "uuid",
    "nome": "Bebidas",
    "descricao": "Bebidas geladas",
    "ordem": 1,
    "pizza_mode": false,
    "drink_mode": false,
    "criado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### PUT /api/catalogo/{loja_uuid}/categorias/{uuid} 🔒

**Request Body:**
```json
{ "nome": "Bebidas Geladas", "descricao": "string | null", "pizza_mode": false, "drink_mode": false }
```

**Response `200`:** objeto categoria atualizado.

---

### DELETE /api/catalogo/{loja_uuid}/categorias/{uuid} 🔒

> ⚠️ Apenas funciona se a categoria não tiver produtos vinculados.

**Response `204`:** No Content

**Response `400`:** `{ "error": "Não é possível deletar categoria com 3 produto(s). Remova os produtos primeiro." }`
