-- ============================================================================
-- MIGRATION 0009: Add UNIQUE constraint on celular column
-- ============================================================================

-- Remove duplicates first (keep the oldest by uuid)
DELETE FROM usuarios
WHERE uuid NOT IN (
    SELECT MIN(uuid)
    FROM usuarios
    GROUP BY celular
);

-- Add UNIQUE constraint
ALTER TABLE usuarios
    ADD CONSTRAINT usuarios_celular_key UNIQUE (celular);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_usuarios_celular ON usuarios(celular) WHERE deletado = false;
