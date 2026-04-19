-- Migration 0010: Torna usuario_uuid opcional em pedidos
-- Pedidos agora podem ser criados sem usuário cadastrado (ex: pedidos balcão/convidado)

-- 1. Remover FK constraint (CASCADE DELETE impede NULL)
ALTER TABLE pedidos
DROP CONSTRAINT IF EXISTS pedidos_usuario_uuid_fkey;

-- 2. Tornar coluna nullable
ALTER TABLE pedidos
ALTER COLUMN usuario_uuid DROP NOT NULL;

-- 3. Recriar FK sem CASCADE, permitindo NULL
ALTER TABLE pedidos
ADD CONSTRAINT pedidos_usuario_uuid_fkey
FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE SET NULL;
