-- ============================================================================
-- MIGRATION 0009: Add UNIQUE constraint on celular column
-- ============================================================================

-- Remove duplicates first (keep the row with the smallest uuid per celular)
DELETE FROM usuarios
WHERE uuid IN (
    SELECT u2.uuid
    FROM usuarios u1
    JOIN usuarios u2 ON u1.celular = u2.celular AND u1.uuid < u2.uuid
);

-- Add UNIQUE constraint
ALTER TABLE usuarios
    ADD CONSTRAINT usuarios_celular_key UNIQUE (celular);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_usuarios_celular ON usuarios(celular) WHERE deletado = false;
