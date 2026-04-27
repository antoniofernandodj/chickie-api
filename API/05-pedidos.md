# Pedidos — `/api/pedidos/`

> Auth opcional em criação; demais endpoints requerem 🔒. Todos os responses incluem `contato` e `endereco_entrega`.

---

## POST /api/pedidos/criar

Auth opcional. Sem token, o pedido é criado como anônimo.

**Request Body:**
```json
{
  "loja_uuid": "uuid",
  "taxa_entrega": 5.0,
  "forma_pagamento": "PIX",
  "observacoes": "string | null",
  "contato": "string | null",
  "codigo_cupom": "string | null",
  "itens": [
    {
      "quantidade": 1,
      "observacoes": "string | null",
      "partes": [
        { "produto_uuid": "uuid", "posicao": 1 }
      ]
    }
  ],
  "endereco_entrega": null
}
```

> `endereco_entrega` opcional. Se fornecido:
> ```json
> { "cep": "string | null", "logradouro": "string", "numero": "string",
>   "complemento": "string | null", "bairro": "string", "cidade": "string", "estado": "string" }
> ```

**Response `201`:**
```json
{ "uuid": "uuid", "codigo": "A1B2C3" }
```

---

## GET /api/pedidos/listar 🔒

Lista todos os pedidos da plataforma (mais recente primeiro).

**Response `200`:** array de pedidos completos (ver schema abaixo).

---

## GET /api/pedidos/meus 🔒

Retorna pedidos do usuário autenticado.

**Response `200`:** array de pedidos completos | `[]`

---

## GET /api/pedidos/codigo/{codigo}

Endpoint público para localizar um pedido pelo código alfanumérico de 6 caracteres.

**Response `200`:** objeto pedido.

---

## GET /api/pedidos/por-loja/{loja_uuid} 🔒

**Response `200`:** array de pedidos completos.

---

## GET /api/pedidos/{uuid} 🔒

**Response `200`:** objeto pedido completo (schema abaixo).

---

## GET /api/pedidos/{pedido_uuid}/com-entrega 🔒

Mantido por compatibilidade. Retorna `{ pedido, endereco_entrega }` onde ambos contêm o endereço.

---

## PUT /api/pedidos/{uuid}/status 🔒

**Request Body:**
```json
{ "novo_status": "em_preparo" }
```

Valores válidos: `criado`, `aguardando_confirmacao_de_loja`, `confirmado_pela_loja`, `em_preparo`, `pronto`, `saiu_para_entrega`, `entregue`.

**Response `200`:**
```json
{
  "uuid": "pedido-uuid",
  "status": "em_preparo",
  "transicoes_permitidas": ["pronto", "confirmado_pela_loja"]
}
```

**Response `400`:**
```json
{ "error": "Transição inválida: Criado -> Entregue. Transições permitidas: [AguardandoConfirmacaoDeLoja]" }
```

---

## PUT /api/pedidos/{pedido_uuid}/entregador/{loja_uuid} 🔒

Vincula um entregador ao pedido.

**Request Body:**
```json
{ "entregador_uuid": "uuid" }
```

**Response `204`:** No Content

---

## DELETE /api/pedidos/{pedido_uuid}/entregador/{loja_uuid} 🔒

Remove o entregador vinculado (define `entregador_uuid` como NULL).

**Response `204`:** No Content

---

## GET /api/pedidos/{pedido_uuid}/com-entregador 🔒

**Response `200`:**
```json
{
  "uuid": "uuid",
  "usuario_uuid": "uuid",
  "loja_uuid": "uuid",
  "entregador_uuid": "uuid",
  "status": "saiu_para_entrega",
  "total": 65.90,
  "subtotal": 55.90,
  "taxa_entrega": 5.0,
  "desconto": 0.0,
  "forma_pagamento": "PIX",
  "observacoes": "Sem cebola",
  "contato": "11999999999",
  "tempo_estimado_min": 45,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z",
  "endereco_entrega": { "...": "..." },
  "entregador_nome": "Carlos Entregador",
  "veiculo": "Moto",
  "placa": "ABC-1234"
}
```

---

## Schema de Pedido Completo

```json
{
  "uuid": "uuid",
  "codigo": "A1B2C3",
  "usuario_uuid": "uuid | null",
  "loja_uuid": "uuid",
  "entregador_uuid": "uuid | null",
  "status": "criado",
  "total": 65.90,
  "subtotal": 55.90,
  "taxa_entrega": 5.0,
  "desconto": 0.0,
  "forma_pagamento": "PIX",
  "observacoes": "Sem cebola",
  "contato": "11999999999",
  "tempo_estimado_min": 45,
  "criado_em": "2026-04-04T00:00:00Z",
  "atualizado_em": "2026-04-04T00:00:00Z",
  "itens": [
    {
      "uuid": "uuid",
      "loja_uuid": "uuid",
      "pedido_uuid": "uuid",
      "quantidade": 1,
      "observacoes": null,
      "adicionais": [],
      "partes": [
        {
          "uuid": "uuid",
          "loja_uuid": "uuid",
          "item_uuid": "uuid",
          "produto_nome": "Pizza Grande",
          "produto_uuid": "uuid",
          "preco_unitario": 49.90,
          "posicao": 1,
          "adicionais": [
            {
              "uuid": "uuid",
              "item_uuid": "uuid",
              "loja_uuid": "uuid",
              "nome": "Queijo Extra",
              "descricao": "Queijo mussarela adicional",
              "preco": 3.50
            }
          ]
        }
      ]
    }
  ],
  "endereco_entrega": {
    "uuid": "uuid",
    "loja_uuid": "uuid",
    "pedido_uuid": "uuid",
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
}
```

> `endereco_entrega` é `null` quando não cadastrado.
