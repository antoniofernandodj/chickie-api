-- Migration 0006: Consolidar estrutura aninhada de pedidos em JSONB
-- Remove tabelas itens_pedido, partes_item_pedido, adicionais_item_pedido
-- e substitui por coluna JSONB 'itens' na tabela pedidos

-- 1. Adicionar coluna JSONB para armazenar itens/partes/adicionais
ALTER TABLE pedidos
ADD COLUMN itens JSONB NOT NULL DEFAULT '[]'::jsonb;

-- 2. Migrar dados existentes das tabelas relacionais para JSONB
-- Estrutura do JSON (coluna `itens` em `pedidos`):
-- Pedido não tem partes diretamente — partes pertencem a cada ItemPedido.
-- [
--   {                            <- ItemPedido
--     "uuid": "...",
--     "pedido_uuid": "...",
--     "loja_uuid": "...",
--     "quantidade": 1,
--     "observacoes": "...",
--     "partes": [                <- partes do item (não do pedido)
--       {                        <- ParteDeItemPedido
--         "uuid": "...",
--         "loja_uuid": "...",
--         "produto_uuid": "...",
--         "produto_nome": "...",
--         "preco_unitario": 0.00,
--         "posicao": 1,
--         "adicionais": [        <- adicionais da parte
--           {
--             "uuid": "...",
--             "loja_uuid": "...",
--             "nome": "...",
--             "descricao": "...",
--             "preco": 0.00
--           }
--         ]
--       }
--     ]
--   }
-- ]

DO $$
DECLARE
    pedido_record RECORD;
    item_record RECORD;
    parte_record RECORD;
    adicional_record RECORD;
    itens_json JSONB := '[]'::jsonb;
    partes_json JSONB := '[]'::jsonb;
    adicionais_item_json JSONB := '[]'::jsonb;
    adicionais_parte_json JSONB := '[]'::jsonb;
BEGIN
    -- Iterar sobre todos os pedidos existentes
    FOR pedido_record IN SELECT uuid FROM pedidos WHERE itens = '[]'::jsonb
    LOOP
        -- Resetar JSON collectors
        itens_json := '[]'::jsonb;
        
        -- Buscar itens deste pedido
        FOR item_record IN 
            SELECT * FROM itens_pedido 
            WHERE pedido_uuid = pedido_record.uuid 
            ORDER BY criado_em ASC
        LOOP
            -- Buscar adicionais do item
            adicionais_item_json := '[]'::jsonb;
            FOR adicional_record IN 
                SELECT * FROM adicionais_item_pedido 
                WHERE item_uuid = item_record.uuid
            LOOP
                adicionais_item_json := adicionais_item_json || jsonb_build_object(
                    'uuid', adicional_record.uuid,
                    'item_uuid', adicional_record.item_uuid,
                    'loja_uuid', adicional_record.loja_uuid,
                    'nome', adicional_record.nome,
                    'descricao', adicional_record.descricao,
                    'preco', adicional_record.preco
                );
            END LOOP;
            
            -- Buscar partes deste item
            partes_json := '[]'::jsonb;
            FOR parte_record IN 
                SELECT * FROM partes_item_pedido 
                WHERE item_uuid = item_record.uuid 
                ORDER BY posicao ASC
            LOOP
                -- Buscar adicionais desta parte
                adicionais_parte_json := '[]'::jsonb;
                FOR adicional_record IN 
                    SELECT * FROM adicionais_item_pedido 
                    WHERE item_uuid = parte_record.uuid
                LOOP
                    adicionais_parte_json := adicionais_parte_json || jsonb_build_object(
                        'uuid', adicional_record.uuid,
                        'item_uuid', adicional_record.item_uuid,
                        'loja_uuid', adicional_record.loja_uuid,
                        'nome', adicional_record.nome,
                        'descricao', adicional_record.descricao,
                        'preco', adicional_record.preco
                    );
                END LOOP;
                
                -- Construir parte com adicionais
                partes_json := partes_json || jsonb_build_object(
                    'uuid', parte_record.uuid,
                    'loja_uuid', parte_record.loja_uuid,
                    'produto_uuid', parte_record.produto_uuid,
                    'produto_nome', parte_record.produto_nome,
                    'preco_unitario', parte_record.preco_unitario,
                    'posicao', parte_record.posicao,
                    'adicionais', adicionais_parte_json
                );
            END LOOP;
            
            -- Construir item com partes e adicionais
            itens_json := itens_json || jsonb_build_object(
                'uuid', item_record.uuid,
                'pedido_uuid', item_record.pedido_uuid,
                'loja_uuid', item_record.loja_uuid,
                'quantidade', item_record.quantidade,
                'observacoes', item_record.observacoes,
                'partes', partes_json,
                'adicionais', adicionais_item_json
            );
        END LOOP;
        
        -- Atualizar pedido com JSON consolidado
        UPDATE pedidos 
        SET itens = itens_json 
        WHERE uuid = pedido_record.uuid;
    END LOOP;
END $$;

-- 3. Adicionar GIN index para consultas eficientes dentro do JSONB
CREATE INDEX idx_pedidos_itens_gin ON pedidos USING GIN (itens);

-- 4. Remover tabelas relacionais antigas (em cascata)
-- Nota: Primeiro removemos constraints FK que apontam para estas tabelas
DROP TABLE IF EXISTS adicionais_item_pedido CASCADE;
DROP TABLE IF EXISTS partes_item_pedido CASCADE;
DROP TABLE IF EXISTS itens_pedido CASCADE;

-- 5. Atualizar trigger de updated_at para incluir coluna itens
-- (já existe trigger_pedidos_atualizado, não precisa recriar)
