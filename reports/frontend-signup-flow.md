# Relatório: Adaptação do Frontend ao Novo Fluxo de Signup

> **Data:** 2026-04-28  
> **Contexto:** O endpoint `POST /api/auth/signup` foi alterado. O usuário não é mais criado imediatamente — o cadastro agora exige confirmação por email antes da conta existir no banco.

---

## 1. Visão Geral da Mudança

### Fluxo Antigo

```
Tela de Cadastro
     │
     ▼
POST /api/auth/signup
     │
     ▼
← { usuario: {...} }   ← conta criada imediatamente
     │
     ▼
Login automático (JWT)
     │
     ▼
App
```

### Fluxo Novo

```
Tela de Cadastro
     │
     ▼
POST /api/auth/signup
     │
     ▼
← { message: "Email enviado para ..." }
     │
     ▼
Tela de "Verifique seu email"
     │
     ▼  (usuário clica no link do email)
     │
     ▼
GET /api/auth/confirmar-email?token=<token>
     │
     ▼
← { token: "...", usuario: {...} }
     │
     ▼
App (usuário logado automaticamente)
```

---

## 2. Telas a Construir / Modificar

### 2.1 Tela de Cadastro — **MODIFICAR**

A tela já existe. As mudanças necessárias são no comportamento pós-submissão:

**Antes:** ao receber sucesso, salvar JWT e redirecionar para o app.  
**Agora:** ao receber `202 Accepted`, redirecionar para a **Tela de Verificação Pendente**.

Não há mudança nos campos do formulário. O campo `cpf` já é obrigatório (se ainda não estiver no formulário, adicionar).

---

### 2.2 Tela de Verificação Pendente — **CRIAR**

Exibida logo após o signup bem-sucedido. O usuário ainda não tem conta ativa.

**Objetivo:** informar o usuário que um email foi enviado e orientá-lo a clicar no link.

**Conteúdo da tela:**
- Ícone de email / envelope
- Título: *"Verifique seu email"*
- Subtítulo com o email usado: *"Enviamos um link de confirmação para **joao@email.com**"*
- Aviso de validade: *"O link expira em 1 hora."*
- Botão secundário: **"Reenviar email"** (chama novamente `POST /api/auth/signup` com os mesmos dados — ver seção 4.1)
- Link: *"Usar outro email"* → volta para a tela de cadastro

**Estado a guardar (temporariamente, ex: `sessionStorage`):**
```json
{
  "email": "joao@email.com",
  "nome": "João Silva"
}
```
Necessário para exibir o email na tela e para o botão de reenvio.

---

### 2.3 Tela de Confirmação de Email — **CRIAR**

Esta tela é acessada quando o usuário clica no link do email. O link tem o formato:

```
https://seuapp.com/auth/confirmar-email?token=abc123...
```

O frontend deve ter uma rota que captura esse `token` da URL, chama a API e faz o login automático.

**Fluxo desta tela:**

```
Usuário clica no link do email
        │
        ▼
Rota /auth/confirmar-email?token=<token>
        │
        ▼ (ao montar o componente)
GET /api/auth/confirmar-email?token=<token>
        │
        ├── Sucesso (200) ──────────────────────────────┐
        │                                               ▼
        │                              Salvar JWT → Redirecionar para o app
        │
        ├── Erro 400 (token inválido/expirado) ─────────┐
        │                                               ▼
        │                              Exibir mensagem de erro
        │                              + botão "Fazer cadastro novamente"
        │
        └── Loading ──────────────────────────────────┐
                                                      ▼
                                         Spinner + "Confirmando seu cadastro..."
```

**Estados da tela:**

| Estado | UI |
|--------|----|
| `loading` | Spinner: *"Confirmando seu cadastro..."* |
| `success` | Ícone ✅ + *"Cadastro confirmado! Redirecionando..."* |
| `error` | Ícone ❌ + mensagem do erro + botão para refazer cadastro |

---

## 3. Endpoints

### 3.1 `POST /api/auth/signup` — Iniciar Cadastro

**URL:** `POST /api/auth/signup`  
**Auth:** Não requerida

**Request Body:**
```json
{
  "nome": "João Silva",
  "username": "joaosilva",
  "senha": "minhasenha123",
  "email": "joao@email.com",
  "celular": "(11) 99999-9999",
  "cpf": "123.456.789-09",
  "auth_method": "email",
  "classe": "cliente"
}
```

| Campo | Tipo | Obrigatório | Observação |
|-------|------|-------------|------------|
| `nome` | string | ✅ | Nome completo |
| `username` | string | ✅ | Sem espaços, único |
| `senha` | string | ✅ | — |
| `email` | string | ✅ | Único no sistema |
| `celular` | string | ✅ | API filtra automaticamente, só dígitos chegam ao banco. Pode enviar `"(11) 99999-9999"` |
| `cpf` | string | ✅ | API filtra automaticamente. Pode enviar `"123.456.789-09"` |
| `auth_method` | string | ✅ | Sempre enviar `"email"` |
| `classe` | string | ❌ | Default: `"cliente"`. Outros valores: `"administrador"` |

**Response `202 Accepted` (sucesso):**
```json
{
  "message": "Email de verificação enviado para joao@email.com. Você tem 1 hora para confirmar."
}
```

**Response `400 Bad Request` (erros de validação):**
```json
{ "error": "CPF inválido. Informe os 11 dígitos sem pontuação ou verifique os dígitos verificadores." }
```
```json
{ "error": "Email já cadastrado." }
```
```json
{ "error": "Username já cadastrado." }
```
```json
{ "error": "Celular já cadastrado." }
```

> **Importante:** o status de sucesso agora é `202`, não `200` nem `201`. Verificar se o cliente HTTP trata `2xx` como sucesso (geralmente sim, mas vale confirmar).

---

### 3.2 `GET /api/auth/confirmar-email` — Confirmar Cadastro

**URL:** `GET /api/auth/confirmar-email?token=<token>`  
**Auth:** Não requerida

**Query Parameters:**

| Parâmetro | Tipo | Obrigatório |
|-----------|------|-------------|
| `token` | string | ✅ |

**Response `200 OK` (sucesso):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "usuario": {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "nome": "João Silva",
    "username": "joaosilva",
    "email": "joao@email.com",
    "celular": "11999999999",
    "cpf": "12345678909",
    "classe": "cliente",
    "ativo": true,
    "bloqueado": false,
    "passou_pelo_primeiro_acesso": false,
    "criado_em": "2026-04-28T14:00:00Z",
    "atualizado_em": "2026-04-28T14:00:00Z",
    "modo_de_cadastro": "email",
    "senha_hash": "...",
    "marcado_para_remocao": null,
    "deletado": false,
    "asaas_customer_id": null
  }
}
```

**Response `400 Bad Request` (token inválido ou expirado):**
```json
{ "error": "Token inválido ou expirado. Faça o cadastro novamente." }
```

**Response `400 Bad Request` (parâmetro ausente):**
```json
{ "error": "Parâmetro 'token' não informado." }
```

> **Ação ao receber `200`:** salvar `token` (o JWT) no storage do app e usar como `Bearer` nos requests autenticados. O campo `usuario` pode ser salvo no estado global.

---

### 3.3 `POST /api/auth/login` — Login (sem mudanças)

Este endpoint **não mudou**, mas fica documentado para referência. Usuários confirmados fazem login normalmente.

**URL:** `POST /api/auth/login`  
**Auth:** Não requerida

**Request Body:**
```json
{
  "identifier": "joao@email.com",
  "senha": "minhasenha123"
}
```

> `identifier` aceita email, username ou celular.

**Response `200 OK`:**
```json
{
  "access_token": "eyJhbGci...",
  "token_type": "Bearer",
  "usuario": {
    "uuid": "...",
    "nome": "João Silva",
    "username": "joaosilva",
    "email": "joao@email.com",
    "classe": "cliente",
    "ativo": true,
    "bloqueado": false
  }
}
```

> Note: este endpoint retorna `access_token`, enquanto o `confirmar-email` retorna `token`. Padronizar o consumo desses campos no cliente.

---

## 4. Casos Especiais a Tratar

### 4.1 Reenvio de Email

O backend não tem um endpoint dedicado de reenvio. Para reenviar, o frontend deve **chamar novamente `POST /api/auth/signup`** com os mesmos dados do formulário.

Comportamento esperado:
- Se o pré-cadastro anterior ainda não expirou, ele é **substituído** (o banco usa `ON CONFLICT ... DO UPDATE`)
- Um novo token e novo email são gerados
- O link anterior fica inválido (novo token sobrescreve o antigo)

**Implicação de UX:** manter os dados do formulário em memória (ex: `sessionStorage`) até o cadastro ser confirmado, para que o botão "Reenviar" funcione sem pedir tudo de novo.

**Sugestão:** adicionar um cooldown de 60 segundos no botão de reenvio para evitar spam.

---

### 4.2 Token Expirado (1 hora)

Se o usuário clicar no link após 1 hora, a API retorna `400`. A tela de confirmação deve:

1. Detectar o erro `"Token inválido ou expirado"`
2. Exibir mensagem clara: *"Este link expirou. Faça o cadastro novamente."*
3. Botão: **"Cadastrar novamente"** → redireciona para `/signup`

---

### 4.3 Usuário Clica no Link em Outro Dispositivo / Navegador

O link do email é um `GET` sem cookies ou sessão — funciona em qualquer navegador. Ao confirmar, o frontend recebe o JWT e pode salvar normalmente. Não há dependência de sessão do dispositivo original.

---

### 4.4 Usuário Já Confirmado Clica no Link Novamente

O token é removido do cache após a confirmação. Se clicar novamente, a API retorna `400` com *"Token inválido ou expirado"*. Tratar igual ao caso de token expirado.

---

### 4.5 Email Não Chegou / Caixa de Spam

Orientar o usuário na **Tela de Verificação Pendente**:
- *"Não recebeu? Verifique a caixa de spam."*
- Botão de reenvio (ver 4.1)

---

## 5. Diagrama de Navegação

```
/signup ──────────────────────────────────────────────────────┐
    │                                                          │
    │ POST /api/auth/signup                                    │
    │ ├── 202 Accepted                                         │
    │ │       └──► /signup/verificar-email                     │
    │ │                  │                                     │
    │ │                  │ Reenviar email                      │
    │ │                  │ └──► POST /api/auth/signup          │
    │ │                  │            └──► mesmo estado        │
    │ │                  │                                     │
    │ │                  │ Usar outro email                    │
    │ │                  └──────────────────────────────────┐  │
    │ │                                                     ▼  ▼
    │ └── 400 Bad Request                                  /signup
    │         └──► exibir erro inline no formulário
    │
    │
/auth/confirmar-email?token=<token>   ← link clicado no email
    │
    │ GET /api/auth/confirmar-email?token=<token>
    │ ├── 200 OK
    │ │       └──► Salvar JWT → /home (ou onboarding)
    │ └── 400 Bad Request
    │         └──► /signup (com mensagem "link expirado")
```

---

## 6. Resumo das Mudanças por Componente

| Componente | Ação | Prioridade |
|------------|------|------------|
| Tela de Cadastro | Tratar `202` como sucesso (não `200`/`201`). Redirecionar para verificação pendente em vez de logar. | Alta |
| Tela de Cadastro | Garantir que o campo `cpf` está no formulário | Alta |
| Tela de Verificação Pendente | Criar do zero | Alta |
| Tela de Confirmação de Email | Criar rota `/auth/confirmar-email?token=` que chama a API ao montar | Alta |
| Storage do app | `confirmar-email` retorna `token` (não `access_token`) — mapear corretamente ao salvar o JWT | Alta |
| Fluxo de reenvio | Reutilizar `POST /api/auth/signup` com os dados salvos | Média |
| Tratamento de erro de token expirado | Mensagem clara + botão para refazer cadastro | Média |
