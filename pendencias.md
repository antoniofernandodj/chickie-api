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
| 7 | **CRUD de ingredientes** | Sem endpoints para criar, listar, atualizar ingredientes. | Baixa |
| 8 | **CRUD de horários de funcionamento** | Sem endpoints para gerenciar horários. | Baixa |
| 9 | **CRUD de configurações de pedido** | Sem endpoints para `max_partes` e `tipo_calculo`. | Baixa |
| 10 | **CRUD de funcionários** | Só existe `adicionar`, faltam listar, atualizar, deletar. | Média |
| 11 | **CRUD de entregadores** | Só existe `adicionar`, faltam listar, atualizar, deletar, ativar/desativar. | Média |
| 12 | **CRUD de cupons (admin)** | Faltam atualizar, deletar, ativar/desativar. | Média |
| 13 | **Pagamentos** | Tabela e endpoints para registrar pagamentos. | Alta (futuro) |
| 14 | **Notificações push** | Sistema de notificações para status do pedido. | Baixa (futuro) |
| 15 | **CI/CD pipeline** | Linters, testes automatizados, deploy. | Média |
| 16 | **Soft-delete de usuários** | Marcado `a_remover` + scheduler (conforme docs). | Média |
| 17 | **Soft-delete de lojas** | Mesmo mecanismo de soft-delete. | Média |

## 📝 Documentação

| # | Tarefa | Detalhe |
|---|--------|---------|
| 18 | **Atualizar diagrama de relacionamento** | Entidades foram expandidas — diagrama de ER está ausente. |
| 19 | **Documentar enum `EstadoDePedido`** | Todos os 7 status com descrição e transições permitidas. |
| 20 | **Documentar permissões por classe** | Tabela de quais classes podem acessar quais endpoints. |

---

## Resumo por Prioridade

| Prioridade | Count | Itens |
|------------|-------|-------|
| 🔴 Crítico | 2 | 1, 2 |
| 🟡 Bugs | 0 | — |
| 🟢 Melhorias | 2 | 3, 4 |
| 📋 Features | 13 | 5–17 |
| 📝 Docs | 3 | 18, 19, 20 |

**Total: 20 pendências**
