# Endereços de Entrega — `/api/enderecos-entrega/`

> Todos requerem 🔒.

---

## POST /api/enderecos-entrega/{pedido_uuid}/{loja_uuid} 🔒

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
  "uuid": "uuid",
  "loja_uuid": "uuid",
  "pedido_uuid": "uuid",
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

## GET /api/enderecos-entrega/{pedido_uuid} 🔒

**Response `200`:** objeto endereço de entrega.

---

## GET /api/enderecos-entrega/{loja_uuid}/loja 🔒

**Response `200`:** array de endereços da loja.
