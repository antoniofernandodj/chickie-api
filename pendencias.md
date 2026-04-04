# Pendências — Chickie API

> Lista de tarefas, bugs conhecidos e melhorias identificadas. Última atualização: 2026-04-05.

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
| 4 | **Unused imports** | Vários warnings de imports não usados. | Diversos |

## 📋 Funcionalidades Pendentes

| # | Feature | Detalhe | Prioridade |
|---|---------|---------|------------|
| 5 | **Atribuir entregador ao pedido** | Sem endpoint para vincular entregador a um pedido. | Alta |
| 6 | **Listar pedidos por usuário** | Endpoint para cliente ver seus próprios pedidos. | Média |
| 7 | **CRUD de ingredientes** | ✅ Criar, listar, atualizar, deletar implementados. | — |
| 8 | **CRUD de horários de funcionamento** | Service + Usecase prontos. Faltam handlers e rotas. | Baixa |
| 9 | **CRUD de configurações de pedido** | Service + Usecase prontos. Faltam handlers e rotas. | Baixa |
| 10 | **CRUD de funcionários** | ✅ Listar e atualizar implementados. Faltam handler de criar e deletar. | — |
| 11 | **CRUD de entregadores** | ✅ Listar, atualizar e set_disponivel implementados. Faltam handler de criar e deletar. | — |
| 12 | **CRUD de cupons (admin)** | Service + Usecase prontos. Faltam handlers e rotas (atualizar, deletar). | Média |
| 13 | **Pagamentos** | Tabela e endpoints para registrar pagamentos. | Alta (futuro) |
| 14 | **Notificações push** | Sistema de notificações para status do pedido. | Baixa (futuro) |
| 15 | **CI/CD pipeline** | Linters, testes automatizados, deploy. | Média |
| 16 | **Soft-delete de usuários** | Marcado `a_remover` + scheduler (conforme docs). | Média |
| 17 | **Soft-delete de lojas** | Mesmo mecanismo de soft-delete. | Média |

## 📝 Documentação

| # | Tarefa | Detalhe |
|---|--------|---------|
| 18 | **Atualizar diagrama de relacionamento** | Entidades foram expandidas — diagrama de ER está ausente. |
| 19 | **Documentar permissões por classe** | Tabela de quais classes podem acessar quais endpoints. |

---

## Resumo por Prioridade

| Prioridade | Count | Itens |
|------------|-------|-------|
| 🔴 Crítico | 2 | 1, 2 |
| 🟡 Bugs | 0 | — |
| 🟢 Melhorias | 2 | 3, 4 |
| 📋 Features | 11 | 5, 6, 8–17 |
| 📝 Docs | 2 | 18, 19 |

**Total: 17 pendências**

### Infraestrutura Pronta (services + usecases criados)

| Entidade | Service | Usecase | Handlers | Rotas |
|----------|---------|---------|----------|-------|
| **Ingredientes** | ✅ | ✅ | ✅ (4/4) | ✅ |
| **Horários** | ✅ | ✅ | ❌ | ❌ |
| **Config Pedido** | ✅ | ✅ | ❌ | ❌ |
| **Funcionários** | ✅ | ✅ | ❌ | ❌ |
| **Entregadores** | ✅ | ✅ | ❌ | ❌ |
| **Cupons (update/delete)** | ✅ | ✅ | ❌ | ❌ |
