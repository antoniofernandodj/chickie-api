# Horários de Funcionamento — `/api/horarios/`

> GET é público. POST, PUT e DELETE requerem 🔒.

---

## GET /api/horarios/{loja_uuid}

**Response `200`:**
```json
[
  {
    "uuid": "uuid",
    "loja_uuid": "uuid",
    "dia_semana": 1,
    "abertura": "08:00",
    "fechamento": "22:00",
    "ativo": true
  }
]
```

---

## POST /api/horarios/{loja_uuid} 🔒

Cria ou atualiza o horário de um dia.

**Request Body:**
```json
{ "dia_semana": 1, "abertura": "08:00", "fechamento": "22:00", "ativo": true }
```

**Response `204`:** No Content

---

## PUT /api/horarios/{loja_uuid}/dia/{dia_semana}/ativo 🔒

**Request Body:**
```json
{ "ativo": false }
```

**Response `204`:** No Content

---

## DELETE /api/horarios/{loja_uuid}/dia/{dia_semana} 🔒

**Response `204`:** No Content
