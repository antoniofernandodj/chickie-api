-- Migration 0013: WhatsApp Integration Tables

-- 1. Tabela de Vínculo WhatsApp-Usuário
CREATE TABLE IF NOT EXISTS user_whatsapp_bindings (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES usuarios(uuid) ON DELETE CASCADE,
    phone_number TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    verification_code_hash TEXT,
    verification_expires_at TIMESTAMPTZ,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, phone_number)
);

CREATE INDEX idx_user_whatsapp_bindings_phone ON user_whatsapp_bindings(phone_number);
CREATE INDEX idx_user_whatsapp_bindings_user ON user_whatsapp_bindings(user_id);

CREATE TRIGGER trigger_user_whatsapp_bindings_atualizado
BEFORE UPDATE ON user_whatsapp_bindings
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- 2. Tabela de Mensagens Processadas (Idempotência)
CREATE TABLE IF NOT EXISTS processed_twilio_messages (
    message_sid TEXT PRIMARY KEY,
    processed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Note: whatsapp_conversations will be stored in Redis as per spec point 8.2, 
-- but we could also use a table if Redis is unavailable.
-- Since the user asked to use the existing pedidos table for guest flow, 
-- and we already have 'contato' and nullable 'usuario_uuid', we are good there.
