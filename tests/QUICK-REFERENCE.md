# Chickie CLI Tests - Quick Reference

## Run All Tests

```bash
clurl tests.clurl
```

## Common Options

```bash
# Verbose output
clurl --verbose tests.clurl

# Stop on first failure
clurl --stop-on-fail tests.clurl

# Run single test
clurl clurl/tests/01-auth.clurl
```

## Test Order (MUST be sequential)

```
00-database → 01-auth → 02-lojas → 03 to 19
```

## Key Files

| File | Purpose |
|------|---------|
| `tests.clurl` | Runs all tests in order |
| `tests/00-database-setup.clurl` | Wipes DB + migrations |
| `tests/01-auth.clurl` | Creates 6 test users |
| `tests/02-lojas.clurl` | Creates 4 test stores |
| `tests/17-error-handling.clurl` | Error cases |
| `tests/18-integration-order-flow.clurl` | Full order lifecycle |

## Adding New Tests

1. Create: `tests/20-my-feature.clurl`
2. Add to: `tests.clurl` in `[group]` section
3. Use: `{{main_store_uuid}}` (from test 02)

## Captured Variables

**From 01-auth.clurl:**
- `{{admin_uuid}}`
- `{{user_uuid}}`
- `{{order_user_uuid}}`
- `{{review_user_uuid}}`
- `{{fav_user_uuid}}`
- `{{staff_admin_uuid}}`

**From 02-lojas.clurl:**
- `{{main_store_uuid}}`
- `{{product_store_uuid}}`
- `{{category_store_uuid}}`
- `{{order_store_uuid}}`

## Stats

- **20 test files**
- **~90 test scenarios**
- **~280 assertions**
- **All 80+ CLI commands tested**
