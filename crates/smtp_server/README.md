# smtp-server

Servidor SMTP em Rust com suporte a autenticação, TLS/SSL, relay e fila de entregas com retry automático.

## Funcionalidades

- **Protocolo SMTP completo**: HELO/EHLO, MAIL FROM, RCPT TO, DATA, RSET, NOOP, QUIT
- **Autenticação**: AUTH PLAIN e AUTH LOGIN com senhas em hash SHA-256
- **TLS/SSL**: STARTTLS (portas 25/587) e TLS implícito (porta 465)
- **Relay**: integração com SendGrid, MailerSend, Gmail ou qualquer SMTP externo
- **Fila de entregas**: retry exponencial automático (1min → 5min → 30min → 2h → 6h)
- **Logs estruturados**: via `tracing` com suporte a JSON e filtragem por nível
- **Validação**: RFC 2822 para e-mails, validação de endereços e tamanho de mensagem

## Instalação

### Pré-requisitos

```bash
# Instala Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Para gerar certificados de desenvolvimento
openssl req -x509 -newkey rsa:4096 \
  -keyout certs/key.pem -out certs/cert.pem \
  -days 365 -nodes -subj "/CN=localhost"
```

### Rodando localmente

```bash
# Clone e entre no diretório
cd smtp-server

# Configure o ambiente
cp .env.example .env
# Edite o .env conforme necessário

# Compile e execute
cargo run --release
```

### Com Docker

```bash
# Build e sobe todos os serviços
docker-compose up -d

# Acompanha os logs
docker-compose logs -f smtp-server
```

## Configuração

Todas as configurações são feitas via variáveis de ambiente (arquivo `.env`):

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `SMTP_HOSTNAME` | `localhost` | Hostname no greeting SMTP |
| `SMTP_PORT` | `2525` | Porta SMTP principal |
| `SMTPS_PORT` | `4465` | Porta SMTPS (TLS implícito) |
| `SMTP_SUBMISSION_PORT` | `2587` | Porta de submission |
| `REQUIRE_AUTH` | `true` | Exige autenticação para envio |
| `SMTP_USERNAME` | — | Usuário padrão |
| `SMTP_PASSWORD` | — | Senha do usuário padrão |
| `TLS_CERT_PATH` | `certs/cert.pem` | Certificado TLS |
| `TLS_KEY_PATH` | `certs/key.pem` | Chave privada TLS |
| `MAX_MESSAGE_SIZE` | `10485760` | Tamanho máximo (bytes) |
| `RELAY_HOST` | — | Host do relay SMTP externo |
| `RELAY_PORT` | `587` | Porta do relay |
| `RELAY_USERNAME` | — | Usuário do relay |
| `RELAY_PASSWORD` | — | Senha do relay |
| `RUST_LOG` | `info` | Nível de log |

## Testando o servidor

### Com telnet

```bash
telnet localhost 2525

# Sequência SMTP:
EHLO cliente.teste
AUTH PLAIN AGFkbWluQGxvY2FsaG9zdABhZG1pbjEyMw==
MAIL FROM:<remetente@exemplo.com>
RCPT TO:<destinatario@exemplo.com>
DATA
Subject: Teste
From: remetente@exemplo.com
To: destinatario@exemplo.com

Olá! Este é um e-mail de teste.
.
QUIT
```

A string base64 `AGFkbWluQGxvY2FsaG9zdABhZG1pbjEyMw==` corresponde a `\0admin@localhost\0admin123`.

### Com swaks (Swiss Army Knife SMTP)

```bash
# Instala swaks
apt install swaks  # ou brew install swaks

# Envia um e-mail de teste
swaks \
  --server localhost:2525 \
  --auth PLAIN \
  --auth-user admin@localhost \
  --auth-password admin123 \
  --from remetente@exemplo.com \
  --to destinatario@exemplo.com \
  --header "Subject: Teste via swaks" \
  --body "Olá do smtp-server!"
```

## Integração com relay externo

### MailerSend

```env
RELAY_HOST=smtp.mailersend.net
RELAY_PORT=587
RELAY_USERNAME=seu-usuario@mailersend.net
RELAY_PASSWORD=sua-api-key
```

### SendGrid

```env
RELAY_HOST=smtp.sendgrid.net
RELAY_PORT=587
RELAY_USERNAME=apikey
RELAY_PASSWORD=SG.sua-chave-aqui
```

### Gmail (com senha de app)

```env
RELAY_HOST=smtp.gmail.com
RELAY_PORT=587
RELAY_USERNAME=seu@gmail.com
RELAY_PASSWORD=senha-de-app-16-chars
```

## Arquitetura

```
src/
├── main.rs          — Entry point, inicialização de serviços
├── smtp/mod.rs      — State machine do protocolo SMTP
├── auth/mod.rs      — UserStore com hashing de senhas
├── handlers/mod.rs  — MailMessage: parsing RFC 2822
├── queue/mod.rs     — Fila de entrega com retry exponencial
└── tls/mod.rs       — Configuração TLS via rustls
```

## Deploy em produção

Em produção, rode nas portas 25, 587 e 465. Para não precisar de root:

```bash
# Permite binding em portas baixas sem root (Linux)
sudo setcap 'cap_net_bind_service=+ep' ./target/release/smtp-server

# Ou use um redirecionamento de porta via iptables:
sudo iptables -t nat -A PREROUTING -p tcp --dport 25 -j REDIRECT --to-port 2525
```

Para certificados de produção, use Let's Encrypt:

```bash
certbot certonly --standalone -d smtp.seudominio.com
# Aponte TLS_CERT_PATH e TLS_KEY_PATH para os arquivos do certbot
```
