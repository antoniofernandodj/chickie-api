ALTER TABLE pedidos DROP CONSTRAINT IF EXISTS pedidos_status_check;

ALTER TABLE pedidos ADD CONSTRAINT pedidos_status_check CHECK (
    status IN (
        'criado',
        'aguardando_confirmacao_de_loja',
        'confirmado_pela_loja',
        'em_preparo',
        'pronto',
        'saiu_para_entrega',
        'entregue',
        'cancelado'
    )
);
