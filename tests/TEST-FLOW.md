# Test Execution Flow

## Visual Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    PHASE 1: SETUP                           │
│                  (MUST RUN FIRST)                           │
└─────────────────────────────────────────────────────────────┘

00-database-setup.clurl
    │
    ├─ RUN: chickie wipe
    │   └─→ Cleans all tables
    │
    └─ RUN: chickie migrate
        └─→ Applies all migrations
            │
            ▼
01-auth.clurl
    │
    ├─ Creates: admin_uuid (administrador)
    ├─ Creates: user_uuid (cliente)
    ├─ Creates: order_user_uuid
    ├─ Creates: review_user_uuid
    ├─ Creates: fav_user_uuid
    └─ Creates: staff_admin_uuid
        │
        ▼
02-lojas.clurl
    │
    ├─ Creates: main_store_uuid (using admin_uuid)
    ├─ Creates: product_store_uuid
    ├─ Creates: category_store_uuid
    └─ Creates: order_store_uuid
        │
        ▼
┌─────────────────────────────────────────────────────────────┐
│                 PHASE 2: DOMAIN CRUD                        │
│              (Uses users & stores from Phase 1)             │
└─────────────────────────────────────────────────────────────┘
        │
        ├─→ 03-produtos.clurl
        │   └─ Uses: {{product_store_uuid}}
        │   └─ Creates: categoria → produto → tests CRUD
        │
        ├─→ 04-categorias.clurl
        │   └─ Uses: {{category_store_uuid}}
        │   └─ Tests: categoria CRUD + pizza-mode
        │
        ├─→ 05-adicionais.clurl
        │   └─ Uses: {{main_store_uuid}}
        │   └─ Tests: adicional CRUD
        │
        ├─→ 06-pedidos.clurl
        │   └─ Uses: {{order_user_uuid}}, {{order_store_uuid}}
        │   └─ Tests: order creation, status updates, address
        │
        ├─→ 07-cupons.clurl
        │   └─ Uses: {{main_store_uuid}}
        │   └─ Tests: coupon CRUD
        │
        ├─→ 08-promocoes.clurl
        │   └─ Uses: {{main_store_uuid}}
        │   └─ Tests: promotion CRUD
        │
        ├─→ 09-avaliacoes.clurl
        │   └─ Uses: {{review_user_uuid}}, {{main_store_uuid}}
        │   └─ Tests: store review
        │
        ├─→ 10-favoritos.clurl
        │   └─ Uses: {{fav_user_uuid}}, {{main_store_uuid}}
        │   └─ Tests: favorite add/remove/verify
        │
        ├─→ 11-horarios.clurl
        │   └─ Uses: {{main_store_uuid}}
        │   └─ Tests: business hours CRUD
        │
        ├─→ 12-config-pedido.clurl
        │   └─ Uses: {{main_store_uuid}}
        │   └─ Tests: order configuration
        │
        ├─→ 13-ingredientes.clurl
        │   └─ Uses: {{main_store_uuid}}
        │   └─ Tests: ingredient CRUD
        │
        ▼
┌─────────────────────────────────────────────────────────────┐
│            PHASE 3: ADVANCED OPERATIONS                     │
│              (Builds on Phase 1-2)                          │
└─────────────────────────────────────────────────────────────┘
        │
        ├─→ 14-funcionarios.clurl
        │   └─ Uses: {{main_store_uuid}}
        │   └─ Creates: funcionario, entregador, cliente
        │
        ├─→ 15-enderecos-loja.clurl
        │   └─ Uses: {{main_store_uuid}}
        │   └─ Tests: store address CRUD
        │
        ├─→ 16-driver-assignment.clurl
        │   └─ Uses: {{main_store_uuid}}, {{entregador_uuid}}
        │   └─ Tests: assign/remove driver from order
        │
        ▼
┌─────────────────────────────────────────────────────────────┐
│         PHASE 4: ERRORS & INTEGRATION                       │
│                    (Final Tests)                            │
└─────────────────────────────────────────────────────────────┘
        │
        ├─→ 17-error-handling.clurl
        │   └─ Uses: {{admin_uuid}}, {{order_user_uuid}}
        │   └─ Tests: invalid operations, duplicates,
        │            bad transitions, missing resources
        │
        ├─→ 18-integration-order-flow.clurl
        │   └─ Uses: {{order_user_uuid}}, {{main_store_uuid}}
        │   └─ Tests: COMPLETE state machine:
        │        criado
        │        → aguardando_confirmacao_de_loja
        │        → confirmado_pela_loja
        │        → em_preparo
        │        → pronto
        │        → saiu_para_entrega
        │        → entregue (terminal)
        │
        └─→ 19-environment.clurl
            └─ Tests: DATABASE_URL, help commands
```

## Variable Capture Summary

### Phase 1: Users (01-auth.clurl)
```
admin_uuid         → administrador (can create stores)
user_uuid          → cliente (regular user)
order_user_uuid    → dedicated order user
review_user_uuid   → dedicated review user
fav_user_uuid      → dedicated favorites user
staff_admin_uuid   → second admin for staff tests
```

### Phase 1: Stores (02-lojas.clurl)
```
main_store_uuid       → primary store (used by most tests)
product_store_uuid    → dedicated for products
category_store_uuid   → dedicated for categories
order_store_uuid      → dedicated for orders
```

### Phase 2: Domain Entities
```
From 03-produtos:      categoria_uuid, produto_uuid
From 04-categorias:    categoria_uuid
From 05-adicionais:    adicional_uuid
From 06-pedidos:       pedido_uuid
From 07-cupons:        cupom_uuid
From 08-promocoes:     promocao_uuid
From 13-ingredientes:  ingrediente_uuid
From 14-funcionarios:  funcionario_uuid, entregador_uuid
From 15-enderecos:     endereco_uuid
From 16-driver:        driver_test_pedido_uuid
From 18-integration:   int_categoria_uuid, integration_pedido_uuid
From 17-errors:        invalid_transition_pedido_uuid
```

## Test Dependencies Matrix

| Test File | Requires | Creates | Used By |
|-----------|----------|---------|---------|
| 00-database | Nothing | Clean DB | 01 |
| 01-auth | 00 | 6 users | 02, 06, 09, 10, 14, 16, 17, 18 |
| 02-lojas | 01 | 4 stores | 03-16, 17, 18 |
| 03-produtos | 02 | produto | - |
| 04-categorias | 02 | categoria | - |
| 05-adicionais | 02 | adicional | - |
| 06-pedidos | 01, 02 | pedido | - |
| 07-cupons | 02 | cupom | - |
| 08-promocoes | 02 | promocao | - |
| 09-avaliacoes | 01, 02 | avaliacao | - |
| 10-favoritos | 01, 02 | favorita | - |
| 11-horarios | 02 | horario | - |
| 12-config | 02 | config | - |
| 13-ingredientes | 02 | ingrediente | - |
| 14-funcionarios | 01, 02 | funcionario, entregador | 16 |
| 15-enderecos | 02 | endereco | - |
| 16-driver | 01, 02, 14 | assignment | - |
| 17-errors | 01, 02 | error cases | - |
| 18-integration | 01, 02 | full flow | - |
| 19-environment | Nothing | - | - |

## Running All Tests

```bash
# From clurl directory
clurl all-tests.aggregation.clurl

# With verbose output
clurl --verbose all-tests.aggregation.clurl

# Stop on first failure
clurl --stop-on-fail all-tests.aggregation.clurl
```

## Runner Behavior

The aggregation file (`all-tests.aggregation.clurl`):

1. Lists all test files in `[group]` section (executed in order)
2. Runs each test file sequentially
3. **Stops on first failure** (configured in `[Options]` section)
4. Shows pass/fail for each file
5. Captures variables across files (auth → lojas → rest)

## Adding New Tests

1. Create file with next number: `20-my-feature.clurl`
2. Use captured variables: `{{main_store_uuid}}`, etc.
3. DON'T create users/stores (already exist)
4. Add to `all-tests.aggregation.clurl` in the `[group]` section
5. Update this diagram
