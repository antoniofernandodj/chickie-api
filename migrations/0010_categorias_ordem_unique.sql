-- Migration 0010: Tornar ordem de categorias obrigatória e única por loja

-- 1. Normalizar ordens existentes — reatribuir sequencialmente por loja
WITH ranked AS (
    SELECT uuid,
           ROW_NUMBER() OVER (PARTITION BY loja_uuid ORDER BY COALESCE(ordem, 99999), criado_em) AS nova_ordem
    FROM categorias_produtos
)
UPDATE categorias_produtos cp
SET ordem = ranked.nova_ordem::integer
FROM ranked
WHERE cp.uuid = ranked.uuid;

-- 2. Tornar ordem NOT NULL
ALTER TABLE categorias_produtos ALTER COLUMN ordem SET NOT NULL;

-- 3. Adicionar constraint UNIQUE (loja_uuid, ordem) como DEFERRABLE INITIALLY DEFERRED
--    para permitir reordenação atômica dentro de uma transação
ALTER TABLE categorias_produtos
ADD CONSTRAINT uq_categorias_loja_ordem
UNIQUE NULLS NOT DISTINCT (loja_uuid, ordem) DEFERRABLE INITIALLY DEFERRED;
