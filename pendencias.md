# Pendências — Chickie API

> Lista de tarefas, bugs conhecidos e melhorias identificadas. Última atualização: 2026-04-19.

---

## 🔴 Crítico / Segurança

| # | Tarefa | Detalhe | Arquivo(s) |
|---|--------|---------|------------|
| 1 | **Remover `/api/wipe` antes de produção** | Endpoint apaga todo o banco sem autenticação. | `src/api/routers.rs` |
| 2 | **`validar_cupom` sem autenticação** | Expõe dados de cupons sem auth. | `src/api/cupom/validar_cupom.rs` |

## 🟡 Bugs Conhecidos

_(nenhum bug conhecido)_

## 🟢 Melhorias de Código

| # | Tarefa | Detalhe | Arquivo(s) |
|---|--------|---------|------------|
| 3 | **`listar_usuarios` sem paginação** | Retorna todos os usuários de uma vez. | `src/api/usuario/listar_usuarios.rs` |

### ✅ Melhorias Concluídas

| # | Tarefa | Detalhe | Data |
|---|--------|---------|------|
| 31 | **Campo `contato` em pedidos** | `contato: Option<String>` (11 dígitos, filtrado de não-numéricos) adicionado ao model, repository (INSERT/UPDATE), usecase, handler. Migration `0006` absorveu `usuario_uuid` nullable + `contato VARCHAR(11)` (migration `0010` removida). | 2026-04-24 |
| 30 | **Pedido sem usuário obrigatório** | `usuario_uuid` em `pedidos` agora é nullable (absorvida na migration `0006`). `endereco_entrega` no body de `/api/pedidos/criar` é opcional. Endpoint sem auth obrigatória (usa `optional_auth_middleware`). Stack completa atualizada. | 2026-04-19 |
| 29 | **Celular UNIQUE + endpoint verificar** | Migration `0009` adiciona UNIQUE constraint em `celular`. Handler signup filtra caracteres especiais, mantendo apenas dígitos. Novo endpoint `GET /api/usuarios/verificar-celular/{celular}`. Service `verificar_celular_disponivel` adicionado. | 2026-04-13 |
| 28 | **OwnerPermission + filtro por classe** | Sistema `OwnerPermission` via env var `OWNER_EMAIL`. `GET /api/usuarios/?classe=...` filtra por classe. Endpoints de soft delete de usuário agora permitem Self/Owner. `/api/wipe` protegido por Owner. Novos extractors: `AdminPermission`, `OwnerPermission`, helper `is_self_or_owner`. | 2026-04-13 |
| 27 | **Campo bloqueado para usuários e lojas** | Novo campo `bloqueado: bool` adicionado a `usuarios` e `lojas`. Endpoints POST `/api/usuarios/{uuid}/bloqueado` e POST `/api/lojas/{uuid}/bloqueado` para toggle. Usuários bloqueados não podem fazer login (verificado em `autenticar` e `auth_middleware`). Migration `0008` criada. Stack completa: models, ports, adapters, repositories, services, handlers, routes. | 2026-04-13 |
| 25 | **CRUD completo de cupons** | Novos endpoints padronizados em `/api/cupons/`: POST (criar), GET (listar todos), GET/{uuid} (buscar), PUT (atualizar), DELETE (deletar). Rotas legadas mantidas em `/api/marketing/` para compatibilidade. MarketingService atualizado com `buscar_cupom` e `listar_todos_cupons`. | 2026-04-12 |
| 26 | **CRUD completo de avaliações** | Novos endpoints para avaliações de loja e produto: GET (listar por loja/produto), GET/{uuid} (buscar), PUT (atualizar), DELETE (deletar). Ports, repositories, service, usecase e handlers atualizados. | 2026-04-12 |
| 14 | **Timestamps com tipo correto** | Models migrados de `String` para `chrono::DateTime<Utc>` para compatibilidade com PostgreSQL `TIMESTAMPTZ`. INSERT/UPDATE agora omitem `criado_em`/`atualizado_em` (usam defaults/triggers do DB). | 2026-04-05 |
| 15 | **Campos TIME com tipo correto** | `horario_abertura`/`horario_fechamento` (`loja`) e `abertura`/`fechamento` (`horarios_funcionamento`) migrados de `String` para `chrono::NaiveTime`. | 2026-04-05 |
| 16 | **Endpoint minhas lojas** | `GET /api/admin/minhas-lojas` lista lojas criadas pelo admin logado. Tabela `lojas` ganhou campo `criado_por UUID` (FK para `usuarios`). Migration `0003` criada. | 2026-04-05 |
| 22 | **CRUD completo de adicionais** | Novo endpoint `PUT /api/catalogo/{loja_uuid}/adicionais/{adicional_uuid}` para editar nome, descrição e preço de adicional. Endpoint `DELETE` para deletar adicional. Endpoint `PUT /.../disponibilidade` para toggle. Segue arquitetura Handler → Service → Repository. | 2026-04-09 |
| 23 | **Disponibilidade de produto** | Novo endpoint `PUT /api/produtos/{loja_uuid}/{produto_uuid}/disponibilidade` para ativar/desativar produto via body `{ disponivel: bool }`. Segue arquitetura Handler → Service → Repository. | 2026-04-09 |
| 17 | **Campos NUMERIC com tipo correto** | Todos os campos `f64`/`Option<f64>` mapeados para `NUMERIC` migrados para `rust_decimal::Decimal`. Afeta preço, nota, salario, taxa_entrega, valor_minimo, latitude/longitude, quantidade, total, subtotal, desconto em 10+ models. | 2026-04-05 |
| 18 | **Pesquisa de lojas** | Novos endpoints `GET /api/lojas/pesquisar?termo=...`, `GET /api/lojas/{uuid}` e `GET /api/lojas/slug/{slug}` para busca pública de lojas. Segue arquitetura Handler → Usecase → Service → Repository. | 2026-04-05 |
| 19 | **Campo pizza_mode na categoria** | `categorias_produtos` ganhou campo `pizza_mode BOOLEAN DEFAULT FALSE`. Migration `0004` criada. Stack completa atualizada: model, repository, service, handlers criar/atualizar categoria. | 2026-04-07 |
| 20 | **Listar pedidos por usuário** | Novo endpoint `GET /api/pedidos/meus` retorna todos os pedidos do usuário autenticado com hidratação completa (itens, partes, adicionais). Usa `buscar_completos_por_usuario` do repository. | 2026-04-07 |
| 21 | **Atribuir entregador ao pedido** | Endpoints `PUT /api/pedidos/{pedido_uuid}/entregador/{loja_uuid}` e `DELETE` para vincular/remover entregador. Migration `0005` adicionou `entregador_uuid` à tabela `pedidos`. Stack completa: model, repository, service, usecase, handlers. | 2026-04-07 |
| 24 | **Clean Architecture aplicada** | Ports (23 traits), adapters, DomainError, services usam traits, database movido para api/infrastructure, documentação completa com tutorial em CLEAN_ARCHITECTURE_GUIDE.md | 2026-04-11 |

## 📋 Funcionalidades Pendentes

| # | Feature | Detalhe | Prioridade |
|---|---------|---------|------------|
| 7 | **Pagamentos** | Tabela e endpoints para registrar pagamentos. | Alta (futuro) |
| 8 | **Notificações push** | Sistema de notificações para status do pedido. | Baixa (futuro) |
| 9 | **CI/CD pipeline** | Linters, testes automatizados, deploy. | Média |
| 10 | **Soft-delete de usuários** | Marcado `a_remover` + scheduler (conforme docs). | Média |
| 11 | **Soft-delete de lojas** | Mesmo mecanismo de soft-delete. | Média |

## 📝 Documentação

| # | Tarefa | Detalhe |
|---|--------|---------|
| 12 | **Atualizar diagrama de relacionamento** | Entidades foram expandidas — diagrama de ER está ausente. |
| 13 | **Documentar permissões por classe** | Tabela de quais classes podem acessar quais endpoints. |

---

## Resumo por Prioridade

| Prioridade | Count | Itens |
|------------|-------|-------|
| 🔴 Crítico | 2 | 1, 2 |
| 🟡 Bugs | 0 | — |
| 🟢 Melhorias | 2 | 3, 4 |
| ✅ Concluídas | 15 | 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29 |
| 📋 Features | 5 | 7–11 |
| 📝 Docs | 2 | 12, 13 |

**Total: 11 pendências ativas, 15 concluídas recentemente**
