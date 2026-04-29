# FEATURES PENDENTES — Chickie API

> Análise arquitetural profunda das funcionalidades faltantes para que o Chickie atinja paridade com sistemas de delivery completos (iFood, UberEats, Rappi, DoorDash).
>
> **Data da análise:** 2026-04-29
> **Versão da API analisada:** branch `main-api`
> **Total de features identificadas:** 89 features distribuídas em 9 categorias

---

## Sumário Executivo

O Chickie API hoje cobre o **fluxo essencial de criação e ciclo de vida de pedidos** com uma arquitetura limpa e bem segmentada (Hexagonal + Ports & Adapters). Entretanto, falta um conjunto significativo de funcionalidades que separam um MVP funcional de um produto de delivery completo.

As lacunas mais críticas identificadas são:

1. **Fluxo completo do entregador** — não existem painéis, geolocalização, aceite de entrega, repasse, ranking
2. **Rastreamento de pedido em tempo real** — cliente não sabe onde está seu pedido
3. **Pagamentos integrados** — não há gateway, split, repasse, antifraude, reembolso
4. **Notificações** — push, SMS, email transacional, eventos
5. **Operacional/SLA** — não há cálculo de ETA, área de entrega, taxa dinâmica, fila de pedidos
6. **Analytics** — sem dashboards, métricas de loja, relatórios financeiros
7. **Catálogo avançado** — variações de produto, combos, estoque, fora de horário

---

## Tabela Resumo por Categoria

| Categoria                       | Features | Crítico | Alto | Médio | Baixo |
|---------------------------------|----------|---------|------|-------|-------|
| 1. Fluxo do Entregador          | 12       | 5       | 4    | 2     | 1     |
| 2. Rastreamento de Pedido       | 8        | 3       | 3    | 2     | 0     |
| 3. Pagamentos & Financeiro      | 14       | 6       | 5    | 2     | 1     |
| 4. Notificações                 | 9        | 3       | 4    | 2     | 0     |
| 5. Experiência do Cliente       | 13       | 2       | 5    | 5     | 1     |
| 6. Gestão da Loja               | 11       | 2       | 4    | 4     | 1     |
| 7. Operacional & Infraestrutura | 9        | 3       | 3    | 2     | 1     |
| 8. Analytics & Relatórios       | 7        | 0       | 3    | 3     | 1     |
| 9. Segurança & Compliance       | 6        | 2       | 2    | 2     | 0     |
| **TOTAL**                       | **89**   | **26**  | **33** | **24** | **6** |

---

## 1. Fluxo do Entregador

> Atualmente existe a entidade `Entregador` e o endpoint para vincular/remover entregador a um pedido (`PUT/DELETE /api/pedidos/{pedido_uuid}/entregador/{loja_uuid}`), mas **não há um fluxo de uso real para o entregador como ator do sistema**.

### 1.1 Painel do Entregador (impacto: Crítico, complexidade: Alta)

Endpoints específicos para o entregador autenticado:

- `GET /api/entregador/me/pedidos-disponiveis` — lista pedidos próximos disponíveis para aceite
- `GET /api/entregador/me/pedidos-ativos` — pedidos em andamento atribuídos a ele
- `GET /api/entregador/me/historico` — histórico paginado
- `GET /api/entregador/me/estatisticas` — entregas hoje/semana/mês, faturamento, avaliação média

**Dependências:** classe `entregador` já existe, falta extractor `EntregadorPermission` e usecase `EntregadorPainelUsecase`.

### 1.2 Aceite/Recusa de Entrega (impacto: Crítico, complexidade: Média)

- `POST /api/entregador/pedidos/{uuid}/aceitar` — entregador aceita; pedido sai do pool
- `POST /api/entregador/pedidos/{uuid}/recusar` — registra recusa (com motivo) e libera pedido
- `POST /api/entregador/pedidos/{uuid}/desistir` — desistência após aceite (com penalização)

**Mecânica:** estado intermediário `aguardando_entregador` entre `pronto` e `saiu_para_entrega`; lock otimista (versioning) para evitar race condition entre dois entregadores aceitando simultaneamente.

### 1.3 Status de Disponibilidade do Entregador (impacto: Crítico, complexidade: Baixa)

Campo `status_entregador` no usuário/entregador:
- `offline` — não recebe pedidos
- `online_disponivel` — aceita novos pedidos
- `online_ocupado` — em entrega
- `pausa` — almoço/intervalo

`PATCH /api/entregador/me/status` para alternar.

### 1.4 Geolocalização do Entregador (impacto: Crítico, complexidade: Alta)

Tabela nova `localizacao_entregador (entregador_uuid, latitude, longitude, atualizado_em, precisao_metros, bateria, velocidade_kmh)`.

- `POST /api/entregador/me/localizacao` — upsert da última posição (chamado a cada 10–30s pelo app)
- `GET /api/entregador/{uuid}/localizacao` — busca última posição (consumido por loja e cliente)

**Persistência:** considerar Redis para a posição "atual" e PostgreSQL+PostGIS para histórico/auditoria. Indexar por `ST_GeoHash` para queries de proximidade.

### 1.5 Cálculo de Pedidos Próximos (impacto: Alto, complexidade: Alta)

Quando uma loja libera um pedido para entrega, calcular entregadores em raio de N km da loja, ordenados por distância e ranking.

**Tecnologia:** PostGIS (`ST_DWithin`, `ST_Distance`) ou implementação manual com Haversine + bounding box.

### 1.6 Comprovante de Entrega (impacto: Crítico, complexidade: Média)

- `POST /api/entregador/pedidos/{uuid}/comprovante` — upload de foto + assinatura digital + código de confirmação informado pelo cliente
- Tabela `comprovantes_entrega (pedido_uuid, foto_url, assinatura_url, codigo_confirmacao, latitude, longitude, criado_em)`

### 1.7 Avaliação do Entregador (impacto: Alto, complexidade: Baixa)

Espelha as avaliações de loja/produto:
- `AvaliacaoEntregador { entregador_uuid, pedido_uuid, usuario_uuid, nota, comentario }`
- Cliente avalia entregador após pedido entregue

### 1.8 Ranking e Score do Entregador (impacto: Alto, complexidade: Média)

Score calculado por: avaliação média, taxa de aceitação, taxa de cancelamento, pontualidade. Usado no algoritmo de despacho (entregadores com score alto recebem prioridade).

### 1.9 Multi-loja por Entregador (impacto: Alto, complexidade: Média)

Hoje `Entregador` é vinculado a uma única loja. Em modelos modernos (iFood, Uber), entregadores são autônomos e atendem várias lojas. Tabela `entregador_lojas (entregador_uuid, loja_uuid, ativo)` e flag `entregador_marketplace BOOLEAN` no entregador.

### 1.10 Repasse Financeiro ao Entregador (impacto: Alto, complexidade: Alta)

Cálculo automático do valor a repassar por entrega (taxa fixa + km variável + bonificações). Ver categoria 3 (Pagamentos).

### 1.11 Comunicação Entregador–Cliente (impacto: Médio, complexidade: Alta)

Chat in-app ou número proxy (mascaramento de telefone, ex: serviço Twilio). Cliente nunca vê o número real do entregador.

### 1.12 Histórico de Eventos do Entregador (impacto: Baixo, complexidade: Baixa)

Auditoria de todas as ações: aceite, recusa, mudança de status, geolocalização, comprovante.

---

## 2. Rastreamento de Pedido

> Hoje o cliente vê apenas o status textual do pedido. Não há visualização em mapa nem ETA dinâmico.

### 2.1 Endpoint Unificado de Rastreamento (impacto: Crítico, complexidade: Média)

`GET /api/pedidos/{uuid}/rastreamento` retorna:

```json
{
  "pedido_uuid": "...",
  "status_atual": "saiu_para_entrega",
  "status_historico": [
    { "status": "criado", "em": "2026-04-29T10:00:00Z" },
    { "status": "confirmado_pela_loja", "em": "2026-04-29T10:02:30Z" }
  ],
  "loja": { "lat": -23.5, "lng": -46.6 },
  "destino": { "lat": -23.55, "lng": -46.62 },
  "entregador": {
    "uuid": "...",
    "nome": "Carlos",
    "lat": -23.52,
    "lng": -46.61,
    "atualizado_em": "2026-04-29T10:35:12Z"
  },
  "eta_minutos": 12,
  "rota_polyline": "..."
}
```

### 2.2 Histórico de Mudanças de Status (impacto: Crítico, complexidade: Baixa)

Tabela `historico_status_pedido (pedido_uuid, status_anterior, status_novo, alterado_por, alterado_em, motivo)`. Justifica-se para auditoria mesmo sem rastreamento.

### 2.3 Cálculo de ETA — Estimated Time of Arrival (impacto: Crítico, complexidade: Alta)

ETA dinâmico baseado em:
- Tempo de preparo médio da loja por categoria/horário
- Distância loja → cliente
- Velocidade média do entregador
- Trânsito atual (Google Maps Distance Matrix API ou OSRM self-hosted)

`GET /api/pedidos/{uuid}/eta` e snapshot em campo `eta_estimado: DateTime<Utc>` no pedido.

### 2.4 WebSocket / Server-Sent Events (impacto: Alto, complexidade: Alta)

`/api/sse/pedidos/{uuid}` — push de eventos: mudança de status, atualização de localização do entregador, mensagens da loja.

**Alternativa MVP:** SSE (Server-Sent Events) é mais simples e cabe bem em Axum (`axum::response::sse`).

### 2.5 Polling Otimizado (impacto: Alto, complexidade: Baixa)

Para clientes mobile sem suporte a SSE, expor cabeçalho `ETag` / `Last-Modified` em `GET /pedidos/{uuid}/rastreamento` para que o app só receba atualizações reais (304 Not Modified).

### 2.6 Mapa com Rota (impacto: Alto, complexidade: Alta)

Backend retorna polyline da rota loja → cliente via OSRM/Mapbox/Google Directions. Atualizada a cada N minutos ou quando o entregador desvia da rota.

### 2.7 Notificações de Mudança de Status (impacto: Médio, complexidade: Média)

Ver categoria 4. Cada transição da máquina de estados dispara push/email.

### 2.8 Página Pública de Rastreamento (impacto: Médio, complexidade: Baixa)

Token de rastreamento sem autenticação para compartilhar (quando o pedido é feito sem login). `GET /api/pedidos/publico/{token_rastreamento}`.

---

## 3. Pagamentos & Financeiro

> Hoje pedidos têm `forma_pagamento` como string mas **não há integração real com gateway, split, repasse ou antifraude**.

### 3.1 Integração com Gateway de Pagamento (impacto: Crítico, complexidade: Alta)

Asaas já parece estar parcialmente integrado. Expandir para:
- Cartão de crédito (online)
- Cartão de débito
- Pix (instantâneo + QR Code)
- Boleto
- Pagamento na entrega (dinheiro/máquina)

`POST /api/pagamentos/{pedido_uuid}/iniciar` retorna URL/QR/dados de cobrança.

### 3.2 Webhook de Confirmação de Pagamento (impacto: Crítico, complexidade: Média)

`POST /api/webhooks/asaas` — endpoint que recebe notificação do gateway e atualiza status de pagamento + libera pedido.

### 3.3 Tabela de Pagamentos (impacto: Crítico, complexidade: Baixa)

```
Pagamento {
  uuid, pedido_uuid, gateway, gateway_id,
  metodo, status (pendente/aprovado/recusado/estornado),
  valor, valor_taxa, valor_liquido,
  iniciado_em, aprovado_em, estornado_em
}
```

### 3.4 Split de Pagamento (impacto: Crítico, complexidade: Alta)

Distribuição automática: loja recebe X%, plataforma cobra taxa Y%, entregador recebe Z%. Asaas tem split nativo.

### 3.5 Pix com QR Code Dinâmico (impacto: Crítico, complexidade: Média)

QR Code gerado por pedido com expiração. Webhook confirma e libera pedido automaticamente.

### 3.6 Reembolso e Estorno (impacto: Crítico, complexidade: Média)

- `POST /api/pagamentos/{uuid}/estornar` — total ou parcial
- Política de estorno (pedido cancelado em até X minutos = estorno total)

### 3.7 Carteira / Saldo do Cliente (impacto: Alto, complexidade: Média)

Crédito por reembolso, cashback, indicação. Tabela `carteira_usuario (usuario_uuid, saldo, saldo_bloqueado)` + extrato de movimentações.

### 3.8 Cashback (impacto: Alto, complexidade: Média)

Percentual de cashback por loja/categoria/promoção, creditado na carteira após confirmação do pedido.

### 3.9 Repasse Financeiro à Loja (impacto: Crítico, complexidade: Alta)

- Conciliação por loja (D+1, D+7, D+14)
- Relatório de repasse com cálculo de taxa da plataforma
- Estorno descontado automaticamente
- `GET /api/financeiro/loja/{uuid}/repasses`

### 3.10 Repasse ao Entregador (impacto: Alto, complexidade: Alta)

Saldo acumulado por entrega + saque sob demanda ou agendado.

### 3.11 Antifraude (impacto: Alto, complexidade: Alta)

- Score por usuário (idade da conta, frequência, valor médio, geolocalização)
- Limite de tentativas de pagamento
- Bloqueio automático em padrões suspeitos
- Integração com Clearsale, Konduto ou Stripe Radar

### 3.12 Múltiplas Formas de Pagamento por Pedido (impacto: Médio, complexidade: Média)

Cliente paga R$ 30 com saldo da carteira + R$ 20 no cartão.

### 3.13 Salvar Cartões (Tokenização) (impacto: Alto, complexidade: Média)

`POST /api/usuarios/me/cartoes` — armazena token do gateway (PCI-compliant). Nunca armazenar PAN.

### 3.14 Nota Fiscal Eletrônica (impacto: Baixo, complexidade: Alta)

Integração com NFe/NFCe (depende da legislação e do tipo de loja). Importante para B2B.

---

## 4. Notificações

> Hoje só existe email transacional para verificação de cadastro (MailerSend).

### 4.1 Push Notifications Mobile (impacto: Crítico, complexidade: Alta)

- `POST /api/usuarios/me/dispositivos` — registra device token (FCM/APNs)
- Tabela `dispositivos_usuario (uuid, usuario_uuid, plataforma, token, ativo)`
- Worker que escuta eventos e envia push via FCM (Android) e APNs (iOS)

### 4.2 Notificação de Mudança de Status do Pedido (impacto: Crítico, complexidade: Média)

Templates por status: confirmado, em preparo, saiu para entrega, entregue. Multi-canal (push, email, opcionalmente SMS).

### 4.3 Notificação para Loja (impacto: Crítico, complexidade: Média)

Quando novo pedido entra, loja recebe push (e som contínuo no painel até confirmar).

### 4.4 Notificação para Entregador (impacto: Alto, complexidade: Média)

Pedido disponível para aceite, push com vibração distintiva e timeout para aceite.

### 4.5 SMS Transacional (impacto: Alto, complexidade: Baixa)

OTP de cadastro/recuperação, status crítico (pedido entregue, problema). Twilio/Zenvia/TotalVoice.

### 4.6 Email Transacional (impacto: Alto, complexidade: Média)

- Recibo de pedido por email
- Pesquisa de satisfação pós-entrega
- Newsletter de promoções (com opt-out)

### 4.7 Central de Preferências de Notificação (impacto: Alto, complexidade: Baixa)

Cliente escolhe o que quer receber e por qual canal. LGPD-friendly.

### 4.8 Outbox Pattern para Eventos (impacto: Médio, complexidade: Alta)

Tabela `outbox_eventos` para garantir entrega de notificação mesmo se o serviço externo cair. Worker processa fila com retry e backoff exponencial.

### 4.9 Histórico de Notificações (impacto: Médio, complexidade: Baixa)

`GET /api/usuarios/me/notificacoes` — central in-app com lida/não lida.

---

## 5. Experiência do Cliente

### 5.1 Carrinho Persistente (impacto: Alto, complexidade: Média)

Hoje o pedido é criado direto. Faltam endpoints de carrinho (criar, adicionar item, remover, alterar quantidade, aplicar cupom temporário, calcular total). Cliente pode fechar o app e retornar.

### 5.2 Reordenação / "Pedir de Novo" (impacto: Alto, complexidade: Baixa)

`POST /api/pedidos/{uuid}/duplicar` — recria carrinho com os mesmos itens.

### 5.3 Busca Global (impacto: Alto, complexidade: Média)

`GET /api/buscar?q=hamburguer&lat=&lng=` — busca lojas e produtos. Full-text search PostgreSQL (`tsvector`) ou Meilisearch/Typesense.

### 5.4 Filtros Avançados de Loja (impacto: Alto, complexidade: Média)

Por categoria de cozinha, preço médio, taxa de entrega grátis, tempo de entrega, avaliação mínima, promoção ativa, aberta agora.

### 5.5 Recomendação Personalizada (impacto: Médio, complexidade: Alta)

"Pedidos novamente", "Você pode gostar", "Populares na sua região". MVP: heurísticas (mais pedidos do usuário, mais pedidos da região). Avançado: ML colaborativo.

### 5.6 Programa de Fidelidade (impacto: Médio, complexidade: Média)

Pontos por pedido, troca por descontos. Por loja ou plataforma.

### 5.7 Indicação / Programa de Referral (impacto: Médio, complexidade: Média)

Já há `referral_system_analysis.md` no repo. Implementar: código de indicação, bonificação para indicador e indicado, anti-abuso.

### 5.8 Múltiplos Endereços com Apelido (impacto: Crítico, complexidade: Baixa)

Já tem CRUD de endereços. Falta: apelido (Casa, Trabalho), endereço padrão, validação de área de entrega.

### 5.9 Validação de Área de Entrega (impacto: Alto, complexidade: Alta)

Loja define polígono de área de entrega (PostGIS `geometry(Polygon)` ou raio simples). API rejeita pedido fora da área.

### 5.10 Taxa de Entrega Dinâmica (impacto: Alto, complexidade: Média)

Calculada por distância (Haversine ou OSRM), faixas de CEP ou polígonos com taxas distintas.

### 5.11 Agendamento de Pedido (impacto: Médio, complexidade: Média)

Cliente agenda pedido para horário futuro. Pedido entra na fila X minutos antes. Campo `agendado_para: Option<DateTime<Utc>>` no pedido.

### 5.12 Gorjeta para Entregador (impacto: Médio, complexidade: Baixa)

Campo opcional `gorjeta: Decimal` no pedido, repassado integralmente ao entregador.

### 5.13 Modo Convidado / Guest Checkout (impacto: Baixo, complexidade: Baixa)

Já existe parcialmente (pedido sem auth). Falta: vincular guest ao usuário se ele criar conta depois.

---

## 6. Gestão da Loja

### 6.1 Painel da Loja (Dashboard) (impacto: Crítico, complexidade: Alta)

Endpoints agregadores:
- Pedidos do dia (em aberto, finalizados, cancelados)
- Faturamento
- Ticket médio
- Produtos mais vendidos
- Avaliação atual

### 6.2 Aceite/Recusa do Pedido pela Loja (impacto: Crítico, complexidade: Baixa)

`POST /api/loja/pedidos/{uuid}/aceitar` (com tempo estimado de preparo customizado), `POST /api/loja/pedidos/{uuid}/recusar` (com motivo). Hoje a transição é manual sem motivos.

### 6.3 Modo Pausado / "Loja Fechada Temporariamente" (impacto: Alto, complexidade: Baixa)

Flag `pausada_ate: Option<DateTime>` no model Loja. Bloqueia novos pedidos sem mexer nos horários.

### 6.4 Estoque de Produtos (impacto: Alto, complexidade: Média)

- Quantidade disponível por produto
- Auto-desativar quando estoque = 0
- Alertas de estoque baixo
- Histórico de movimentação

### 6.5 Variações de Produto (impacto: Alto, complexidade: Média)

Tamanho (P/M/G), sabor base, embalagem. Hoje só existem partes (para pizza). Variação é diferente: é o produto base com preço/SKU diferente.

### 6.6 Combos / Kits (impacto: Alto, complexidade: Média)

Produto composto (ex: hambúrguer + batata + refri = R$ 30). Modelagem: `combo_itens (combo_uuid, produto_uuid, quantidade)`.

### 6.7 Fora de Horário por Categoria/Produto (impacto: Médio, complexidade: Baixa)

Café da manhã só de 6h–10h. Campos `disponivel_de`/`disponivel_ate` em categoria/produto.

### 6.8 Integração com KDS (Kitchen Display System) (impacto: Médio, complexidade: Alta)

Webhook ou API que envia pedido confirmado para impressora térmica do estabelecimento ou tela de cozinha.

### 6.9 Gestão de Funcionários com Permissões Granulares (impacto: Médio, complexidade: Média)

Hoje funcionário é só uma classe. Falta: granularidade — caixa, cozinha, gerente, com permissões diferentes por papel.

### 6.10 Multi-loja por Administrador (impacto: Médio, complexidade: Baixa)

Admin pode ter várias lojas e alternar entre elas. Já há `criado_por` em loja, falta endpoint de listagem e contexto ativo.

### 6.11 Configurações Operacionais (impacto: Baixo, complexidade: Baixa)

- Tempo médio de preparo por produto
- Limite máximo de pedidos simultâneos
- Aceite automático sim/não
- Tempo limite para confirmar antes de cancelar automaticamente

---

## 7. Operacional & Infraestrutura

### 7.1 Cancelamento de Pedido com Motivos (impacto: Crítico, complexidade: Baixa)

Tabela `motivos_cancelamento` + campo no pedido. Status `cancelado_pelo_cliente`, `cancelado_pela_loja`, `cancelado_pelo_sistema`. Política de quem pode cancelar e quando.

### 7.2 Despacho Automático de Entregadores (impacto: Crítico, complexidade: Alta)

Worker que roda a cada N segundos: pega pedidos prontos sem entregador → calcula entregadores próximos disponíveis → ordena por score → envia push de oferta com timeout. Algoritmo de despacho é o coração logístico.

### 7.3 Filas e Workers (impacto: Crítico, complexidade: Alta)

Hoje tudo é síncrono no request. Necessário sistema de filas (Redis + worker Rust, ou RabbitMQ, ou `pgmq` Postgres-based):
- Envio de notificações
- Processamento de webhooks
- Cálculo de repasses
- Limpeza de pré-cadastros expirados
- Soft-delete scheduler

### 7.4 Idempotência em Endpoints Críticos (impacto: Alto, complexidade: Média)

Header `Idempotency-Key` em `POST /pedidos/criar`, `POST /pagamentos`. Tabela de chaves usadas para evitar duplicação em retries.

### 7.5 Rate Limiting (impacto: Alto, complexidade: Baixa)

Por IP, por usuário, por endpoint. Tower middleware ou crate `tower_governor`.

### 7.6 Cache com Redis (impacto: Alto, complexidade: Média)

Redis para:
- Catálogo público (lojas, produtos)
- Sessões / blacklist de JWT
- Localização atual de entregadores
- Rate limit counters

### 7.7 Observabilidade (impacto: Médio, complexidade: Média)

- Prometheus metrics endpoint (`/metrics`)
- OpenTelemetry tracing distribuído
- Sentry para erros não tratados

### 7.8 CI/CD (impacto: Médio, complexidade: Média)

GitHub Actions: lint (clippy), test, build, deploy via Dokploy.

### 7.9 Backup e Disaster Recovery (impacto: Baixo, complexidade: Média)

PG backups regulares, point-in-time recovery, plano de DR documentado.

---

## 8. Analytics & Relatórios

### 8.1 Dashboard Operacional para Loja (impacto: Alto, complexidade: Alta)

Métricas em tempo real: pedidos/h, ticket médio, taxa de aceitação, tempo médio de preparo, cancelamentos.

### 8.2 Relatório Financeiro (impacto: Alto, complexidade: Média)

Por período: faturamento bruto, taxa da plataforma, repasse líquido, estornos, exportação CSV/PDF.

### 8.3 Relatório de Avaliações (impacto: Alto, complexidade: Baixa)

Por loja/produto: distribuição de notas, comentários recentes, evolução temporal.

### 8.4 Funil de Conversão (impacto: Médio, complexidade: Alta)

Visitas → produto visualizado → adicionado ao carrinho → checkout → pago. Requer tracking de eventos.

### 8.5 Métricas para Owner/Plataforma (impacto: Médio, complexidade: Média)

GMV total, número de lojas ativas, número de entregadores, churn de cliente, CAC/LTV.

### 8.6 Heatmap de Pedidos por Região (impacto: Médio, complexidade: Média)

Visualização geográfica de demanda. Útil para decisões de expansão.

### 8.7 Exportação de Dados (impacto: Baixo, complexidade: Baixa)

CSV/Excel/JSON com filtros de período. LGPD: cliente pode exportar seus próprios dados (Direito de Portabilidade).

---

## 9. Segurança & Compliance

### 9.1 Recuperação de Senha (impacto: Crítico, complexidade: Baixa)

`POST /api/auth/esqueci-senha` → email com token → `POST /api/auth/redefinir-senha`. Reusa infra de email já implementada.

### 9.2 Refresh Token / Logout Real (impacto: Crítico, complexidade: Média)

Hoje JWT é único e não há logout. Implementar:
- Access token curto (15 min)
- Refresh token longo (30 dias) armazenado no servidor
- Endpoint de revogação (logout)
- Blacklist de JWT em Redis

### 9.3 Autenticação Multifator (2FA/MFA) (impacto: Alto, complexidade: Média)

TOTP (Google Authenticator) para admin/owner. SMS OTP para cliente em ações críticas.

### 9.4 Auditoria (Audit Log) (impacto: Alto, complexidade: Média)

Tabela `audit_log (usuario_uuid, acao, entidade, entidade_uuid, ip, user_agent, payload, em)`. Todas as ações críticas (criação/edição/exclusão de loja, alteração de pedido, login).

### 9.5 LGPD / GDPR (impacto: Médio, complexidade: Média)

- Termo de aceite versionado (`termos_aceitos`)
- Direito ao esquecimento (já tem soft-delete; falta hard-delete pós scheduler)
- Direito à portabilidade (exportação)
- Política de retenção de dados

### 9.6 CORS e Hardening HTTP (impacto: Médio, complexidade: Baixa)

Configurar origens permitidas, CSP, headers de segurança (HSTS, X-Frame-Options, etc.).

---

## Detalhamento Técnico — Fluxo do Entregador

### Modelagem proposta

```sql
ALTER TABLE entregadores ADD COLUMN status_disponibilidade VARCHAR(32) DEFAULT 'offline';
ALTER TABLE entregadores ADD COLUMN score_atual NUMERIC(5,2) DEFAULT 5.0;
ALTER TABLE entregadores ADD COLUMN total_entregas INTEGER DEFAULT 0;

CREATE TABLE localizacao_entregador (
  entregador_uuid UUID PRIMARY KEY REFERENCES entregadores(uuid),
  latitude  NUMERIC(10,7) NOT NULL,
  longitude NUMERIC(10,7) NOT NULL,
  precisao_metros NUMERIC(8,2),
  velocidade_kmh NUMERIC(6,2),
  bateria SMALLINT,
  atualizado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE localizacao_entregador_historico (
  uuid UUID PRIMARY KEY,
  entregador_uuid UUID NOT NULL,
  latitude  NUMERIC(10,7),
  longitude NUMERIC(10,7),
  registrado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE ofertas_entrega (
  uuid UUID PRIMARY KEY,
  pedido_uuid UUID NOT NULL REFERENCES pedidos(uuid),
  entregador_uuid UUID NOT NULL REFERENCES entregadores(uuid),
  status VARCHAR(32) NOT NULL DEFAULT 'pendente',
  oferecido_em TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  expira_em   TIMESTAMPTZ NOT NULL,
  respondido_em TIMESTAMPTZ,
  motivo_recusa TEXT
);

CREATE TABLE avaliacoes_entregador (
  uuid UUID PRIMARY KEY,
  entregador_uuid UUID NOT NULL,
  pedido_uuid UUID NOT NULL UNIQUE,
  usuario_uuid UUID NOT NULL,
  nota NUMERIC(2,1) NOT NULL CHECK (nota BETWEEN 1 AND 5),
  comentario TEXT,
  criado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE comprovantes_entrega (
  pedido_uuid UUID PRIMARY KEY REFERENCES pedidos(uuid),
  foto_url TEXT,
  assinatura_url TEXT,
  codigo_confirmacao VARCHAR(8),
  latitude NUMERIC(10,7),
  longitude NUMERIC(10,7),
  registrado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Endpoints sugeridos

```
GET    /api/entregador/me/dashboard
GET    /api/entregador/me/pedidos-disponiveis
GET    /api/entregador/me/pedidos-ativos
GET    /api/entregador/me/historico?periodo=hoje
GET    /api/entregador/me/estatisticas
PATCH  /api/entregador/me/status
POST   /api/entregador/me/localizacao

POST   /api/entregador/ofertas/{uuid}/aceitar
POST   /api/entregador/ofertas/{uuid}/recusar
POST   /api/entregador/pedidos/{uuid}/iniciar-coleta
POST   /api/entregador/pedidos/{uuid}/coletado
POST   /api/entregador/pedidos/{uuid}/iniciar-entrega
POST   /api/entregador/pedidos/{uuid}/entregue
POST   /api/entregador/pedidos/{uuid}/comprovante

GET    /api/loja/pedidos/{uuid}/entregadores-proximos
```

### Camadas necessárias

- `EntregadorPainelUsecase` — orquestra queries do dashboard
- `EntregadorService` — lógica de aceite/recusa/status, validações
- `LocalizacaoService` — upsert + cálculo de proximidade + histórico
- `OfertaEntregaService` — gestão das ofertas com timeout
- `DespachoUsecase` — algoritmo que cria ofertas (rodado por worker)
- Ports: `LocalizacaoEntregadorPort`, `OfertaEntregaPort`, `ComprovanteEntregaPort`

---

## Detalhamento Técnico — Rastreamento de Pedido

### Modelagem proposta

```sql
CREATE TABLE historico_status_pedido (
  uuid UUID PRIMARY KEY,
  pedido_uuid UUID NOT NULL REFERENCES pedidos(uuid),
  status_anterior VARCHAR(64),
  status_novo VARCHAR(64) NOT NULL,
  alterado_por_uuid UUID,
  alterado_por_classe VARCHAR(32),
  motivo TEXT,
  alterado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE pedidos ADD COLUMN eta_estimado TIMESTAMPTZ;
ALTER TABLE pedidos ADD COLUMN tempo_preparo_minutos SMALLINT;
ALTER TABLE pedidos ADD COLUMN token_rastreamento UUID DEFAULT gen_random_uuid();
```

### Estratégia de real-time — 3 fases

**Fase MVP — Polling com ETag:**
```
GET /api/pedidos/{uuid}/rastreamento
If-None-Match: "v123"

→ 304 Not Modified  (sem mudança)
→ 200 OK + ETag: "v124"  (com nova versão)
```
App mobile faz polling a cada 5–10s. Versão incrementada quando: muda status, muda localização do entregador (threshold de 50m).

**Fase Crescimento — Server-Sent Events (SSE):**
```rust
async fn rastrear_pedido_sse(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let receiver = state.event_bus.subscribe_pedido(uuid).await;
    let stream = ReceiverStream::new(receiver)
        .map(|evt| Ok(Event::default().json_data(evt).unwrap()));
    Sse::new(stream).keep_alive(KeepAlive::default())
}
```

**Fase Escala — WebSocket bidirecional:**
Quando precisar de chat, ações do entregador em tempo real, múltiplos pedidos por conexão.

### EventBus interno

```rust
pub enum PedidoEvento {
    StatusAlterado { de: Status, para: Status },
    LocalizacaoEntregadorAtualizada { lat: f64, lng: f64, eta: DateTime<Utc> },
    EntregadorAtribuido { entregador_uuid: Uuid, nome: String },
    Cancelado { motivo: String },
}
```

### Cálculo de ETA

```rust
pub async fn calcular(&self, pedido: &Pedido) -> DateTime<Utc> {
    let preparo = self.tempo_medio_preparo_loja(&pedido.loja_uuid).await;
    let deslocamento = match pedido.status {
        // Antes de sair: loja → cliente
        Criado | AguardandoConfirmacao | Confirmado | EmPreparo => {
            self.distancia_service.calcular_tempo(loja.coords, destino.coords).await
        }
        // Em trânsito: entregador atual → cliente
        Pronto | SaiuParaEntrega => {
            let pos = self.localizacao_service.buscar_atual(pedido.entregador_uuid?).await?;
            self.distancia_service.calcular_tempo(pos, destino.coords).await
        }
        _ => Duration::zero(),
    };
    Utc::now() + preparo + deslocamento
}
```

Provedores de distance/duration: Google Maps Distance Matrix (mais preciso, pago), Mapbox Directions (bom tier free), OSRM self-hosted (gratuito), Haversine puro (MVP barato).

---

## Roadmap Sugerido em Fases

### Fase 1 — MVP Comercial (estimativa: 6–8 semanas)

> Mínimo viável para começar a operar comercialmente.

| # | Feature | Categoria |
|---|---------|-----------|
| 1 | Recuperação de senha | 9.1 |
| 2 | Refresh token + logout | 9.2 |
| 3 | Integração Asaas: cartão + Pix + webhook | 3.1, 3.2, 3.3, 3.5 |
| 4 | Cancelamento de pedido com motivos | 7.1 |
| 5 | Histórico de status do pedido | 2.2 |
| 6 | Push notifications (FCM/APNs) | 4.1, 4.2, 4.3 |
| 7 | Aceite/recusa pela loja com motivos | 6.2 |
| 8 | Painel da loja básico | 6.1 |
| 9 | Modo pausado da loja | 6.3 |
| 10 | Validação de área de entrega | 5.9 |
| 11 | Taxa de entrega dinâmica | 5.10 |
| 12 | Carrinho persistente | 5.1 |
| 13 | Reembolso/estorno | 3.6 |
| 14 | Audit log básico | 9.4 |

### Fase 2 — Operação Logística (estimativa: 8–10 semanas)

> Fluxo completo de entregador e rastreamento.

| # | Feature | Categoria |
|---|---------|-----------|
| 1 | Painel + status do entregador | 1.1, 1.3 |
| 2 | Geolocalização do entregador | 1.4 |
| 3 | Aceite/recusa de entrega | 1.2 |
| 4 | Despacho automático | 7.2 |
| 5 | Comprovante de entrega | 1.6 |
| 6 | Avaliação do entregador | 1.7 |
| 7 | Endpoint de rastreamento + ETA | 2.1, 2.3 |
| 8 | SSE para real-time | 2.4 |
| 9 | Filas e workers | 7.3 |
| 10 | Idempotência | 7.4 |
| 11 | Repasse à loja e ao entregador | 3.9, 3.10 |
| 12 | Split de pagamento | 3.4 |
| 13 | Cache Redis | 7.6 |
| 14 | Notificações ao entregador | 4.4 |
| 15 | Página pública de rastreamento | 2.8 |

### Fase 3 — Crescimento (estimativa: 10–12 semanas)

> Recursos que aumentam retenção, conversão e ticket médio.

| # | Feature | Categoria |
|---|---------|-----------|
| 1 | Estoque de produtos | 6.4 |
| 2 | Variações de produto | 6.5 |
| 3 | Combos / kits | 6.6 |
| 4 | Busca global e filtros avançados | 5.3, 5.4 |
| 5 | "Pedir de novo" | 5.2 |
| 6 | Carteira do cliente + cashback | 3.7, 3.8 |
| 7 | Tokenização de cartões | 3.13 |
| 8 | Programa de fidelidade | 5.6 |
| 9 | Indicação/referral | 5.7 |
| 10 | Agendamento de pedido | 5.11 |
| 11 | Gorjeta para entregador | 5.12 |
| 12 | Dashboard operacional + relatório financeiro | 8.1, 8.2 |
| 13 | Múltiplas formas de pagamento por pedido | 3.12 |
| 14 | Multi-loja por entregador | 1.9 |
| 15 | Ranking/score do entregador | 1.8 |

### Fase 4 — Escala & Maturidade (contínuo)

> Performance, segurança avançada, ML e diferenciação competitiva.

| # | Feature | Categoria |
|---|---------|-----------|
| 1 | Recomendação personalizada | 5.5 |
| 2 | Antifraude | 3.11 |
| 3 | 2FA/MFA | 9.3 |
| 4 | LGPD completo | 9.5 |
| 5 | Chat entregador–cliente | 1.11 |
| 6 | Mapa com rota dinâmica | 2.6 |
| 7 | Heatmap geográfico de demanda | 8.6 |
| 8 | Funil de conversão | 8.4 |
| 9 | Métricas para owner/plataforma | 8.5 |
| 10 | KDS / impressão térmica | 6.8 |
| 11 | Permissões granulares de funcionário | 6.9 |
| 12 | Observabilidade (Prometheus, OTEL, Sentry) | 7.7 |
| 13 | CI/CD completo | 7.8 |
| 14 | Backup e DR | 7.9 |
| 15 | NFe/NFCe | 3.14 |
| 16 | Migração para microserviços (ChickiePayment, ChickieAuth, ChickiePushNotification, ChickieWorker) | — |

---

## Considerações Arquiteturais

1. **Manter Hexagonal/Clean** — toda nova feature deve seguir Handler → Usecase → Service → Port → Repository.
2. **Workers como crate separado** — criar `chickie-worker` que compartilha `services` e `repositories`. Facilita o futuro split em microserviços.
3. **Outbox Pattern** — adotar desde já. Cada mudança crítica grava em `outbox_eventos` para garantir entrega de side-effects (notificação, repasse, analytics).
4. **PostGIS** — ativar antes de implementar área de entrega e geolocalização. Migration custa pouco e abre muitas portas.
5. **Redis** — entra naturalmente na Fase 2 (cache, pub/sub, rate limit, sessões).
6. **Versionamento de API** — adotar `/api/v1/` antes de expor publicamente, para permitir evolução sem breaking changes.
7. **Testes** — não há testes no projeto. Crítico ter testes de integração para máquina de estados de pedido, cálculo de total, regras de cupom e transições de pagamento.
