# Cupons — `/api/cupons/`

> CRUD padronizado de cupons. Endpoints legados em [07-marketing.md](./07-marketing.md).

---

## POST /api/cupons/ 🔒

**Request Body:**
```json
{
  "loja_uuid": "uuid",
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
  "uuid": "uuid",
  "loja_uuid": "uuid",
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

## GET /api/cupons/ 🔒

Retorna todos os cupons de todas as lojas.

**Response `200`:** array de cupons.

---

## GET /api/cupons/{uuid} 🔒

**Response `200`:** objeto cupom.

**Response `404`:** `{ "error": "Cupom não encontrado" }`

---

## PUT /api/cupons/{uuid} 🔒

**Request Body:** mesmo schema de criação (com `loja_uuid`).

**Response `204`:** No Content

---

## DELETE /api/cupons/{uuid} 🔒

**Response `204`:** No Content

**Response `404`:** `{ "error": "Cupom não encontrado" }`

---

## PATCH /api/cupons/{uuid}/status 🔒

**Request Body:**
```json
{ "ativo": true }
```

**Response `204`:** No Content
