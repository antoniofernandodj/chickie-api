# Utilitários

---

## GET /

Health check raiz.

**Response `200`:**
```json
{ "message": "🚀 Servidor compilado com sucesso!" }
```

---

## GET /api/ok

**Response `200`:**
```json
{ "msg": "ok" }
```

---

## DELETE /api/wipe 🔒 👑 Owner

> ⚠️ Dev only. Apaga todas as tabelas e reaplica migrações. Remover antes de produção.

**Response `200`:**
```json
{
  "message": "Database wiped successfully",
  "warning": "⚠️ All data has been permanently deleted"
}
```
