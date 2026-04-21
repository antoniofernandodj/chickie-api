# Relatório de Refatoração: Categorias Globais de Produtos

**Data:** 21 de Abril de 2026
**Autor:** Gemini CLI
**Status:** Concluído
**Objetivo:** Ajustar o modelo de dados para permitir categorias de produtos globais (compartilhadas entre todas as lojas) e implementar o seed inicial.

---

## 1. Visão Geral das Mudanças

A arquitetura do Chickie API foi atualizada para suportar categorias que não pertencem a uma loja específica (`loja_uuid IS NULL`). Estas categorias representam grupos genéricos como "Bebidas", "Pizzas" ou "Hambúrgueres", facilitando a padronização do catálogo para novas lojas.

## 2. Detalhamento Técnico por Camada

### 2.1 Banco de Dados (Migrations)

- **`migrations/0001_criar_tabelas.sql`**: 
    - Alterada a definição da tabela `categorias_produtos` para tornar a coluna `loja_uuid` opcional.
    - Mantida a chave estrangeira para `lojas(uuid)`, porém permitindo valores nulos.
- **`migrations/0011_categorias_ordem_unique.sql`**:
    - Atualizada a constraint `uq_categorias_loja_ordem`.
    - Utilizada a cláusula `UNIQUE NULLS NOT DISTINCT` (PostgreSQL 15+). Isso garante que a unicidade da coluna `ordem` seja respeitada mesmo quando `loja_uuid` é nulo, impedindo múltiplas categorias globais com a mesma ordem.

### 2.2 Camada de Domínio (`crates/core`)

- **Modelo (`models/categoria.rs`)**:
    - O campo `loja_uuid` na struct `CategoriaProdutos` foi migrado de `Uuid` para `Option<Uuid>`.
    - O construtor `CategoriaProdutos::new` foi atualizado para refletir essa mudança.
- **Portas (`ports/categoria_port.rs`)**:
    - Adicionado método `listar_globais`.
    - Métodos `proxima_ordem` e `reordenar` agora operam sobre `Option<Uuid>`, permitindo gestão de ordem tanto no escopo de loja quanto no escopo global.

### 2.3 Camada de Infraestrutura e Repositórios

- **Repositório (`repositories/categoria_produtos_repository.rs`)**:
    - **`buscar_por_loja`**: Refatorado para retornar categorias globais (`loja_uuid IS NULL`) em conjunto com as categorias específicas da loja solicitada.
    - **`proxima_ordem`**: Lógica de cálculo de `MAX(ordem)` ajustada para diferenciar entre categorias globais e locais.
    - **`reordenar`**: Implementada transação que suporta a atualização de ordem para categorias globais de forma isolada.

### 2.4 Camada de Serviço (`services/catalogo_service.rs`)

- Métodos de criação e gestão de categorias agora tratam `loja_uuid` como opcional onde apropriado.
- Adicionada validação explícita em `atualizar_categoria` e `deletar_categoria` para garantir que lojas não possam modificar ou excluir categorias globais por engano (verificação de `categoria.loja_uuid == Some(loja_uuid)`).

### 2.5 Camada de API (`crates/api`)

- **Handlers**: 
    - Os handlers existentes em `api/handlers/catalogo/` foram atualizados para passar `Some(loja_uuid)` nas operações de escrita.
    - **Novo Handler `listar_categorias_globais`**: Implementado endpoint público `GET /api/catalogo/categorias/globais` que retorna exclusivamente as categorias de sistema.
    - **Novo Handler `criar_categoria_global`**: Implementado endpoint `POST /api/admin/categorias/globais` protegido por `OwnerPermission`, permitindo que apenas o dono da plataforma cadastre categorias sem vínculo com loja.

---

## 3. Inicialização de Dados (Seed)

Foi implementado um mecanismo de "seeding" automático no arquivo `crates/core/src/database.rs`.

- **Função `seed_categorias_globais`**: Executada imediatamente após o processo de migrações.
- **Categorias Iniciais**: O sistema agora pré-popula o banco de dados com as seguintes categorias (se não existirem):
    1. Pizzas
    2. Hambúrgueres
    3. Bebidas
    4. Sobremesas
    5. Porções
    6. Massas
    7. Saladas
    8. Açaí
    9. Pastéis
    10. Quentinhas
    11. Quentinhas Fitness

---

## 4. Impacto e Próximos Passos

- **Navegação**: O endpoint `GET /api/catalogo/{loja_uuid}/categorias` agora retorna automaticamente a lista completa (Globais + Locais).
- **Consistência**: A constraint `UNIQUE NULLS NOT DISTINCT` previne inconsistências de ordenação na interface do usuário.
- **Melhoria Futura**: Avaliar a criação de um endpoint administrativo exclusivo para o `Owner` gerenciar as categorias globais separadamente.

---
**Chickie API Architecture Report**
