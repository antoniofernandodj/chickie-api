# Pendências — Chickie API

> Lista de tarefas, bugs conhecidos e melhorias identificadas. Última atualização: 2026-04-04.

---

## 🔴 Crítico / Segurança

| # | Tarefa | Detalhe | Arquivo(s) |
|---|--------|---------|------------|
| 1 | **Remover `/api/wipe` antes de produção** | Endpoint apaga todo o banco sem autenticação. | `src/api/routers.rs` |
| 2 | **`criar_pedido` recebe `loja_uuid` no body E no path** | O path tem `{loja_uuid}` mas o handler extrai do body. Inconsistente — deveria vir só do path. | `src/api/pedido/criar_pedido.rs`, `src/api/routers.rs` |
| 3 | **`validar_cupom` sem autenticação** | Expõe dados de cupons sem auth. | `src/api/cupom/validar_cupom.rs` |

## 🟡 Bugs Conhecidos

| # | Bug | Detalhe | Arquivo(s) |
|---|-----|---------|------------|
| 4 | **`ClienteRepository::listar_todos_por_loja` retorna `Vec<Produto>`** | Método copia/cola errado — tipo de retorno não bate com a entidade. | `src/repositories/cliente_repository.rs` |
| 5 | **`Promocao` aplica para toda a loja** | Modelo atual não suporta promoção por produto ou categoria. | `src/models/promocoes.rs` |

## 🟢 Melhorias de Código

| # | Tarefa | Detalhe | Arquivo(s) |
|---|--------|---------|------------|
| 6 | **Substituir `println!` por `tracing`** | 3 em `pedido_service.rs`, 1 em `loja_service.rs`, 10 em `pedido_repository.rs`. | `src/services/pedido_service.rs`, `src/services/loja_service.rs`, `src/repositories/pedido_repository.rs` |
| 7 | **Remover `src/main2.rs`** | Arquivo morto de teste (556 linhas) não referenciado. | `src/main2.rs` |
| 8 | **`PedidoUsecase` vazio** | Struct existe sem métodos. Implementar ou remover. | `src/usecases/pedido.rs` |
| 9 | **`listar_usuarios` sem paginação** | Retorna todos os usuários de uma vez. | `src/api/usuario/listar_usuarios.rs` |
| 10 | **Unused imports** | Vários warnings de imports não usados. | Diversos |

## 📋 Funcionalidades Pendentes

| # | Feature | Detalhe | Prioridade |
|---|---------|---------|------------|
| 11 | **Atualizar status do pedido** | Não existe endpoint para transicionar pedido (criado → em preparo → entregue). | Alta |
| 12 | **Atribuir entregador ao pedido** | Sem endpoint para vincular entregador a um pedido. | Alta |
| 13 | **Listar pedidos por loja** | `listar_pedidos` existe mas pode precisar de filtro por loja/status. | Média |
| 14 | **Listar pedidos por usuário** | Endpoint para cliente ver seus próprios pedidos. | Média |
| 15 | **CRUD de ingredientes** | Sem endpoints para criar, listar, atualizar ingredientes. | Baixa |
| 16 | **CRUD de horários de funcionamento** | Sem endpoints para gerenciar horários. | Baixa |
| 17 | **CRUD de configurações de pedido** | Sem endpoints para `max_partes` e `tipo_calculo`. | Baixa |
| 18 | **CRUD de funcionários** | Só existe `adicionar`, faltam listar, atualizar, deletar. | Média |
| 19 | **CRUD de entregadores** | Só existe `adicionar`, faltam listar, atualizar, deletar, ativar/desativar. | Média |
| 20 | **CRUD de cupons (admin)** | Faltam atualizar, deletar, ativar/desativar. | Média |
| 21 | **CRUD de promoções** | Só existe `criar`, faltam listar, atualizar, deletar. | Média |
| 22 | **Pagamentos** | Tabela e endpoints para registrar pagamentos. | Alta (futuro) |
| 23 | **Notificações push** | Sistema de notificações para status do pedido. | Baixa (futuro) |
| 24 | **CI/CD pipeline** | Linters, testes automatizados, deploy. | Média |
| 25 | **Soft-delete de usuários** | Marcado `a_remover` + scheduler (conforme docs). | Média |
| 26 | **Soft-delete de lojas** | Mesmo mecanismo de soft-delete. | Média |

## 📝 Documentação

| # | Tarefa | Detalhe |
|---|--------|---------|
| 27 | **Atualizar diagrama de relacionamento** | Entidades foram expandidas — diagrama de ER está ausente. |
| 28 | **Documentar enum `EstadoDePedido`** | Todos os 7 status com descrição e transições permitidas. |
| 29 | **Documentar permissões por classe** | Tabela de quais classes podem acessar quais endpoints. |

---

## Resumo por Prioridade

| Prioridade | Count | Itens |
|------------|-------|-------|
| 🔴 Crítico | 3 | 1, 2, 3 |
| 🟡 Bugs | 2 | 4, 5 |
| 🟢 Melhorias | 5 | 6, 7, 8, 9, 10 |
| 📋 Features | 16 | 11–26 |
| 📝 Docs | 3 | 27, 28, 29 |

**Total: 29 pendências**
