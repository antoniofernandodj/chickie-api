-- ============================================================================
-- MIGRATION: 001_initial_schema.sql
-- Projeto: Chickie - Sistema de Delivery
-- Banco: PostgreSQL 14+
-- ============================================================================

-- ============================================================================
-- EXTENSÕES NECESSÁRIAS
-- ============================================================================
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- FUNÇÕES PARA TRIGGERS (PostgreSQL requer função separada)
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.atualizado_em = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- TABELA: usuarios
-- ============================================================================
CREATE TABLE IF NOT EXISTS usuarios (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nome TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    senha_hash TEXT,
    telefone TEXT,
    celular TEXT NOT NULL,
    modo_de_cadastro TEXT NOT NULL DEFAULT 'email',
    classe TEXT NOT NULL DEFAULT 'cliente' CHECK (classe IN ('cliente', 'administrador', 'funcionario', 'entregador', 'owner')),
    ativo BOOLEAN NOT NULL DEFAULT TRUE,
    passou_pelo_primeiro_acesso BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_usuarios_email ON usuarios(email);
CREATE INDEX IF NOT EXISTS idx_usuarios_username ON usuarios(username);

-- Trigger para atualizar atualizado_em
CREATE TRIGGER trigger_usuarios_atualizado
BEFORE UPDATE ON usuarios
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- TABELA: lojas
-- ============================================================================
CREATE TABLE IF NOT EXISTS lojas (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nome TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    descricao TEXT,
    email TEXT NOT NULL,
    telefone TEXT,
    ativa BOOLEAN NOT NULL DEFAULT TRUE,
    logo_url TEXT,
    banner_url TEXT,
    horario_abertura TIME,
    horario_fechamento TIME,
    dias_funcionamento TEXT[],  -- PostgreSQL array: ['seg','ter','qua']
    tempo_preparo_min INTEGER,
    taxa_entrega NUMERIC(10,2),
    valor_minimo_pedido NUMERIC(10,2),
    raio_entrega_km NUMERIC(5,2),
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_lojas_slug ON lojas(slug);
CREATE INDEX IF NOT EXISTS idx_lojas_email ON lojas(email);
CREATE INDEX IF NOT EXISTS idx_lojas_ativa ON lojas(ativa);

CREATE TRIGGER trigger_lojas_atualizado
BEFORE UPDATE ON lojas
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- TABELA: clientes (relacionamento usuario-loja — usuarios favoritos da loja)
-- ============================================================================
CREATE TABLE IF NOT EXISTS clientes (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    usuario_uuid UUID NOT NULL,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    UNIQUE(usuario_uuid, loja_uuid)
);

CREATE INDEX IF NOT EXISTS idx_clientes_usuario ON clientes(usuario_uuid);
CREATE INDEX IF NOT EXISTS idx_clientes_loja ON clientes(loja_uuid);

-- ============================================================================
-- TABELA: lojas_favoritas (relacionamento usuario-loja — lojas favoritas do usuario)
-- ============================================================================
CREATE TABLE IF NOT EXISTS lojas_favoritas (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    usuario_uuid UUID NOT NULL,
    loja_uuid UUID NOT NULL,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    UNIQUE(usuario_uuid, loja_uuid)
);

CREATE INDEX IF NOT EXISTS idx_lojas_favoritas_usuario ON lojas_favoritas(usuario_uuid);
CREATE INDEX IF NOT EXISTS idx_lojas_favoritas_loja ON lojas_favoritas(loja_uuid);

-- ============================================================================
-- TABELA: categorias_produtos
-- ============================================================================
CREATE TABLE IF NOT EXISTS categorias_produtos (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT,
    ordem INTEGER,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_categorias_loja ON categorias_produtos(loja_uuid);

-- ============================================================================
-- TABELA: produtos
-- ============================================================================
CREATE TABLE IF NOT EXISTS produtos (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    categoria_uuid UUID NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT,
    preco NUMERIC(10,2) NOT NULL,
    imagem_url TEXT,
    disponivel BOOLEAN NOT NULL DEFAULT FALSE,
    tempo_preparo_min INTEGER,
    destaque BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    FOREIGN KEY (categoria_uuid) REFERENCES categorias_produtos(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_produtos_loja ON produtos(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_produtos_categoria ON produtos(categoria_uuid);
CREATE INDEX IF NOT EXISTS idx_produtos_disponivel ON produtos(disponivel);

CREATE TRIGGER trigger_produtos_atualizado
BEFORE UPDATE ON produtos
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- TABELA: adicionais
-- ============================================================================
CREATE TABLE IF NOT EXISTS adicionais (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT NOT NULL,
    preco NUMERIC(10,2) NOT NULL,
    disponivel BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_adicionais_loja ON adicionais(loja_uuid);

-- ============================================================================
-- TABELA: ingredientes
-- ============================================================================
CREATE TABLE IF NOT EXISTS ingredientes (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    nome TEXT NOT NULL,
    unidade_medida TEXT,
    quantidade NUMERIC(12,4) NOT NULL DEFAULT 0.0,
    preco_unitario NUMERIC(10,2) NOT NULL,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_ingredientes_loja ON ingredientes(loja_uuid);

CREATE TRIGGER trigger_ingredientes_atualizado
BEFORE UPDATE ON ingredientes
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- TABELA: enderecos_usuario
-- ============================================================================
CREATE TABLE IF NOT EXISTS enderecos_usuario (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    usuario_uuid UUID NOT NULL,
    cep TEXT,
    logradouro TEXT NOT NULL,
    numero TEXT NOT NULL,
    complemento TEXT,
    bairro TEXT NOT NULL,
    cidade TEXT NOT NULL,
    estado TEXT NOT NULL,
    latitude NUMERIC(10,8),
    longitude NUMERIC(11,8),
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_enderecos_usuario_usuario ON enderecos_usuario(usuario_uuid);

-- ============================================================================
-- TABELA: entregadores
-- ============================================================================
CREATE TABLE IF NOT EXISTS entregadores (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    usuario_uuid UUID NOT NULL,
    veiculo TEXT,
    placa TEXT,
    disponivel BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_entregadores_loja ON entregadores(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_entregadores_disponivel ON entregadores(disponivel);

-- ============================================================================
-- TABELA: funcionarios
-- ============================================================================
CREATE TABLE IF NOT EXISTS funcionarios (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    usuario_uuid UUID NOT NULL,
    cargo TEXT,
    salario NUMERIC(10,2),
    data_admissao DATE,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_funcionarios_loja ON funcionarios(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_funcionarios_usuario ON funcionarios(usuario_uuid);

-- ============================================================================
-- TABELA: horarios_funcionamento
-- ============================================================================
CREATE TABLE IF NOT EXISTS horarios_funcionamento (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    dia_semana INTEGER NOT NULL CHECK (dia_semana >= 0 AND dia_semana <= 6),
    abertura TIME NOT NULL,
    fechamento TIME NOT NULL,
    ativo BOOLEAN NOT NULL DEFAULT TRUE,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    UNIQUE(loja_uuid, dia_semana)
);

CREATE INDEX IF NOT EXISTS idx_horarios_loja ON horarios_funcionamento(loja_uuid);

-- ============================================================================
-- TABELA: configuracoes_pedidos_loja
-- ============================================================================
CREATE TABLE IF NOT EXISTS configuracoes_pedidos_loja (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL UNIQUE,
    max_partes INTEGER NOT NULL DEFAULT 4 CHECK (max_partes >= 1),
    tipo_calculo TEXT NOT NULL DEFAULT 'mais_caro' CHECK (tipo_calculo IN ('media_ponderada', 'mais_caro')),
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE TRIGGER trigger_config_pedidos_atualizado
BEFORE UPDATE ON configuracoes_pedidos_loja
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- TABELA: pedidos (antes de enderecos_entrega, pois há FK)
-- ============================================================================
CREATE TABLE IF NOT EXISTS pedidos (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    usuario_uuid UUID NOT NULL,
    status TEXT NOT NULL DEFAULT 'criado' CHECK (
        status IN (
            'criado',
            'aguardando_confirmacao_de_loja',
            'confirmado_pela_loja',
            'em_preparo',
            'pronto_para_retirada',
            'saiu_para_entrega',
            'entregue'
        )
    ),
    total NUMERIC(10,2) NOT NULL DEFAULT 0.0,
    subtotal NUMERIC(10,2) NOT NULL DEFAULT 0.0,
    taxa_entrega NUMERIC(10,2) NOT NULL DEFAULT 0.0,
    desconto NUMERIC(10,2),
    forma_pagamento TEXT NOT NULL,
    observacoes TEXT,
    tempo_estimado_min INTEGER,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_pedidos_usuario ON pedidos(usuario_uuid);
CREATE INDEX IF NOT EXISTS idx_pedidos_loja ON pedidos(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_pedidos_status ON pedidos(status);
CREATE INDEX IF NOT EXISTS idx_pedidos_criado_em ON pedidos(criado_em);

CREATE TRIGGER trigger_pedidos_atualizado
BEFORE UPDATE ON pedidos
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- TABELA: enderecos_loja
-- ============================================================================
CREATE TABLE IF NOT EXISTS enderecos_loja (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    cep TEXT,
    logradouro TEXT NOT NULL,
    numero TEXT NOT NULL,
    complemento TEXT,
    bairro TEXT NOT NULL,
    cidade TEXT NOT NULL,
    estado TEXT NOT NULL,
    latitude NUMERIC(10,8),
    longitude NUMERIC(11,8),
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_enderecos_loja_loja ON enderecos_loja(loja_uuid);

-- ============================================================================
-- TABELA: enderecos_entrega (depende de pedidos)
-- ============================================================================
CREATE TABLE IF NOT EXISTS enderecos_entrega (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    pedido_uuid UUID NOT NULL,
    cep TEXT,
    logradouro TEXT NOT NULL,
    numero TEXT NOT NULL,
    complemento TEXT,
    bairro TEXT NOT NULL,
    cidade TEXT NOT NULL,
    estado TEXT NOT NULL,
    latitude NUMERIC(10,8),
    longitude NUMERIC(11,8),
    FOREIGN KEY (pedido_uuid) REFERENCES pedidos(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_enderecos_entrega_pedido ON enderecos_entrega(pedido_uuid);

-- ============================================================================
-- TABELA: itens_pedido
-- ============================================================================
CREATE TABLE IF NOT EXISTS itens_pedido (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pedido_uuid UUID NOT NULL,
    loja_uuid UUID NOT NULL,
    quantidade INTEGER NOT NULL DEFAULT 1,
    observacoes TEXT,
    FOREIGN KEY (pedido_uuid) REFERENCES pedidos(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_itens_pedido ON itens_pedido(pedido_uuid);

-- ============================================================================
-- TABELA: partes_item_pedido
-- ============================================================================
CREATE TABLE IF NOT EXISTS partes_item_pedido (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    item_uuid UUID,
    produto_uuid UUID NOT NULL,
    produto_nome TEXT NOT NULL,
    preco_unitario NUMERIC(10,2) NOT NULL,
    posicao INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (item_uuid) REFERENCES itens_pedido(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_partes_item ON partes_item_pedido(item_uuid);
CREATE INDEX IF NOT EXISTS idx_partes_posicao ON partes_item_pedido(item_uuid, posicao);

-- ============================================================================
-- TABELA: adicionais_item_pedido
-- ============================================================================
CREATE TABLE IF NOT EXISTS adicionais_item_pedido (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    item_uuid UUID NOT NULL,
    loja_uuid UUID NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT NOT NULL,
    preco NUMERIC(10,2) NOT NULL,
    FOREIGN KEY (item_uuid) REFERENCES itens_pedido(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_adicionais_item ON adicionais_item_pedido(item_uuid);

-- ============================================================================
-- TABELA: avaliacoes_loja
-- ============================================================================
CREATE TABLE IF NOT EXISTS avaliacoes_loja (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    usuario_uuid UUID NOT NULL,
    nota NUMERIC(3,2) NOT NULL CHECK (nota >= 0 AND nota <= 5),
    comentario TEXT,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_avaliacoes_loja_loja ON avaliacoes_loja(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_avaliacoes_loja_usuario ON avaliacoes_loja(usuario_uuid);

-- ============================================================================
-- TABELA: avaliacoes_produto
-- ============================================================================
CREATE TABLE IF NOT EXISTS avaliacoes_produto (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    usuario_uuid UUID NOT NULL,
    produto_uuid UUID NOT NULL,
    nota NUMERIC(3,2) NOT NULL CHECK (nota >= 0 AND nota <= 5),
    comentario TEXT,
    descricao TEXT NOT NULL,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    FOREIGN KEY (produto_uuid) REFERENCES produtos(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_avaliacoes_produto_produto ON avaliacoes_produto(produto_uuid);
CREATE INDEX IF NOT EXISTS idx_avaliacoes_produto_usuario ON avaliacoes_produto(usuario_uuid);

-- ============================================================================
-- TABELA: cupons
-- ============================================================================
CREATE TABLE IF NOT EXISTS cupons (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    codigo TEXT NOT NULL,
    descricao TEXT NOT NULL,
    tipo_desconto TEXT NOT NULL CHECK (tipo_desconto IN ('percentual', 'valor_fixo', 'frete_gratis')),
    valor_desconto NUMERIC(10,2),
    valor_minimo NUMERIC(10,2),
    data_validade TIMESTAMPTZ NOT NULL,
    limite_uso INTEGER,
    uso_atual INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'ativo' CHECK (status IN ('ativo', 'inativo', 'expirado', 'esgotado')),
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    UNIQUE(loja_uuid, codigo)
);

CREATE INDEX IF NOT EXISTS idx_cupons_loja ON cupons(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_cupons_codigo ON cupons(codigo);
CREATE INDEX IF NOT EXISTS idx_cupons_status ON cupons(status);

-- ============================================================================
-- TABELA: uso_cupons
-- ============================================================================
CREATE TABLE IF NOT EXISTS uso_cupons (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    cupom_uuid UUID NOT NULL,
    usuario_uuid UUID NOT NULL,
    pedido_uuid UUID NOT NULL,
    valor_desconto NUMERIC(10,2) NOT NULL,
    usado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (cupom_uuid) REFERENCES cupons(uuid) ON DELETE CASCADE,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE,
    FOREIGN KEY (pedido_uuid) REFERENCES pedidos(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_uso_cupons_cupom ON uso_cupons(cupom_uuid);
CREATE INDEX IF NOT EXISTS idx_uso_cupons_usuario ON uso_cupons(usuario_uuid);
CREATE INDEX IF NOT EXISTS idx_uso_cupons_pedido ON uso_cupons(pedido_uuid);

-- ============================================================================
-- TABELA: promocoes
-- ============================================================================
CREATE TABLE IF NOT EXISTS promocoes (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT NOT NULL,
    tipo_desconto TEXT NOT NULL CHECK (tipo_desconto IN ('percentual', 'valor_fixo', 'frete_gratis')),
    valor_desconto NUMERIC(10,2),
    valor_minimo NUMERIC(10,2),
    data_inicio TIMESTAMPTZ NOT NULL,
    data_fim TIMESTAMPTZ NOT NULL,
    dias_semana_validos INTEGER[],  -- PostgreSQL array: [0,1,2] para seg,ter,qua
    status TEXT NOT NULL DEFAULT 'ativo' CHECK (status IN ('ativo', 'inativo', 'expirado', 'esgotado')),
    prioridade INTEGER NOT NULL DEFAULT 1,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_promocoes_loja ON promocoes(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_promocoes_status ON promocoes(status);
CREATE INDEX IF NOT EXISTS idx_promocoes_prioridade ON promocoes(loja_uuid, prioridade);

-- ============================================================================
-- VIEWS: Views úteis para consultas frequentes
-- ============================================================================

-- View: Resumo de pedidos por loja
CREATE OR REPLACE VIEW view_resumo_pedidos_loja AS
SELECT 
    l.uuid AS loja_uuid,
    l.nome AS loja_nome,
    COUNT(p.uuid) AS total_pedidos,
    COALESCE(SUM(p.total), 0) AS valor_total,
    COALESCE(AVG(p.total), 0) AS ticket_medio,
    COUNT(CASE WHEN p.status = 'entregue' THEN 1 END) AS pedidos_entregues,
    COUNT(CASE WHEN p.status IN ('criado', 'em_preparo') THEN 1 END) AS pedidos_pendentes
FROM lojas l
LEFT JOIN pedidos p ON l.uuid = p.loja_uuid
GROUP BY l.uuid;

-- View: Produtos mais vendidos
CREATE OR REPLACE VIEW view_produtos_mais_vendidos AS
SELECT 
    p.uuid AS produto_uuid,
    p.nome AS produto_nome,
    l.nome AS loja_nome,
    COUNT(ip.uuid) AS vezes_pedido,
    COALESCE(SUM(ip.quantidade), 0) AS quantidade_total
FROM produtos p
JOIN partes_item_pedido pip ON p.uuid = pip.produto_uuid
JOIN itens_pedido ip ON pip.item_uuid = ip.uuid
JOIN lojas l ON p.loja_uuid = l.uuid
GROUP BY p.uuid, l.nome
ORDER BY vezes_pedido DESC;

-- ============================================================================
-- DADOS INICIAIS: Inserts opcionais para teste
-- ============================================================================

-- Descomente para inserir dados de teste
-- INSERT INTO usuarios (uuid, nome, username, email, senha_hash, celular, modo_de_cadastro)
-- VALUES (
--     '00000000-0000-0000-0000-000000000001', 
--     'Admin', 
--     'admin', 
--     'admin@chickie.com', 
--     'hashed_password', 
--     '11999999999', 
--     'email'
-- );

-- ============================================================================
-- FIM DA MIGRATION
-- ============================================================================