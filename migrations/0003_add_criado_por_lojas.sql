-- ============================================================================
-- MIGRATION: 003_add_criado_por_lojas
-- Adiciona campo criado_por para rastrear qual admin criou a loja
-- ============================================================================

ALTER TABLE lojas
ADD COLUMN IF NOT EXISTS criado_por UUID REFERENCES usuarios(uuid) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_lojas_criado_por ON lojas(criado_por);
