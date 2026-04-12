-- ============================================================================
-- MIGRATION 0007: Soft delete + campo ativo para usuarios e lojas
-- ============================================================================
-- Soft delete funciona assim:
-- 1. marquar_para_remocao: timestamp que indica quando foi marcado para remoção
-- 2. Scheduler checa diariamente e após 30 dias define deletado=true
-- 3. Queries normais excluem: WHERE deletado = false
-- 4. Campo ativo: permite desativar temporariamente sem deletar (ex: inadimplência)
-- ============================================================================

-- ============================================================================
-- USUARIOS: adicionar colunas de soft delete + ativo para bloqueio
-- ============================================================================
ALTER TABLE usuarios
    ADD COLUMN IF NOT EXISTS marcado_para_remocao TIMESTAMPTZ DEFAULT NULL,
    ADD COLUMN IF NOT EXISTS deletado BOOLEAN NOT NULL DEFAULT FALSE;

-- Index para performance na busca de usuários não deletados
CREATE INDEX IF NOT EXISTS idx_usuarios_deletado ON usuarios(deletado) WHERE deletado = false;

-- Index para consulta do scheduler (usuários marcados para remoção pendentes)
CREATE INDEX IF NOT EXISTS idx_usuarios_marcado_remocao ON usuarios(marcado_para_remocao)
    WHERE marcado_para_remocao IS NOT NULL AND deletado = FALSE;

COMMENT ON COLUMN usuarios.marcado_para_remocao IS 'Timestamp quando o usuário foi marcado para remoção. Após 30 dias, o scheduler define deletado=true.';
COMMENT ON COLUMN usuarios.deletado IS 'True quando o usuário foi permanentemente removido pelo scheduler (após 30 dias de marcação).';

-- ============================================================================
-- LOJAS: adicionar colunas de soft delete + ativo para bloqueio
-- ============================================================================
ALTER TABLE lojas
    ADD COLUMN IF NOT EXISTS marcado_para_remocao TIMESTAMPTZ DEFAULT NULL,
    ADD COLUMN IF NOT EXISTS deletado BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS ativo BOOLEAN NOT NULL DEFAULT TRUE;

-- Index para performance na busca de lojas não deletadas
CREATE INDEX IF NOT EXISTS idx_lojas_deletado ON lojas(deletado) WHERE deletado = false;

-- Index para consulta do scheduler (lojas marcadas para remoção pendentes)
CREATE INDEX IF NOT EXISTS idx_lojas_marcado_remocao ON lojas(marcado_para_remocao)
    WHERE marcado_para_remocao IS NOT NULL AND deletado = FALSE;

-- Index para busca de lojas ativas (não bloqueadas por inadimplência)
CREATE INDEX IF NOT EXISTS idx_lojas_ativo ON lojas(ativo) WHERE ativo = true;

COMMENT ON COLUMN lojas.marcado_para_remocao IS 'Timestamp quando a loja foi marcada para remoção. Após 30 dias, o scheduler define deletado=true.';
COMMENT ON COLUMN lojas.deletado IS 'True quando a loja foi permanentemente removida pelo scheduler (após 30 dias de marcação).';
COMMENT ON COLUMN lojas.ativo IS 'Permite desativar temporariamente a loja (ex: inadimplência). Diferente de deletado, é reversível pelo admin.';
