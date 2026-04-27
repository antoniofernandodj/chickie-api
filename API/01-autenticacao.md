# Autenticação — `/api/auth/`

> Endpoints públicos de cadastro, login e validações.

---

## POST /api/auth/signup

**Request Body:**
```json
{
  "nome": "string",
  "username": "string",
  "senha": "string",
  "email": "string",
  "celular": "string",
  "auth_method": "string",
  "classe": "cliente" | "administrador" | "funcionario" | "entregador" | "owner"
}
```

> `classe` é opcional. Default: `"cliente"`.
> `celular` é filtrado automaticamente — apenas dígitos. Ex: `"(11) 99999-9999"` → `"11999999999"`.

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "João Silva",
  "username": "joao",
  "email": "joao@email.com",
  "celular": "11999999999",
  "classe": "cliente",
  "ativo": true,
  "passou_pelo_primeiro_acesso": false,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z",
  "modo_de_cadastro": "email",
  "senha_hash": "$2b$12$..."
}
```

---

## POST /api/auth/login

**Request Body:**
```json
{
  "email": "string",
  "senha": "string"
}
```

**Response `200`:**
```json
{
  "access_token": "string (JWT)",
  "token_type": "Bearer",
  "usuario": {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "João Silva",
    "username": "joao",
    "email": "joao@email.com",
    "classe": "owner",
    "ativo": true,
    "bloqueado": false
  }
}
```

> Se o email corresponder à variável de ambiente `OWNER_EMAIL`, `classe` será retornado como `"owner"`.

---

## GET /api/auth/me 🔒

**Response `200`:**
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "nome": "João Silva",
  "username": "joao",
  "email": "joao@email.com",
  "celular": "11999999999",
  "classe": "owner",
  "ativo": true,
  "bloqueado": false,
  "passou_pelo_primeiro_acesso": false,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z",
  "modo_de_cadastro": "email"
}
```

**Response `401`:**
```json
{ "error": "Token de autenticação não fornecido. Inclua o header: Authorization: Bearer <token>" }
```

---

## POST /api/auth/verificar-celular

**Request Body:**
```json
{ "celular": "11999999999" }
```

**Response `200`:**
```json
{ "disponivel": true }
```
