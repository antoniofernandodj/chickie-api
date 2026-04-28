  O que foi feito

  Fluxo

  1. POST /api/auth/signup → valida dados, faz hash da senha, salva pré-cadastro em cache PostgreSQL (TTL 1h) e envia email de verificação → retorna 202
  Accepted
  2. GET /api/auth/confirmar-email?token=<token> → valida token, cria usuário em banco, remove do cache, retorna JWT + dados do usuário

  Camadas criadas (Clean Architecture)

  ┌────────────────┬──────────────────────────────────────────────────────────────┐
  │     Camada     │                           Arquivo                            │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Migration      │ migrations/0012_pre_cadastro.sql                             │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Port           │ crates/core/src/ports/pre_cadastro_port.rs                   │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Port           │ crates/core/src/ports/email_port.rs                          │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Repository     │ crates/core/src/repositories/pre_cadastro_repository.rs      │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Service        │ crates/core/src/services/email_service.rs                    │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Template       │ crates/core/src/templates/verificacao_email.html (Tera HTML) │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Service update │ UsuarioService ganhou iniciar_cadastro e confirmar_cadastro  │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Handler update │ criar_usuario.rs chama iniciar_cadastro                      │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Handler novo   │ confirmar_cadastro.rs                                        │
  ├────────────────┼──────────────────────────────────────────────────────────────┤
  │ Router         │ GET /api/auth/confirmar-email adicionado                     │
  └────────────────┴──────────────────────────────────────────────────────────────┘

  Variáveis de ambiente necessárias

  ┌──────────────────────┬────────────────────────────────────────────────────────────────┐
  │       Variável       │                           Descrição                            │
  ├──────────────────────┼────────────────────────────────────────────────────────────────┤
  │ MAILERSEND_API_TOKEN │ Token da API MailerSend                                        │
  ├──────────────────────┼────────────────────────────────────────────────────────────────┤
  │ EMAIL_FROM           │ Email remetente (ex: info@seudominio.com)                      │
  ├──────────────────────┼────────────────────────────────────────────────────────────────┤
  │ APP_BASE_URL         │ URL base para o link no email (default: http://localhost:3000) │
  └──────────────────────┴────────────────────────────────────────────────────────────────┘

  ▎ Se MAILERSEND_API_TOKEN não estiver configurado, o serviço loga um warning e retorna erro apenas se alguém tentar enviar — não trava o startup.

✻ Cooked for 10m 37s

※ recap: Implemented email verification signup flow: POST /api/auth/signup now stores a pre-registration in PostgreSQL cache and sends a MailerSend
  verification email, while GET /api/auth/confirmar-email?token= completes registration. Next: set MAILERSEND_API_TOKEN, EMAIL_FROM, and APP_BASE_URL env vars
   and test the flow end-to-end. (disable recaps in /config)