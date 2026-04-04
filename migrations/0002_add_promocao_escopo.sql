-- ============================================================================
-- MIGRATION: 002_add_promocao_escopo.sql
-- Projeto: Chickie - Adicionar escopo de promoção (loja, produto, categoria)
-- ============================================================================

ALTER TABLE promocoes ADD COLUMN IF NOT EXISTS tipo_escopo TEXT NOT NULL DEFAULT 'loja'
    CHECK (tipo_escopo IN ('loja', 'produto', 'categoria'));

ALTER TABLE promocoes ADD COLUMN IF NOT EXISTS produto_uuid UUID REFERENCES produtos(uuid) ON DELETE CASCADE;

ALTER TABLE promocoes ADD COLUMN IF NOT EXISTS categoria_uuid UUID REFERENCES categorias_produtos(uuid) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_promocoes_escopo ON promocoes(tipo_escopo);
CREATE INDEX IF NOT EXISTS idx_promocoes_produto ON promocoes(produto_uuid);
CREATE INDEX IF NOT EXISTS idx_promocoes_categoria ON promocoes(categoria_uuid);
