-- Migration 0012: Adicionar tabelas de chat
CREATE TABLE mensagens_chat (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pedido_uuid UUID REFERENCES pedidos(uuid),
    loja_uuid UUID NOT NULL REFERENCES lojas(uuid),
    usuario_uuid UUID NOT NULL REFERENCES usuarios(uuid),
    remetente_uuid UUID NOT NULL REFERENCES usuarios(uuid),
    texto TEXT NOT NULL,
    lida BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mensagens_chat_pedido ON mensagens_chat(pedido_uuid);
CREATE INDEX idx_mensagens_chat_loja_usuario ON mensagens_chat(loja_uuid, usuario_uuid);
