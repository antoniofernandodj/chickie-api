# API Specification — Chat

> Endpoints para comunicação em tempo real entre empresas (lojas) e clientes.

O sistema de chat utiliza **WebSockets** para entrega de mensagens em tempo real e **HTTP** para recuperação de histórico e gerenciamento de estado (leitura).

---

## WebSockets

### Conectar ao Chat 🔒
Promove a conexão HTTP para WebSocket.

- **URL**: `GET /api/chat/ws?token={JWT_TOKEN}`
- **Protocolo**: `ws://` ou `wss://`
- **Autenticação**: Token JWT obrigatório via Query Parameter.

#### Comportamento:
- **Clientes**: Recebem mensagens enviadas para o seu `usuario_uuid`.
- **Lojas**: Funcionários, Entregadores e Admins recebem mensagens enviadas para as `loja_uuid` às quais estão vinculados.

#### Formato de Envio (Client -> Server):
```json
{
  "loja_uuid": "UUID",
  "usuario_uuid": "UUID",
  "texto": "String",
  "pedido_uuid": "UUID (Opcional)"
}
```

#### Formato de Recebimento (Server -> Client):
```json
{
  "uuid": "UUID",
  "pedido_uuid": "UUID | null",
  "loja_uuid": "UUID",
  "usuario_uuid": "UUID",
  "remetente_uuid": "UUID",
  "texto": "String",
  "lida": boolean,
  "criado_em": "ISO8601 Timestamp"
}
```

---

## HTTP Endpoints

### Listar Histórico por Pedido 🔒
Retorna todas as mensagens trocadas no contexto de um pedido específico.

- **Método**: `GET`
- **Rota**: `/api/chat/historico/pedido/{pedido_uuid}`
- **Resposta**: `200 OK` com `Array<MensagemChat>`

### Listar Histórico por Loja e Usuário 🔒
Retorna o histórico completo de conversas entre uma loja e um cliente.

- **Método**: `GET`
- **Rota**: `/api/chat/historico/loja/{loja_uuid}/usuario/{usuario_uuid}`
- **Resposta**: `200 OK` com `Array<MensagemChat>`

### Marcar Mensagem como Lida 🔒
Atualiza o status de uma mensagem específica para lida.

- **Método**: `PUT`
- **Rota**: `/api/chat/mensagens/{mensagem_uuid}/lida`
- **Resposta**: `204 No Content`

---

## Model: MensagemChat

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `uuid` | UUID | Identificador único da mensagem |
| `pedido_uuid` | UUID? | UUID do pedido associado (opcional) |
| `loja_uuid` | UUID | UUID da loja |
| `usuario_uuid` | UUID | UUID do cliente |
| `remetente_uuid` | UUID | UUID de quem enviou a mensagem |
| `texto` | String | Conteúdo da mensagem |
| `lida` | Boolean | Status de leitura |
| `criado_em` | Timestamp | Data/hora de criação |
