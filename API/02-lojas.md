# Lojas Públicas — `/api/lojas/`

> Endpoints públicos de leitura de lojas. Mutações ficam em [04-admin.md](./04-admin.md).

---

## GET /api/lojas/

Lista todas as lojas.

**Response `200`:**
```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "Pizzaria do João",
    "slug": "pizzaria-do-joao",
    "descricao": "A melhor pizza da cidade",
    "email": "contato@pizzaria.com",
    "celular": "11999999999",
    "ativa": true,
    "logo_url": "https://example.com/logo.png",
    "banner_url": "https://example.com/banner.jpg",
    "horario_abertura": "18:00",
    "horario_fechamento": "23:00",
    "dias_funcionamento": "2,3,4,5,6,0",
    "tempo_preparo_min": 45,
    "taxa_entrega": 5.0,
    "valor_minimo_pedido": 20.0,
    "raio_entrega_km": 5.0,
    "criado_em": "2026-04-04T00:00:00Z",
    "atualizado_em": "2026-04-04T00:00:00Z"
  }
]
```

---

## GET /api/lojas/pesquisar?termo={valor}

**Response `200`:** mesmo schema de listar.

---

## GET /api/lojas/{uuid}

**Response `200`:** objeto loja (mesmo schema).

---

## GET /api/lojas/slug/{slug}

**Response `200`:** objeto loja (mesmo schema).

---

## GET /api/lojas/verificar-slug/{slug}

**Response `200`:**
```json
{ "disponivel": true, "slug": "pizzaria-do-joao" }
```

---

## PATCH /api/lojas/{loja_uuid}/bloqueado 🔒 👑 Owner

Alterna o status `bloqueado` da loja.

**Response `200`:**
```json
{ "message": "Loja bloqueada com sucesso", "bloqueado": true }
```
