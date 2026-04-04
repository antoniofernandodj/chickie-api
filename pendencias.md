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

### ✅ Melhorias Concluídas

| # | Tarefa | Detalhe | Data |
|---|--------|---------|------|
| 14 | **Timestamps com tipo correto** | Models migrados de `String` para `chrono::DateTime<Utc>` para compatibilidade com PostgreSQL `TIMESTAMPTZ`. INSERT/UPDATE agora omitem `criado_em`/`atualizado_em` (usam defaults/triggers do DB). | 2026-04-05 |
| 15 | **Campos TIME com tipo correto** | `horario_abertura`/`horario_fechamento` (`loja`) e `abertura`/`fechamento` (`horarios_funcionamento`) migrados de `String` para `chrono::NaiveTime`. | 2026-04-05 |
| 16 | **Endpoint minhas lojas** | `GET /api/admin/minhas-lojas` lista lojas criadas pelo admin logado. Tabela `lojas` ganhou campo `criado_por UUID` (FK para `usuarios`). Migration `0003` criada. | 2026-04-05 |
| 17 | **Campos NUMERIC com tipo correto** | Todos os campos `f64`/`Option<f64>` mapeados para `NUMERIC` migrados para `rust_decimal::Decimal`. Afeta preço, nota, salario, taxa_entrega, valor_minimo, latitude/longitude, quantidade, total, subtotal, desconto em 10+ models. | 2026-04-05 |

## 📋 Funcionalidades Pendentes

| # | Feature | Detalhe | Prioridade |
|---|---------|---------|------------|
| 5 | **Atribuir entregador ao pedido** | Sem endpoint para vincular entregador a um pedido. | Alta |
| 6 | **Listar pedidos por usuário** | Endpoint para cliente ver seus próprios pedidos. | Média |
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
| ✅ Concluídas | 3 | 14, 15, 16 |
| 📋 Features | 7 | 5–11 |
| 📝 Docs | 2 | 12, 13 |

**Total: 13 pendências ativas, 3 concluídas recentemente**
