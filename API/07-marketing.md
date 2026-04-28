# Marketing — `/api/marketing/`

> Cupons (legado), avaliações de loja/produto e promoções.

---

## Cupons (Legado)

### POST /api/marketing/{loja_uuid}/cupons 🔒

> ⚠️ Legado. Prefira `POST /api/cupons/`.

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

**Response `200`:** objeto cupom.

---

### GET /api/marketing/cupons 🔒

> ⚠️ Legado. Prefira `GET /api/cupons/`.

**Response `200`:** array de cupons.

---

### GET /api/marketing/cupons/{codigo}

Endpoint **público** para validar cupom pelo código.

**Response `200`:** objeto cupom.

---

## Avaliações de Loja

### POST /api/marketing/{loja_uuid}/avaliar-loja 🔒

**Request Body:**
```json
{ "nota": 4.5, "comentario": "string | null" }
```

**Response `200`:**
```json
{
  "uuid": "uuid",
  "loja_uuid": "uuid",
  "usuario_uuid": "uuid",
  "nota": 4.5,
  "comentario": "Ótima pizza!",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### GET /api/marketing/{loja_uuid}/avaliacoes-loja 🔒

Retorna avaliações ordenadas da mais recente. Inclui `usuario_nome` e `usuario_email` via JOIN.

**Response `200`:**
```json
[
  {
    "uuid": "uuid",
    "loja_uuid": "uuid",
    "usuario_uuid": "uuid",
    "usuario_nome": "João Silva",
    "usuario_email": "joao@email.com",
    "nota": "4.50",
    "comentario": "Ótima pizza!",
    "criado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

### GET /api/marketing/avaliacoes-loja/{uuid} 🔒

**Response `200`:** objeto avaliação de loja.

**Response `404`:** `{ "error": "Avaliação de loja não encontrada" }`

---

### PUT /api/marketing/avaliacoes-loja/{uuid} 🔒

**Request Body:**
```json
{ "nota": 3.5, "comentario": "Melhou depois da última visita" }
```

**Response `200`:** objeto avaliação atualizado.

---

### DELETE /api/marketing/avaliacoes-loja/{uuid} 🔒

**Response `204`:** No Content

---

## Avaliações de Produto

### POST /api/marketing/{loja_uuid}/avaliar-produto 🔒

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
  "uuid": "uuid",
  "usuario_uuid": "uuid",
  "loja_uuid": "uuid",
  "produto_uuid": "uuid",
  "nota": 5.0,
  "descricao": "Pizza deliciosa!",
  "comentario": "Melhor pizza que já comi",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

### GET /api/marketing/{loja_uuid}/avaliacoes-produto 🔒

**Response `200`:** array de avaliações de produto.

---

### GET /api/marketing/avaliacoes-produto/produto/{produto_uuid} 🔒

**Response `200`:** array de avaliações do produto específico.

---

### GET /api/marketing/avaliacoes-produto/{uuid} 🔒

**Response `200`:** objeto avaliação de produto.

**Response `404`:** `{ "error": "Avaliação de produto não encontrada" }`

---

### PUT /api/marketing/avaliacoes-produto/{uuid} 🔒

**Request Body:**
```json
{ "nota": 4.0, "descricao": "Boa pizza, mas poderia ser melhor", "comentario": "Massa boa, molho médio" }
```

**Response `200`:** objeto avaliação atualizado.

---

### DELETE /api/marketing/avaliacoes-produto/{uuid} 🔒

**Response `204`:** No Content

---

## Promoções

### POST /api/marketing/{loja_uuid}/promocoes 🔒

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
  "produto_uuid": "uuid",
  "categoria_uuid": null,
  "prioridade": 1
}
```

**Response `200`:** objeto promoção com `status: "Ativo"`.

---

### GET /api/marketing/{loja_uuid}/promocoes 🔒

**Response `200`:** array de promoções.

---

### PUT /api/marketing/{loja_uuid}/promocoes/{uuid} 🔒

**Request Body:** mesmo schema de criar.

**Response `204`:** No Content

---

### DELETE /api/marketing/{loja_uuid}/promocoes/{uuid} 🔒

**Response `204`:** No Content
