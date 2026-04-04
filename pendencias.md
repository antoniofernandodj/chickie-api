# Pendências — Chickie API

> Lista de tarefas, bugs conhecidos e melhorias identificadas. Última atualização: 2026-04-04.

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
| 6 | **Substituir `println!` por `tracing`** | 3 em `pedido_service.rs`, 1 em `loja_service.rs`, 10 em `pedido_repository.rs`. | `src/services/pedido_service.rs`, `src/services/loja_service.rs`, `src/repositories/pedido_repository.rs` |
| 7 | **Remover `src/main2.rs`** | Arquivo morto de teste (556 linhas) não referenciado. | `src/main2.rs` |
| 8 | **`listar_usuarios` sem paginação** | Retorna todos os usuários de uma vez. | `src/api/usuario/listar_usuarios.rs` |
| 9 | **Unused imports** | Vários warnings de imports não usados. | Diversos |

## 📋 Funcionalidades Pendentes

| # | Feature | Detalhe | Prioridade |
|---|---------|---------|------------|
| 10 | **Atualizar status do pedido** | Não existe endpoint para transicionar pedido (criado → em preparo → entregue). | Alta |
| 11 | **Atribuir entregador ao pedido** | Sem endpoint para vincular entregador a um pedido. | Alta |
| 12 | **Listar pedidos por usuário** | Endpoint para cliente ver seus próprios pedidos. | Média |
| 13 | **CRUD de ingredientes** | Sem endpoints para criar, listar, atualizar ingredientes. | Baixa |
| 14 | **CRUD de horários de funcionamento** | Sem endpoints para gerenciar horários. | Baixa |
| 15 | **CRUD de configurações de pedido** | Sem endpoints para `max_partes` e `tipo_calculo`. | Baixa |
| 16 | **CRUD de funcionários** | Só existe `adicionar`, faltam listar, atualizar, deletar. | Média |
| 17 | **CRUD de entregadores** | Só existe `adicionar`, faltam listar, atualizar, deletar, ativar/desativar. | Média |
| 18 | **CRUD de cupons (admin)** | Faltam atualizar, deletar, ativar/desativar. | Média |
| 19 | **Pagamentos** | Tabela e endpoints para registrar pagamentos. | Alta (futuro) |
| 20 | **Notificações push** | Sistema de notificações para status do pedido. | Baixa (futuro) |
| 21 | **CI/CD pipeline** | Linters, testes automatizados, deploy. | Média |
| 22 | **Soft-delete de usuários** | Marcado `a_remover` + scheduler (conforme docs). | Média |
| 23 | **Soft-delete de lojas** | Mesmo mecanismo de soft-delete. | Média |

## 📝 Documentação

| # | Tarefa | Detalhe |
|---|--------|---------|
| 24 | **Atualizar diagrama de relacionamento** | Entidades foram expandidas — diagrama de ER está ausente. |
| 25 | **Documentar enum `EstadoDePedido`** | Todos os 7 status com descrição e transições permitidas. |
| 26 | **Documentar permissões por classe** | Tabela de quais classes podem acessar quais endpoints. |

---

## Resumo por Prioridade

| Prioridade | Count | Itens |
|------------|-------|-------|
| 🔴 Crítico | 2 | 1, 2 |
| 🟡 Bugs | 0 | — |
| 🟢 Melhorias | 4 | 3, 4, 5, 6 |
| 📋 Features | 14 | 7–20 |
| 📝 Docs | 3 | 21, 22, 23 |

**Total: 23 pendências**
