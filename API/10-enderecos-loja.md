# Endereços de Loja — `/api/enderecos-loja/`

> Todos requerem 🔒.

---

## GET /api/enderecos-loja/{loja_uuid} 🔒

**Response `200`:**
```json
[
  {
    "uuid": "uuid",
    "loja_uuid": "uuid",
    "cep": "01001-000",
    "logradouro": "Rua das Lojas",
    "numero": "100",
    "complemento": "Loja 5",
    "bairro": "Centro",
    "cidade": "São Paulo",
    "estado": "SP",
    "latitude": -23.5505,
    "longitude": -46.6333
  }
]
```

---

## POST /api/enderecos-loja/{loja_uuid} 🔒

**Request Body:**
```json
{
  "cep": "string | null",
  "logradouro": "string",
  "numero": "string",
  "complemento": "string | null",
  "bairro": "string",
  "cidade": "string",
  "estado": "string",
  "latitude": -23.5505,
  "longitude": -46.6333
}
```

**Response `201`:**
```json
{ "uuid": "uuid" }
```

---

## PUT /api/enderecos-loja/{loja_uuid}/{endereco_uuid} 🔒

**Request Body:** mesmo schema de criação.

**Response `204`:** No Content

---

## DELETE /api/enderecos-loja/{loja_uuid}/{endereco_uuid} 🔒

**Response `204`:** No Content

**Response `404`:** `{ "error": "Endereço não encontrado" }`
