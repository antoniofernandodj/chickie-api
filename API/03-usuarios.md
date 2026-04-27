# Usuários — `/api/usuarios/`

> Maioria exige `OwnerPermission`. Owner é definido pela variável de ambiente `OWNER_EMAIL` ou classe `owner`.

---

## GET /api/usuarios/ 🔒 👑 Owner

Lista usuários. Query opcional: `?classe=cliente|administrador|funcionario|entregador|owner`.

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "João Silva",
    "username": "joao",
    "email": "joao@email.com",
    "celular": "11999999999",
    "classe": "cliente",
    "ativo": true,
    "bloqueado": false,
    "passou_pelo_primeiro_acesso": true,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z",
    "modo_de_cadastro": "email",
    "senha_hash": "$2b$12$..."
  }
]
```

---

## PATCH /api/usuarios/{usuario_uuid}/marcar-remocao 🔒 (Self/Owner)

**Response `204`:** No Content

---

## PATCH /api/usuarios/{usuario_uuid}/desmarcar-remocao 🔒 (Self/Owner)

**Response `204`:** No Content

---

## PUT /api/usuarios/{usuario_uuid}/ativo 🔒 👑 Owner

**Request Body:**
```json
{ "ativo": true }
```

**Response `200`:**
```json
{ "message": "Usuário ativado com sucesso", "ativo": true }
```

---

## PATCH /api/usuarios/{usuario_uuid}/bloqueado 🔒 👑 Owner

Alterna o status `bloqueado`. Usuários bloqueados não podem fazer login.

**Response `200`:**
```json
{ "message": "Usuário bloqueado com sucesso", "bloqueado": true }
```
