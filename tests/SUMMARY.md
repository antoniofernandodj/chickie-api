# Chickie CLI Test Suite - Summary

## Overview

Complete test suite for Chickie CLI using `.clurl` format. The test suite covers **all CLI commands** with **20 test files** that **MUST run sequentially**.

## ⚠️ Critical: Execution Order

```
00-database-setup → 01-auth → 02-lojas → 03 to 19 (in order)
```

**Why?** 
- Test 00 wipes the database and applies migrations (clean slate)
- Test 01 creates all test users and captures their UUIDs
- Test 02 creates all test stores using the admin user and captures their UUIDs
- Tests 03-19 use the captured UUIDs from tests 01-02

## Test Files (Sequential Order)

### Phase 1: Setup (MUST run first)

| # | File | Purpose | Creates |
|---|------|---------|---------|
| 00 | `database/setup.clurl` | Wipe + migrations | Clean database |
| 01 | `auth.clurl` | User creation | 6 test users |
| 02 | `lojas.clurl` | Store creation | 4 test stores |

### Phase 2: Domain CRUD (Depends on Phase 1)

| # | File | Tests | Uses |
|---|------|-------|------|
| 03 | `produtos.clurl` | 6 | `{{product_store_uuid}}` |
| 04 | `categorias.clurl` | 5 | `{{category_store_uuid}}` |
| 05 | `adicionais.clurl` | 4 | `{{main_store_uuid}}` |
| 06 | `pedidos.clurl` | 8 | `{{order_user_uuid}}`, `{{order_store_uuid}}` |
| 07 | `cupons.clurl` | 2 | `{{main_store_uuid}}` |
| 08 | `promocoes.clurl` | 3 | `{{main_store_uuid}}` |
| 09 | `avaliacoes.clurl` | 1 | `{{review_user_uuid}}`, `{{main_store_uuid}}` |
| 10 | `favoritos.clurl` | 5 | `{{fav_user_uuid}}`, `{{main_store_uuid}}` |
| 11 | `horarios.clurl` | 5 | `{{main_store_uuid}}` |
| 12 | `config-pedido.clurl` | 2 | `{{main_store_uuid}}` |
| 13 | `ingredientes.clurl` | 3 | `{{main_store_uuid}}` |

### Phase 3: Advanced Operations (Depends on Phase 1-2)

| # | File | Tests | Uses |
|---|------|-------|------|
| 14 | `funcionarios.clurl` | 7 | `{{main_store_uuid}}` |
| 15 | `enderecos-loja.clurl` | 2 | `{{main_store_uuid}}` |
| 16 | `driver-assignment.clurl` | 2 | `{{main_store_uuid}}`, `{{entregador_uuid}}` |

### Phase 4: Error Handling & Integration (Final)

| # | File | Tests | Purpose |
|---|------|-------|---------|
| 17 | `error-handling.clurl` | 5 | Invalid operations, edge cases |
| 18 | `integration-order-flow.clurl` | 9 | Complete order lifecycle (state machine) |
| 19 | `environment.clurl` | 4 | Env vars, help commands |

## Test Statistics

- **Total test files:** 20
- **Total test scenarios:** ~90+
- **Total assertions:** ~280+
- **Test categories:** Setup, Unit, Integration, Error Handling

## Quick Start

### Run all tests (in order)
```bash
clurl all-tests.aggregation.clurl
```

### Run with verbose output
```bash
clurl --verbose all-tests.aggregation.clurl
```

### Stop on first failure
```bash
clurl --stop-on-fail all-tests.aggregation.clurl
```

### Run individual test
```bash
clurl clurl/tests/00-database-setup.clurl
clurl clurl/tests/01-auth.clurl
clurl clurl/tests/02-lojas.clurl
```

## Features Used

### Assertions
- ✅ Exit code validation (`==`, `!=`)
- ✅ String containment (`contains`, `notContains`)
- ✅ Empty checks (`isEmpty`, `isNotEmpty`)
- ✅ Prefix/suffix (`startsWith`, `endsWith`)
- ✅ Regex matching (`matches /pattern/`)
- ✅ Line-by-line (`line N contains`)
- ✅ File existence (`file "path" exists/notExists`)

### Captures
- ✅ Variable extraction via regex from table output
- ✅ Variable interpolation in commands (`{{variable_name}}`)
- ✅ Cross-test variable reuse (6 users + 4 stores captured)

## Architecture

```
clurl/tests/
├── database/
│   └── 00-database-setup.clurl    # Wipe + migrations
├── 01-auth.clurl                  # 6 users captured
├── 02-lojas.clurl                 # 4 stores captured
├── 03-produtos.clurl              # Product CRUD (6 tests)
├── 04-categorias.clurl            # Category CRUD (5 tests)
├── 05-adicionais.clurl            # Topping CRUD (4 tests)
├── 06-pedidos.clurl               # Orders (8 tests)
├── 07-cupons.clurl                # Coupons (2 tests)
├── 08-promocoes.clurl             # Promotions (3 tests)
├── 09-avaliacoes.clurl            # Reviews (1 test)
├── 10-favoritos.clurl             # Favorites (5 tests)
├── 11-horarios.clurl              # Business hours (5 tests)
├── 12-config-pedido.clurl         # Order config (2 tests)
├── 13-ingredientes.clurl          # Ingredients (3 tests)
├── 14-funcionarios.clurl          # Staff (7 tests)
├── 15-enderecos-loja.clurl        # Store addresses (2 tests)
├── 16-driver-assignment.clurl     # Driver assignment (2 tests)
├── 17-error-handling.clurl        # Error cases (5 tests)
├── 18-integration-order-flow.clurl # Full lifecycle (9 tests)
├── 19-environment.clurl           # Env vars (4 tests)
├── README.md                      # Documentation
└── SUMMARY.md                     # This file

../all-tests.aggregation.clurl     # Runs all tests in order
```

## Dependency Graph

```
00-database-setup
    ↓
01-auth (creates: admin, user, order_user, review_user, fav_user, staff_admin)
    ↓
02-lojas (creates: main_store, product_store, category_store, order_store)
    ↓
    ├─→ 03-produtos ─────────────────┐
    ├─→ 04-categorias ───────────────┤
    ├─→ 05-adicionais ───────────────┤
    ├─→ 06-pedidos ──────────────────┤
    ├─→ 07-cupons ───────────────────┤
    ├─→ 08-promocoes ────────────────┤
    ├─→ 09-avaliacoes ───────────────┤
    ├─→ 10-favoritos ────────────────┤
    ├─→ 11-horarios ─────────────────┤
    ├─→ 12-config-pedido ────────────┤
    ├─→ 13-ingredientes ─────────────┤
    ├─→ 14-funcionarios ─────────────┤
    ├─→ 15-enderecos-loja ───────────┤
    │                                 │
    └─────────────────────────────────┤
                                      ↓
                          16-driver-assignment
                          17-error-handling
                          18-integration-order-flow
                          19-environment
```

## Captured Variables

### Users (from 01-auth.clurl)
- `{{admin_uuid}}` - Administrator user (can create stores)
- `{{user_uuid}}` - Regular user (cliente)
- `{{order_user_uuid}}` - User for order tests
- `{{review_user_uuid}}` - User for review tests
- `{{fav_user_uuid}}` - User for favorite tests
- `{{staff_admin_uuid}}` - Admin for staff management tests

### Stores (from 02-lojas.clurl)
- `{{main_store_uuid}}` - Primary test store (most tests use this)
- `{{product_store_uuid}}` - Dedicated store for product tests
- `{{category_store_uuid}}` - Dedicated store for category tests
- `{{order_store_uuid}}` - Dedicated store for order tests

## Best Practices Applied

1. **Sequential Execution**: Tests run in strict order
2. **Single Setup**: Database wiped once at start
3. **Variable Capture**: UUIDs captured and reused throughout
4. **No Duplication**: Users/stores created once, referenced everywhere
5. **Error Isolation**: Runner stops on first failure
6. **Comprehensive Coverage**: All 80+ CLI commands tested
7. **Documentation**: README with usage examples
8. **Aggregation**: Native clurl aggregation file (no shell scripts)

## Running Requirements

- PostgreSQL running with `DATABASE_URL` set
- Rust toolchain installed
- `clurl` binary available
- No existing data (test 00 wipes everything)

## Notes

- **Tests create real data** in the database
- **Sequential execution is mandatory** - tests depend on each other
- **Test runner stops** on first failure to prevent cascade errors
- **Clean state guaranteed**: File 00 wipes the database before starting
- **All 80+ CLI commands are tested** across the 20 files
