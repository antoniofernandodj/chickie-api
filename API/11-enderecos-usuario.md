# Endereços de Usuário — `/api/enderecos-usuario/`

> Todos requerem 🔒.

---

## POST /api/enderecos-usuario/ 🔒

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
  "usuario_uuid": "uuid",
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

## GET /api/enderecos-usuario/ 🔒

**Response `200`:** array de endereços do usuário autenticado.

---

## GET /api/enderecos-usuario/{uuid} 🔒

**Response `200`:** objeto endereço.

---

## PUT /api/enderecos-usuario/{uuid} 🔒

**Request Body:** mesmo schema de criação.

**Response `200`:** objeto endereço atualizado.

---

## DELETE /api/enderecos-usuario/{uuid} 🔒

**Response `204`:** No Content
