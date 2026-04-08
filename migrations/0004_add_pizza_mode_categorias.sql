-- ============================================================================
-- MIGRATION: 0004_add_pizza_mode_categorias.sql
-- Adiciona campo pizza_mode na tabela categorias_produtos
-- ============================================================================

ALTER TABLE categorias_produtos
ADD COLUMN IF NOT EXISTS pizza_mode BOOLEAN NOT NULL DEFAULT FALSE;
