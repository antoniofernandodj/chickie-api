# Chickie CLI Test Suite

Comprehensive test suite for the Chickie CLI using `.clurl` test files.

## ⚠️ Important: Test Execution Order

**Tests MUST run in order!** Each test file depends on data created by previous tests:

```
00-database-setup → 01-auth → 02-lojas → 03+ (all others)
```

- **00-database-setup.clurl**: Wipes database and applies migrations
- **01-auth.clurl**: Creates all test users (admin, regular, order, review, etc.)
- **02-lojas.clurl**: Creates all test stores using admin user
- **03-19**: All other tests use users and stores from steps 01-02

## Running Tests

### Run all tests (in order)

```bash
# From clurl directory
clurl all-tests.aggregation.clurl

# From project root
clurl clurl/all-tests.aggregation.clurl
```

### Run with verbose output

```bash
clurl --verbose all-tests.aggregation.clurl
```

### Stop on first failure

```bash
clurl --stop-on-fail all-tests.aggregation.clurl
```

### Run individual test file

```bash
clurl clurl/tests/00-database-setup.clurl
clurl clurl/tests/01-auth.clurl
clurl clurl/tests/02-lojas.clurl
# ... etc
```

## Test Organization

| # | File | Description | Dependencies |
|---|------|-------------|--------------|
| 00 | `database/setup.clurl` | Wipe DB + migrations | None |
| 01 | `auth.clurl` | Create test users | 00 |
| 02 | `lojas.clurl` | Create test stores | 01 |
| 03 | `produtos.clurl` | Product CRUD | 02 |
| 04 | `categorias.clurl` | Category CRUD | 02 |
| 05 | `adicionais.clurl` | Topping CRUD | 02 |
| 06 | `pedidos.clurl` | Order management | 01, 02 |
| 07 | `cupons.clurl` | Coupon management | 02 |
| 08 | `promocoes.clurl` | Promotion management | 02 |
| 09 | `avaliacoes.clurl` | Reviews | 01, 02 |
| 10 | `favoritos.clurl` | Favorites | 01, 02 |
| 11 | `horarios.clurl` | Business hours | 02 |
| 12 | `config-pedido.clurl` | Order config | 02 |
| 13 | `ingredientes.clurl` | Ingredients | 02 |
| 14 | `funcionarios.clurl` | Staff management | 01, 02 |
| 15 | `enderecos-loja.clurl` | Store addresses | 02 |
| 16 | `driver-assignment.clurl` | Driver assignment | 01, 02, 14 |
| 17 | `error-handling.clurl` | Error cases | 01, 02 |
| 18 | `integration-order-flow.clurl` | Full order lifecycle | 01, 02 |
| 19 | `environment.clurl` | Env vars & help | None |

## Test Categories

### Setup (Files 00-02)
- Database wipe and migration
- User creation (admin, cliente, etc.)
- Store creation (multiple test stores)

### Unit Tests (Files 03-15)
- Individual CRUD operations
- Input validation
- Output format checking

### Integration Tests (Files 16, 18)
- Driver assignment workflow
- Complete order lifecycle (state machine)

### Error Handling (File 17)
- Invalid inputs
- Duplicate entries
- Invalid state transitions
- Missing resources

## Writing New Tests

When adding new test files:

1. **Name with prefix**: Use `NN-name.clurl` format (e.g., `20-new-feature.clurl`)
2. **Use captured variables**: Reference UUIDs from auth/lojas tests
3. **No duplicate setup**: Don't create users/stores if already created
4. **Follow the pattern**: RUN → [Captures] → [Asserts]

Example:

```clurl
# Uses {{main_store_uuid}} from 02-lojas.clurl
RUN cargo run --quiet -p chickie-cli -- some-command --loja-uuid {{main_store_uuid}}

[Captures]
new_uuid: stdout regex /│\s+([0-9a-f-]{36})\s+│/

[Asserts]
exit-code == 0
stdout contains "expected text"
```

## Available Assertions

- `exit-code == N` / `exit-code != N`
- `stdout contains "text"` / `stderr contains "text"`
- `stdout notContains "text"` / `stderr notContains "text"`
- `stdout isEmpty` / `stderr isEmpty`
- `stdout isNotEmpty` / `stderr isNotEmpty`
- `stdout startsWith "text"` / `stderr startsWith "text"`
- `stdout endsWith "text"` / `stderr endsWith "text"`
- `stdout matches /regex/` / `stderr matches /regex/`
- `stdout line N contains "text"`
- `file "path" exists` / `file "path" notExists`

## Captured Variables

The following variables are captured in early tests and available for later use:

### From 01-auth.clurl
- `{{admin_uuid}}` - Admin user UUID
- `{{user_uuid}}` - Regular user UUID
- `{{order_user_uuid}}` - User for order tests
- `{{review_user_uuid}}` - User for review tests
- `{{fav_user_uuid}}` - User for favorite tests
- `{{staff_admin_uuid}}` - Another admin for staff tests

### From 02-lojas.clurl
- `{{main_store_uuid}}` - Main test store
- `{{product_store_uuid}}` - Store for product tests
- `{{category_store_uuid}}` - Store for category tests
- `{{order_store_uuid}}` - Store for order tests

## Notes

- **Tests create real data** in the database
- **Sequential execution** is mandatory - tests depend on each other
- **Test runner stops** on first failure to prevent cascade errors
- **Clean state**: File 00 wipes the database before starting
- **No isolation**: Tests share the same database state
- **CI/CD**: Consider running against a dedicated test database
