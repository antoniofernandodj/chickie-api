# Toggle Bloqueado Endpoints — Chickie API

## New Endpoints Added

### Toggle Usuário Bloqueado

```
POST /api/usuarios/{usuario_uuid}/bloqueado
Authorization: Bearer <token>
```

> Toggle the blocked status of a user. Blocked users cannot login. Requires admin permission.

**Response `200`:**
```json
{
  "message": "Usuário bloqueado com sucesso",
  "bloqueado": true
}
```

Or when unblocking:
```json
{
  "message": "Usuário desbloqueado com sucesso",
  "bloqueado": false
}
```

**Response `400`:**
```json
{
  "error": "Não é possível bloquear usuário deletado"
}
```

**Response `401`:**
```json
{
  "error": "Token de autenticação não fornecido. Inclua o header: Authorization: Bearer <token>"
}
```

**Response `403`:**
```json
{
  "error": "Apenas administradores podem realizar essa ação. classe de usuario: cliente"
}
```

---

### Toggle Loja Bloqueada

```
POST /api/lojas/{loja_uuid}/bloqueado
Authorization: Bearer <token>
```

> Toggle the blocked status of a store. Requires admin permission.

**Response `200`:**
```json
{
  "message": "Loja bloqueada com sucesso",
  "bloqueado": true
}
```

Or when unblocking:
```json
{
  "message": "Loja desbloqueada com sucesso",
  "bloqueado": false
}
```

**Response `400`:**
```json
{
  "error": "Não é possível bloquear loja deletada"
}
```

**Response `401`:**
```json
{
  "error": "Token de autenticação não fornecido. Inclua o header: Authorization: Bearer <token>"
}
```

**Response `403`:**
```json
{
  "error": "Apenas administradores podem realizar essa ação. classe de usuario: cliente"
}
```

---

## Authentication Changes

### Login Prevention for Blocked Users

The `POST /api/auth/login` endpoint now rejects blocked users:

**Response `400` (blocked user):**
```json
{
  "error": "Usuário bloqueado. Contate o suporte."
}
```

### Auth Middleware Changes

The JWT middleware now checks if users are blocked on every authenticated request:

**Response `403` (blocked user with valid JWT):**
```json
{
  "error": "Usuário bloqueado. Contate o suporte."
}
```

---

## Database Migration

Migration `0008_add_bloqueado_usuarios_lojas.sql` adds:

- `bloqueado BOOLEAN NOT NULL DEFAULT FALSE` to `usuarios` table
- `bloqueado BOOLEAN NOT NULL DEFAULT FALSE` to `lojas` table
- Indexes on `bloqueado` column for both tables

---

## Model Changes

### Usuario

New field: `bloqueado: bool`
- Default: `false`
- When `true`: user cannot login
- Checked in `esta_ativo_para_login()` method

New method: `esta_bloqueado() -> bool`

### Loja

New field: `bloqueado: bool`
- Default: `false`
- When `true`: store is blocked from operating
- Checked in `esta_operacional()` method

New method: `esta_bloqueada() -> bool`
