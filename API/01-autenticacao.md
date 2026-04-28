# Autenticação — `/api/auth/`

> Endpoints públicos de cadastro, login e validações.

---

## POST /api/auth/signup

Inicia o cadastro: valida os dados, salva um pré-cadastro em cache PostgreSQL e envia um email de verificação. O usuário só é criado em banco após confirmar o link. O pré-cadastro expira em **1 hora**.

**Request Body:**
```json
{
  "nome": "string",
  "username": "string",
  "senha": "string",
  "email": "string",
  "celular": "string",
  "cpf": "string (11 dígitos)",
  "auth_method": "string",
  "classe": "cliente" | "administrador" | "funcionario" | "entregador" | "owner"
}
```

> `classe` é opcional. Default: `"cliente"`.
> `celular` e `cpf` são filtrados automaticamente — apenas dígitos.

**Response `202`:**
```json
{
  "message": "Email de verificação enviado para joao@email.com. Você tem 1 hora para confirmar."
}
```

---

## GET /api/auth/confirmar-email?token=\<token\>

Confirma o cadastro via token recebido por email. Cria o usuário em banco, remove o pré-cadastro do cache e retorna o JWT para login imediato.

**Query Parameters:**

| Parâmetro | Tipo   | Obrigatório |
|-----------|--------|-------------|
| `token`   | string | ✅           |

**Response `200`:**
```json
{
  "token": "eyJhbGci...",
  "usuario": {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "João Silva",
    "email": "joao@email.com",
    "classe": "cliente"
  }
}
```

**Response `400` (token inválido ou expirado):**
```json
{ "error": "Token inválido ou expirado. Faça o cadastro novamente." }
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
