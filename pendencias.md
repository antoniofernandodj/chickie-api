# Pendências — Chickie API

> Lista de tarefas, bugs conhecidos e melhorias identificadas.

---

## 🔴 Crítico / Segurança

| # | Tarefa | Detalhe | Arquivo(s) |
|---|--------|---------|------------|
| 1 | **Remover `/api/wipe` antes de produção** | Endpoint apaga todo o banco sem autenticação. | `src/api/wipe.rs`, `src/api/routers.rs` |
| 2 | **`criar_cupom` sem autenticação** | Qualquer pessoa pode criar cupons sem estar logado. | `src/api/cupom/criar_cupom.rs` |
| 3 | **`validar_cupom` sem autenticação** | Expõe dados de cupons sem auth. | `src/api/cupom/validar_cupom.rs` |

## 🟡 Bugs Conhecidos

| # | Bug | Detalhe | Arquivo(s) |
|---|-----|---------|------------|
| 5 | **`ClienteRepository::listar_todos_por_loja` retorna `Vec<Produto>`** | Método copia/cola errado — tipo de retorno não bate com a entidade. | `src/repositories/cliente_repository.rs` |
| 6 | **`Promocao` aplica para toda a loja** | Modelo atual não suporta promoção por produto ou categoria. | `src/models/promocoes.rs`, `src/services/marketing_service.rs` |
| 7 | **Rotas de produtos sem `{loja_uuid}` no path** | Handler `criar_produto` espera `Path(loja_uuid)` mas a rota é `/api/produtos/` sem parâmetro. | `src/api/routers.rs`, `src/api/produto/criar_produto.rs` |
| 8 | **Rotas de pedidos sem `{loja_uuid}` no path** | Handlers de pedido usam `Path(loja_uuid)` mas rota é `/api/pedidos/`. | `src/api/routers.rs`, `src/api/pedido/` |

## 🟢 Melhorias de Código

| # | Tarefa | Detalhe | Arquivo(s) |
|---|--------|---------|------------|
| 9 | **Substituir `println!` por `tracing`** | 3 em `pedido_service.rs`, 1 em `loja_service.rs`, 10 em `pedido_repository.rs`. Viola convenção do projeto. | `src/services/pedido_service.rs`, `src/services/loja_service.rs`, `src/repositories/pedido_repository.rs` |
| 10 | **Remover `src/main2.rs`** | Arquivo morto de teste (556 linhas) com `println!` excessivo. Não referenciado em módulo algum. | `src/main2.rs` |
| 11 | **`PedidoUsecase` vazio** | Struct existe sem métodos. Implementar ou remover. | `src/usecases/pedido.rs` |
| 12 | **`Unused import: jsonwebtoken::crypto::CryptoProvider`** | Import não usado em `usuario_service.rs`. | `src/services/usuario_service.rs` |
| 13 | **Handler `listar_usuarios` sem paginação** | Retorna todos os usuários de uma vez. | `src/api/usuario/listar_usuarios.rs` |
| 14 | **`listar_lojas_admin` sem diferenciação de `listar_lojas`** | Mesma lógica, apenas nome diferente. | `src/api/loja/listar_lojas_admin.rs` |

## 📋 Funcionalidades Pendentes

| # | Feature | Detalhe | Prioridade |
|---|---------|---------|------------|
| 15 | **Atualizar status do pedido** | Não existe endpoint para transicionar pedido (criado → em preparo → entregue). | Alta |
| 16 | **Atribuir entregador ao pedido** | Sem endpoint para vincular entregador a um pedido. | Alta |
| 17 | **Listar pedidos por loja** | `listar_pedidos` existe mas pode precisar de filtro por status. | Média |
| 18 | **Listar pedidos por usuário** | Endpoint para cliente ver seus próprios pedidos. | Média |
| 19 | **CRUD de ingredientes** | Sem endpoints para criar, listar, atualizar ingredientes. | Baixa |
| 20 | **CRUD de horários de funcionamento** | Sem endpoints para gerenciar horários. | Baixa |
| 21 | **CRUD de configurações de pedido da loja** | Sem endpoints para `max_partes` e `tipo_calculo`. | Baixa |
| 22 | **CRUD de funcionários** | Só existe `adicionar`, faltam listar, atualizar, deletar. | Média |
| 23 | **CRUD de entregadores** | Só existe `adicionar`, faltam listar, atualizar, deletar, ativar/desativar. | Média |
| 24 | **CRUD de cupons (admin)** | Faltam listar por loja, atualizar, deletar, ativar/desativar. | Média |
| 25 | **CRUD de promoções (admin)** | Só existe `criar`, faltam listar, atualizar, deletar. | Média |
| 26 | **Pagamentos** | Tabela e endpoints para registrar pagamentos. | Alta (futuro) |
| 27 | **Notificações push** | Sistema de notificações para status do pedido. | Baixa (futuro) |
| 28 | **CI/CD pipeline** | Linters, testes automatizados, deploy. | Média |
| 29 | **Soft-delete de usuários** | Marcado `a_remover` + scheduler (conforme docs). | Média |
| 30 | **Soft-delete de lojas** | Mesmo mecanismo de soft-delete. | Média |

## 📝 Documentação

| # | Tarefa | Detalhe |
|---|--------|---------|
| 31 | **Atualizar diagrama de relacionamento** | Entidades foram expandidas — diagrama de ER está ausente. |
| 32 | **Documentar enum `EstadoDePedido`** | Todos os 7 status com descrição e transições permitidas. |
| 33 | **Documentar permissões por classe** | Tabela de quais classes podem acessar quais endpoints. |

---

## Resumo por Prioridade

| Prioridade | Count | Itens |
|------------|-------|-------|
| 🔴 Crítico | 4 | 1, 2, 3, 4 |
| 🟡 Bugs | 4 | 5, 6, 7, 8 |
| 🟢 Melhorias | 6 | 9, 10, 11, 12, 13, 14 |
| 📋 Features | 16 | 15–30 |
| 📝 Docs | 3 | 31, 32, 33 |

**Total: 33 pendências**
