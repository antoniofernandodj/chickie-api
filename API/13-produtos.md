# Produtos — `/api/produtos/`

> GETs são públicos. POST, PUT e DELETE requerem 🔒.

---

## POST /api/produtos/ 🔒

**Request Body:**
```json
{
  "loja_uuid": "uuid",
  "categoria_uuid": "uuid",
  "nome": "Pizza Grande",
  "descricao": "string | null",
  "preco": 49.90,
  "imagem_url": "string | null",
  "disponivel": true,
  "tempo_preparo_min": 30,
  "destaque": false
}
```

**Response `200`:** objeto produto completo.

---

## GET /api/produtos/listar/{loja_uuid}

**Response `200`:**
```json
[
  {
    "uuid": "uuid",
    "loja_uuid": "uuid",
    "categoria_uuid": "uuid",
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

## GET /api/produtos/categoria/{loja_uuid}/{categoria_uuid}

**Response `200`:** array de produtos da categoria.

---

## GET /api/produtos/{uuid}

**Response `200`:** objeto produto.

---

## PUT /api/produtos/{uuid} 🔒

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

**Response `200`:** objeto produto atualizado.

---

## DELETE /api/produtos/{uuid} 🔒

**Response `204`:** No Content

---

## PUT /api/produtos/{loja_uuid}/{produto_uuid}/disponibilidade 🔒

**Request Body:**
```json
{ "disponivel": false }
```

**Response `204`:** No Content

**Response `404`:** `{ "error": "Produto não encontrado" }`

**Response `400`:** `{ "error": "Produto não pertence a esta loja" }`

---

## POST /api/produtos/{uuid}/imagem 🔒

`Content-Type: multipart/form-data` — campo `file`: imagem (JPEG, PNG, WebP, etc.)

**Response `200`:**
```json
{
  "uuid": "uuid",
  "imagem_url": "https://s3.example.com/bucket/produtos/uuid-pizza.jpg",
  "message": "Imagem enviada com sucesso"
}
```

**Variáveis de ambiente necessárias:**
```bash
S3_BUCKET_NAME=your-bucket-name
S3_REGION=us-east-1
S3_ENDPOINT=https://s3.example.com   # opcional
S3_ACCESS_KEY_ID=your-access-key     # opcional
S3_SECRET_ACCESS_KEY=your-secret-key # opcional
```
