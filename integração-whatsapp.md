# Integração Twillio

A especificação a seguir visa fornecer as informações necessárias para a integração do WhatsApp com o sistema de vendas da empresa. É destinado a tratar as requisições de vendas como uma interface de usuario em chat para a venda de produtos e
também como um sistema de notificação de atualização de status de pedidos.

## Provedor

O provedor para o sistema de whatsapp é a Twilio.
seu webhook foi configurado para enviar as mensagens do whatsapp para /webhook/whatsapp
e a confirmação to status de entrega da mensagem será enviado para /webhook/whatsapp/delivery-satatus

## API

### Exemplo de envio de mensagem iniciada por empresa

to_whatsapp="5521992784394"
from_whatsapp="14155238886"
content_id="HXb5b62575e6e4ff6129ad7c8efe1f983e"
path="https://api.twilio.com/2010-04-01/Accounts/${acount_sid}/Messages.json"
curl "${path}" -X POST \
    --data-urlencode "To=whatsapp:+{to_whatsapp}" \
    --data-urlencode "From=whatsapp:+{from_whatsapp}" \
    --data-urlencode "ContentSid={content_id}" \
    --data-urlencode 'ContentVariables={"1":"12/1","2":"3pm"}' \
    -u "${acount_sid}:${auth_token}"

### Exemplo de mensagem iniciada por usuario

text="Your appointment is coming up on July 21 at 3PM"
from_whatsapp="14155238886"
to_whatsapp="5521992784394"
path="https://api.twilio.com/2010-04-01/Accounts/${acount_sid}/Messages.json"
curl "{path}" -X POST \
--data-urlencode 'To=whatsapp:+{to_whatsapp}' \
--data-urlencode 'From=whatsapp:+{from_whatsapp}' \
--data-urlencode "Body={text}" \
-u "${acount_sid}:${auth_token}"

detalhes:

Especificação Técnica: Integração WhatsApp (Twilio) com Backend Axum/SQLx
Versão: 1.0
Objetivo: Integrar canal WhatsApp via Twilio ao sistema de vendas, permitindo que usuários autenticados e não autenticados (guest) interajam com o mesmo Domain Service para vendas e acompanhamento de pedidos.
1. Visão Geral e Princípios
1.1. Contexto
O backend é desenvolvido em Rust utilizando Axum como framework web e SQLx para acesso ao banco PostgreSQL.
O sistema já possui fluxo de autenticação JWT para usuários registrados.
Existe fluxo funcional de compra como guest, onde um token de pedido é gerado e armazenado no dispositivo do usuário para acompanhamento posterior.
O Domain Service é agnóstico ao canal de entrada e contém toda a lógica de negócio central.
1.2. Princípios de Design
Agnosticismo de Canal: O Domain Service não deve saber se a requisição veio da API web, mobile ou WhatsApp.
Segurança por Camadas: Validação na borda (webhook), resolução de identidade, e autorização no domínio.
Idempotência: Mensagens do WhatsApp podem ser reenviadas; o sistema deve evitar processamento duplicado.
Compatibilidade com Guest: Usuários não autenticados devem poder criar e acompanhar pedidos via WhatsApp usando o mesmo mecanismo de token já existente.
2. Arquitetura de Integração
2.1. Componentes Principais
Twilio Provider: Responsável por receber mensagens do WhatsApp e entregar ao webhook configurado, e por enviar mensagens iniciadas pela empresa.
Webhook Receiver (Axum): Endpoint que recebe eventos do Twilio, valida assinatura, parseia payload e delega processamento.
WhatsApp Adapter: Camada de tradução que converte mensagens do WhatsApp em comandos do domínio e respostas do domínio em mensagens formatadas para o WhatsApp.
Identity Resolver: Componente que determina se o remetente é um usuário autenticado, um guest com token válido, ou um anônimo.
Request Context Builder: Constrói um contexto padronizado contendo identidade, canal, permissões e metadados para o Domain Service.
Domain Service: Camada de negócio pura que processa comandos independentemente da origem.
Conversation State Manager: Gerencia estado efêmero da conversa (Redis) para manter contexto entre mensagens.
2.2. Fluxo de Dados

Usuário WhatsApp → Twilio → Webhook Axum → Validação de Assinatura
→ Identity Resolver → Request Context Builder → Domain Service
→ WhatsApp Adapter → Twilio API → Usuário WhatsApp

3. Modelo de Identidade e Autenticação
3.1. Tipos de Identidade
O sistema deve suportar três modos de identidade para requisições originadas do WhatsApp:
Identidade Autenticada
Representada por um user_id UUID.
Resolvida quando o número de telefone do remetente está vinculado e verificado em uma conta de usuário existente.
Permissões derivadas do perfil do usuário (roles).
Identidade Guest
Representada por um guest_token UUID.
Resolvida quando o usuário fornece um token de pedido válido na mensagem ou quando há um token associado ao número no estado de conversa.
Permissões limitadas a operações no próprio pedido guest.
Identidade Anônima
Sem identificador persistente.
Permite apenas operações públicas: listar produtos, iniciar novo pedido guest, solicitar ajuda.
Ao criar um pedido, um guest_token é gerado e retornado ao usuário para acompanhamento futuro.
3.2. Resolução de Identidade no Webhook
Ao receber uma mensagem, o sistema deve executar a seguinte lógica de resolução:
Extrair o número de telefone do remetente no formato E.164 a partir do payload do Twilio.
Consultar o banco de dados para verificar se o número está vinculado a um usuário autenticado e verificado. Se sim, resolver como Identidade Autenticada.
Caso contrário, analisar o corpo da mensagem em busca de um token de pedido (padrão UUID ou código alfanumérico). Se encontrado e válido em guest_orders, resolver como Identidade Guest.
Caso ainda não resolvido, consultar o estado de conversa no Redis para verificar se há um guest_token associado ao número. Se existir, resolver como Identidade Guest.
Se nenhuma das etapas anteriores resultar em identidade, classificar como Identidade Anônima.
3.3. Vínculo de WhatsApp para Usuários Autenticados
Para que um usuário autenticado possa usar o WhatsApp como canal:
O usuário solicita vínculo através da interface web ou mobile.
O sistema gera um código OTP de seis dígitos, armazena seu hash com timestamp de expiração e envia o código via Twilio para o número informado.
O usuário responde ao WhatsApp com o código recebido.
O webhook valida o código, marca o vínculo como verificado e associa permanentemente o número ao user_id.
A partir deste momento, mensagens deste número resolvem automaticamente para a identidade autenticada do usuário.
4. Modelo de Dados (SQLx)
4.1. Tabela de Vínculo WhatsApp-Usuário
Nome: user_whatsapp_bindings
Colunas:
id: UUID, chave primária
user_id: UUID, chave estrangeira para users, não nulo
phone_number: texto, formato E.164, não nulo
verified: boolean, padrão falso
verification_code_hash: texto, hash do OTP
verification_expires_at: timestamp com timezone
created_at, updated_at: timestamps com timezone
Restrições:
Único por combinação de user_id e phone_number
Índices:
phone_number para busca rápida no webhook
user_id para consultas de vínculo por usuário
4.2. Tabela de Pedidos Guest (Existente - Adaptação)
Nome: guest_orders
Colunas relevantes:
guest_token: UUID, único, usado como identificador público do pedido
phone_number: texto opcional, para facilitar vínculo futuro
status: enum de status de pedido
items: JSONB com detalhes dos itens
total_cents: inteiro, valor total em centavos
Índices:
guest_token para busca direta
phone_number para associação com WhatsApp
4.3. Tabela de Estado de Conversa
Nome: whatsapp_conversations
Propósito: armazenar estado efêmero da interação por número de telefone
Colunas:
phone_number: texto, chave de busca
guest_token: UUID opcional, referência a guest_orders
user_id: UUID opcional, referência a users
state: JSONB, armazena contexto da conversa (etapa atual, carrinho temporário, etc.)
last_message_sid: texto único, para controle de idempotência
expires_at: timestamp, para limpeza automática
Estratégia: TTL de 24 horas para entradas inativas; cleanup periódico.
4.4. Tabela de Mensagens Processadas (Idempotência)
Nome: processed_twilio_messages
Colunas:
message_sid: texto, chave primária (SID único do Twilio)
processed_at: timestamp com timezone, padrão agora
Propósito: evitar processamento duplicado de webhooks reenviados pelo Twilio
Estratégia: job de limpeza remove registros com mais de sete dias
5. Fluxos de Negócio
5.1. Guest: Criar Novo Pedido via WhatsApp
Usuário envia mensagem expressando intenção de compra (exemplo: "Quero comprar dois unidades do produto X").
Webhook valida assinatura Twilio e extrai número e corpo da mensagem.
Identity Resolver classifica como Identidade Anônima (sem vínculo nem token).
WhatsApp Adapter inicia novo estado de conversa no Redis, associando um guest_token temporário ao número.
Domain Service cria entrada em guest_orders com status "CARRINHO", itens parseados e guest_token gerado.
Resposta formatada é enviada via Twilio informando:
Confirmação de criação do pedido
Código/token do pedido para acompanhamento futuro
Instruções para próximos passos (exemplo: "Digite status SEU_TOKEN para acompanhar")
Estado de conversa é atualizado com etapa atual e TTL renovado.
5.2. Guest: Acompanhar Pedido Existente
Usuário envia mensagem contendo comando de status e token (exemplo: "status ABC123").
Webhook extrai token da mensagem via parser de comandos.
Identity Resolver valida token em guest_orders e resolve como Identidade Guest.
Domain Service consulta status do pedido e informações relevantes (etapa atual, previsão de entrega).
Resposta formatada é enviada via Twilio com:
Status atual do pedido
Informações logísticas quando aplicável
Opções de ação contextual (exemplo: "Responder CANCELAR para cancelar")
Estado de conversa é atualizado para refletir a interação recente.
5.3. Usuário Autenticado: Interação Completa
Número do usuário já está vinculado e verificado em user_whatsapp_bindings.
Identity Resolver resolve automaticamente como Identidade Autenticada.
Domain Service recebe contexto com user_id e permissões completas.
Usuário pode:
Listar produtos com personalização baseada em histórico
Criar pedidos vinculados à sua conta
Acompanhar todos os seus pedidos (autenticados e guest anteriores)
Gerenciar preferências e endereço de entrega
Respostas podem incluir informações personalizadas e ações com maior privilégio.
5.4. Vinculação de WhatsApp por Usuário Autenticado
Usuário inicia processo de vínculo via interface web/mobile.
Backend gera OTP, armazena hash com expiração e envia via Twilio.
Usuário responde no WhatsApp com o código.
Webhook valida código, atualiza vínculo para verificado.
Confirmação é enviada ao usuário e estado de conversa é inicializado.
6. Comandos e Parser de Linguagem Natural
6.1. Comandos Suportados
O sistema deve reconhecer os seguintes comandos via mensagem de texto:
Listar Produtos: palavras-chave como "lista", "produtos", "catálogo", opcionalmente com categoria
Criar Pedido: expressões como "quero comprar", "adicionar", seguido de quantidade e nome do produto
Acompanhar Pedido: "status", "pedido", "acompanhar", seguido de token ou número do pedido
Finalizar Compra: "finalizar", "checkout", "comprar agora"
Cancelar Pedido: "cancelar", seguido de identificador do pedido (apenas se permitido pelo status)
Ajuda: "ajuda", "suporte", "como funciona"
Vincular Conta: "conectar conta", "vincular whatsapp"
6.2. Estratégia de Parse
Converter mensagem para minúsculas e remover acentos para normalização.
Aplicar regras de correspondência por prefixo e palavras-chave em ordem de especificidade.
Extrair entidades como tokens UUID, números de pedido, quantidades e nomes de produto usando expressões regulares.
Para comandos ambíguos, retornar mensagem de esclarecimento com opções numeradas para o usuário escolher.
Manter histórico de intenções no estado de conversa para permitir referência contextual (exemplo: "quero esse" refere-se ao último produto listado).
7. Segurança e Conformidade
7.1. Validação de Webhook
Todo webhook recebido deve validar o cabeçalho X-Twilio-Signature usando HMAC-SHA256 com o auth token da conta Twilio.
A validação deve considerar a URL completa do endpoint e os parâmetros da requisição conforme documentação oficial do Twilio.
Requisições com assinatura inválida devem ser rejeitadas com status quatrocentos e um.
7.2. Proteção contra Replay e Duplicação
Cada mensagem do Twilio possui um MessageSid único.
Antes de processar, consultar a tabela processed_twilio_messages ou Redis para verificar se o SID já foi processado.
Se já processado, retornar sucesso imediatamente sem reexecutar lógica de negócio.
Registrar o SID como processado apenas após conclusão bem-sucedida do fluxo.
7.3. Controle de Acesso por Identidade
O Domain Service deve validar, para cada operação, se a identidade no RequestContext tem permissão para acessar o recurso solicitado.
Usuários autenticados só acessam seus próprios pedidos, exceto se possuírem role de suporte.
Guests só acessam pedidos correspondentes ao seu guest_token.
Anônimos só acessam operações públicas.
7.4. Proteção de Dados Sensíveis
Nunca solicitar ou transmitir senhas, CPF, dados completos de cartão ou tokens de autenticação via mensagens do WhatsApp.
Para operações sensíveis, enviar links seguros que redirecionam para interfaces web/mobile autenticadas.
Logs devem ofuscar números de telefone e tokens, registrando apenas hashes ou identificadores internos.
7.5. Rate Limiting e Prevenção de Abuso
Aplicar limite de requisições por número de telefone (exemplo: dez mensagens por minuto) usando Redis.
Bloquear temporariamente números que excederem limite de tentativas falhas de autenticação ou comandos inválidos.
Implementar fila assíncrona para processamento de mensagens, permitindo controle de throughput e retry com backoff.
7.6. Conformidade com LGPD
Coletar consentimento explícito antes de vincular número de WhatsApp a uma conta de usuário.
Oferecer endpoint para o usuário revogar vínculo e solicitar exclusão de dados pessoais associados ao número.
Criptografar dados pessoais em repouso e limitar retenção de logs de conversa ao necessário para operação e auditoria.
8. Gerenciamento de Estado de Conversa
8.1. Propósito
Manter contexto entre mensagens sequenciais do mesmo usuário (exemplo: seleção de itens, confirmação de endereço).
Armazenar guest_token temporário associado ao número para facilitar acompanhamento sem exigir digitação repetida do token.
Controlar etapa atual do fluxo (exemplo: "selecionando itens", "confirmando pagamento") para respostas contextualizadas.
8.2. Estratégia de Armazenamento
Usar Redis para estado efêmero com TTL configurável (padrão: vinte e quatro horas de inatividade).
Estrutura de chave: "whatsapp:conversation:{phone_number}".
Valor: objeto JSON contendo:
identity_type: "authenticated", "guest" ou "anonymous"
identifier: user_id ou guest_token quando aplicável
current_step: string descrevendo etapa do fluxo
context: objeto com dados temporários (carrinho, endereço parcial, etc.)
last_interaction: timestamp da última mensagem processada
8.3. Atualização e Limpeza
Atualizar last_interaction a cada mensagem processada e renovar TTL.
Job periódico remove entradas expiradas para liberar memória.
Ao finalizar um fluxo (pedido criado, vínculo concluído), limpar estado temporário exceto vínculo permanente quando aplicável.
9. Integração com Twilio API
9.1. Envio de Mensagens Iniciadas pela Empresa
Usar endpoint Messages da API REST do Twilio.
Parâmetros obrigatórios:
To: número de destino no formato whatsapp:+E.164
From: número da conta Twilio configurado para WhatsApp
Body: texto da mensagem ou referência a ContentSid para templates aprovados
Autenticação: Basic Auth com account_sid e auth_token.
Para mensagens ricas (imagens, botões), usar ContentSid e ContentVariables conforme configuração de templates na console do Twilio.
9.2. Webhook de Entrega (Delivery Status)
Endpoint configurado: /webhook/whatsapp/delivery-status
Recebe atualizações sobre status de entrega das mensagens enviadas.
Atualizar tabela de histórico de notificações para auditoria e métricas.
Não disparar ações de negócio baseadas apenas em status de entrega; usar apenas para monitoramento.
9.3. Tratamento de Erros da API Twilio
Implementar retry com backoff exponencial para falhas transitórias (código quatro e nove, cinco e zero).
Logar erros persistentes com contexto completo para investigação.
Para falhas críticas (autenticação, número inválido), notificar equipe de operações e interromper tentativas.
10. Observabilidade e Monitoramento
10.1. Logs Estruturados
Usar tracing com campos estruturados: channel, identity_type, message_sid, phone_number_hash, command, duration.
Níveis de log:
INFO: fluxos bem-sucedidos, criação de pedidos, vínculos concluídos
WARN: comandos não reconhecidos, tentativas de acesso não autorizado, rate limit atingido
ERROR: falhas de validação, erros de banco, exceções não tratadas
10.2. Métricas Principais
Contador de mensagens recebidas por tipo de identidade
Tempo médio de processamento por webhook
Taxa de sucesso de envio de mensagens via Twilio
Número de pedidos criados via WhatsApp (segmentado por autenticado/guest)
Taxa de conversão de guest para usuário autenticado após vínculo
10.3. Alertas
Alertar se taxa de erro de validação de assinatura ultrapassar limite (possível ataque).
Alertar se latência média de processamento exceder threshold (possível gargalo).
Alertar se volume de mensagens cair abruptamente (possível falha no webhook ou Twilio).
11. Considerações de Implantação
11.1. Variáveis de Ambiente
TWILIO_ACCOUNT_SID: identificador da conta Twilio
TWILIO_AUTH_TOKEN: token para autenticação na API
TWILIO_WHATSAPP_NUMBER: número Twilio configurado para WhatsApp, em formato E.164
WEBHOOK_URL: URL pública do endpoint de webhook (para validação de assinatura)
REDIS_URL: conexão para cache e estado de conversa
DATABASE_URL: conexão PostgreSQL para SQLx
11.2. Migrações de Banco
Executar migrações SQLx em ordem sequencial no startup da aplicação.
Garantir que tabelas de vínculo, estado de conversa e mensagens processadas sejam criadas antes do webhook aceitar tráfego.
11.3. Escalabilidade
Webhook deve ser stateless; todo estado persistente vai para banco ou Redis.
Usar fila assíncrona (exemplo: Redis Streams ou SQS) para processamento de comandos pesados, mantendo resposta rápida ao Twilio.
Horizontalizar instâncias do serviço Axum conforme volume de mensagens.
12. Critérios de Aceite
Webhook valida assinatura Twilio e rejeita requisições inválidas
Identity Resolver distingue corretamente entre autenticado, guest e anônimo
Guest pode criar pedido e receber token para acompanhamento posterior
Guest pode acompanhar pedido usando token via mensagem de texto
Usuário autenticado vinculado acessa funcionalidades completas via WhatsApp
Domain Service é chamado com RequestContext padronizado, independente do canal
Idempotência evita processamento duplicado de mensagens reenviadas
Estado de conversa é mantido no Redis com TTL e limpeza automática
Logs estruturados e métricas estão disponíveis para monitoramento
Documentação de comandos suportados é enviada ao usuário ao digitar "ajuda"
13. Referências Externas
Documentação de Segurança do Twilio: validação de assinaturas de webhook
Guia de Formatos de Número E.164 para WhatsApp Business API
Melhores práticas de design de Domain-Driven Services agnósticos ao canal
Diretrizes da LGPD para tratamento de dados pessoais em canais de comunicação