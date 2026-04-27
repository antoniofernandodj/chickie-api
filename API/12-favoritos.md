# Lojas Favoritas — `/api/favoritos/`

> Todos requerem 🔒.

---

## POST /api/favoritos/{loja_uuid} 🔒

Adiciona loja às favoritas.

**Response `200`:**
```json
{
  "usuario_uuid": "uuid",
  "loja_uuid": "uuid",
  "uuid": "uuid",
  "criado_em": "2026-04-04T00:00:00Z"
}
```

---

## DELETE /api/favoritos/{loja_uuid} 🔒

**Response `200`:**
```json
{ "message": "Loja removida das favoritas" }
```

---

## GET /api/favoritos/minhas 🔒

**Response `200`:**
```json
[
  { "usuario_uuid": "uuid", "loja_uuid": "uuid", "uuid": "uuid", "criado_em": "2026-04-04T00:00:00Z" }
]
```

---

## GET /api/favoritos/{loja_uuid}/verificar 🔒

**Response `200`:**
```json
{ "favorita": true }
```
