-- ============================================================================
-- MIGRATION: 0005_add_entregador_uuid_pedidos.sql
-- Adiciona campo entregador_uuid na tabela pedidos para vincular entregador
-- ============================================================================

ALTER TABLE pedidos
ADD COLUMN IF NOT EXISTS entregador_uuid UUID REFERENCES entregadores(uuid) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_pedidos_entregador ON pedidos(entregador_uuid);
