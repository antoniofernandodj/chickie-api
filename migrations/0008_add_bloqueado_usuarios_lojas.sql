-- ============================================================================
-- MIGRATION 0008: Add bloqueado field to usuarios and lojas tables
-- ============================================================================
-- This migration adds an explicit 'bloqueado' boolean field to both tables.
-- 'bloqueado = true' means the user/store is blocked and cannot login/operate.
-- This is clearer than 'ativo = false' which was ambiguous.
-- ============================================================================

-- ============================================================================
-- USUARIOS: adicionar coluna bloqueado
-- ============================================================================
ALTER TABLE usuarios
    ADD COLUMN IF NOT EXISTS bloqueado BOOLEAN NOT NULL DEFAULT FALSE;

-- Index para busca rápida de usuários bloqueados
CREATE INDEX IF NOT EXISTS idx_usuarios_bloqueado ON usuarios(bloqueado) WHERE bloqueado = true;

COMMENT ON COLUMN usuarios.bloqueado IS 'True quando o usuário está bloqueado e não pode fazer login. Diferente de ativo, é um bloqueio explícito.';

-- ============================================================================
-- LOJAS: adicionar coluna bloqueado
-- ============================================================================
ALTER TABLE lojas
    ADD COLUMN IF NOT EXISTS bloqueado BOOLEAN NOT NULL DEFAULT FALSE;

-- Index para busca rápida de lojas bloqueadas
CREATE INDEX IF NOT EXISTS idx_lojas_bloqueado ON lojas(bloqueado) WHERE bloqueado = true;

COMMENT ON COLUMN lojas.bloqueado IS 'True quando a loja está bloqueada e não pode operar. Diferente de ativa, é um bloqueio explícito.';
