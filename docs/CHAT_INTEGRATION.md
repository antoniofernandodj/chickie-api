# 📑 Guia de Integração: Chat Real-time (Angular + Rust Axum)

Este documento descreve como conectar, enviar mensagens e recuperar o histórico do chat entre clientes e lojas no sistema Chickie.

## 1. Visão Geral da Arquitetura
O chat utiliza **WebSockets (WS)** para mensagens em tempo real e **HTTP** para busca de histórico e confirmações de leitura. O backend orquestra as mensagens via **Redis Pub/Sub**, o que permite que o frontend se conecte a qualquer instância do servidor em um ambiente escalonado (horizontal scaling).

---

## 2. Conexão WebSocket
A conexão é estabelecida via protocolo `ws://` (ou `wss://` em produção).

### URL de Conexão
```
ws://{{BASE_URL}}/api/chat/ws?token={{JWT_TOKEN}}
```
*   **token**: O JWT deve ser passado via Query Parameter. O backend validará o token no momento do handshake.

### Comportamento de Conexão:
- **Clientes**: Automaticamente inscritos em seu próprio canal `chat:usuario:{meu_uuid}`.
- **Lojas (Admins/Funcionários)**: Automaticamente inscritos nos canais de todas as lojas às quais possuem vínculo (onde são criadores, funcionários ou entregadores).

---

## 3. Estrutura das Mensagens (JSON)

### Enviar Mensagem (via WebSocket)
Para enviar uma mensagem, o frontend deve enviar um objeto JSON pelo socket:

```json
{
  "loja_uuid": "3668a649-...",
  "usuario_uuid": "7229b122-...",
  "texto": "Olá, meu pedido vai demorar?",
  "pedido_uuid": "f47ac10b-..." // Opcional, vincular a um pedido específico
}
```

### Receber Mensagem (via WebSocket)
O socket emitirá mensagens no seguinte formato quando houver nova atividade:

```json
{
  "uuid": "550e8400-e29b-...",
  "pedido_uuid": "f47ac10b-...",
  "loja_uuid": "3668a649-...",
  "usuario_uuid": "7229b122-...",
  "remetente_uuid": "7229b122-...", // Compara com seu UUID para saber se a mensagem é "minha" ou do "outro"
  "texto": "Olá, estamos preparando!",
  "lida": false,
  "criado_em": "2026-04-30T10:00:00Z"
}
```

---

## 4. Endpoints HTTP (Histórico e Ações)

### Listar Histórico por Pedido
Útil para chats dentro da tela de acompanhamento de um pedido.
- **Método**: `GET`
- **Rota**: `/api/chat/historico/pedido/{pedido_uuid}`
- **Auth**: Requer JWT (Bearer)

### Listar Histórico Geral (Loja <-> Cliente)
Útil para o SAC da loja ou chat geral do cliente.
- **Método**: `GET`
- **Rota**: `/api/chat/historico/loja/{loja_uuid}/usuario/{usuario_uuid}`
- **Auth**: Requer JWT (Bearer)

### Marcar como Lida
- **Método**: `PUT`
- **Rota**: `/api/chat/mensagens/{mensagem_uuid}/lida`

---

## 5. Implementação Sugerida (Angular Service)

Utilize o `webSocket` do **RxJS** para uma integração reativa:

```typescript
import { Injectable } from '@angular/core';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';
import { Observable, Subject } from 'rxjs';

// Nota: Defina as interfaces conforme os modelos acima
import { CreateMensagemRequest, MensagemChat } from './models';

@Injectable({ providedIn: 'root' })
export class ChatService {
  private socket$: WebSocketSubject<any>;
  private messagesSubject = new Subject<MensagemChat>();
  public messages$ = this.messagesSubject.asObservable();

  constructor() {}

  public connect(token: string): void {
    const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws';
    const url = `${protocol}://${window.location.host}/api/chat/ws?token=${token}`;

    this.socket$ = webSocket(url);

    this.socket$.subscribe({
      next: (msg: MensagemChat) => this.messagesSubject.next(msg),
      error: (err) => {
        console.error('WebSocket Error:', err);
        // Implementar lógica de reconexão aqui se necessário
      },
      complete: () => console.warn('WebSocket Connection Closed')
    });
  }

  public sendMessage(msg: CreateMensagemRequest): void {
    if (this.socket$) {
      this.socket$.next(msg);
    }
  }

  public close(): void {
    this.socket$?.complete();
  }
}
```

---

## 6. Dicas de UI/UX
1.  **Diferenciação de Lado**: Utilize o campo `remetente_uuid` da mensagem recebida para comparar com o UUID do usuário logado e renderizar o balão de fala à esquerda ou direita.
2.  **Reconexão**: Implementar um mecanismo de *exponential backoff* para reconectar o socket caso a conexão caia.
3.  **Scroll Automático**: Ao receber uma nova mensagem via `messages$`, role o container de mensagens para o final.
4.  **Indicador de Digitante**: (Opcional) Pode ser implementado enviando eventos do tipo `{ "typing": true }` via socket, embora não esteja na persistência do banco.

---

**Configuração do Ambiente:**
- O servidor requer a variável de ambiente `REDIS_URL` configurada.
- O CORS deve permitir conexões WebSocket da origem do frontend.
