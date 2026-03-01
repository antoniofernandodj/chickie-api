-- ============================================================================
-- MIGRATION: 001_initial_schema.sql
-- Projeto: Chickie - Sistema de Delivery
-- Banco: SQLite
-- ============================================================================

-- ============================================================================
-- TABELA: usuarios
-- ============================================================================
CREATE TABLE IF NOT EXISTS usuarios (
    uuid TEXT PRIMARY KEY NOT NULL,
    nome TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    senha_hash TEXT,
    telefone TEXT,
    celular TEXT NOT NULL,
    modo_de_cadastro TEXT NOT NULL DEFAULT 'email',
    ativo BOOLEAN NOT NULL DEFAULT TRUE,
    passou_pelo_primeiro_acesso BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TEXT NOT NULL,
    atualizado_em TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_usuarios_email ON usuarios(email);
CREATE INDEX IF NOT EXISTS idx_usuarios_username ON usuarios(username);

-- ============================================================================
-- TABELA: lojas
-- ============================================================================
CREATE TABLE IF NOT EXISTS lojas (
    uuid TEXT PRIMARY KEY NOT NULL,
    nome TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    descricao TEXT,
    email TEXT NOT NULL,
    telefone TEXT,
    ativa BOOLEAN NOT NULL DEFAULT TRUE,
    logo_url TEXT,
    banner_url TEXT,
    horario_abertura TEXT,
    horario_fechamento TEXT,
    dias_funcionamento TEXT,
    tempo_preparo_min INTEGER,
    taxa_entrega REAL,
    valor_minimo_pedido REAL,
    raio_entrega_km REAL,
    criado_em TEXT NOT NULL,
    atualizado_em TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_lojas_slug ON lojas(slug);
CREATE INDEX IF NOT EXISTS idx_lojas_email ON lojas(email);
CREATE INDEX IF NOT EXISTS idx_lojas_ativa ON lojas(ativa);

-- ============================================================================
-- TABELA: clientes (relacionamento usuario-loja)
-- ============================================================================
CREATE TABLE IF NOT EXISTS clientes (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    usuario_uuid TEXT NOT NULL,
    criado_em TEXT NOT NULL,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    UNIQUE(usuario_uuid, loja_uuid)
);

CREATE INDEX IF NOT EXISTS idx_clientes_usuario ON clientes(usuario_uuid);
CREATE INDEX IF NOT EXISTS idx_clientes_loja ON clientes(loja_uuid);

-- ============================================================================
-- TABELA: categorias_produtos
-- ============================================================================
CREATE TABLE IF NOT EXISTS categorias_produtos (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT,
    ordem INTEGER,
    criado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_categorias_loja ON categorias_produtos(loja_uuid);

-- ============================================================================
-- TABELA: produtos
-- ============================================================================
CREATE TABLE IF NOT EXISTS produtos (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    categoria_uuid TEXT NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT,
    preco REAL NOT NULL,
    imagem_url TEXT,
    disponivel BOOLEAN NOT NULL DEFAULT FALSE,
    tempo_preparo_min INTEGER,
    destaque BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TEXT NOT NULL,
    atualizado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    FOREIGN KEY (categoria_uuid) REFERENCES categorias_produtos(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_produtos_loja ON produtos(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_produtos_categoria ON produtos(categoria_uuid);
CREATE INDEX IF NOT EXISTS idx_produtos_disponivel ON produtos(disponivel);

-- ============================================================================
-- TABELA: adicionais
-- ============================================================================
CREATE TABLE IF NOT EXISTS adicionais (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT NOT NULL,
    preco REAL NOT NULL,
    disponivel BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_adicionais_loja ON adicionais(loja_uuid);

-- ============================================================================
-- TABELA: ingredientes
-- ============================================================================
CREATE TABLE IF NOT EXISTS ingredientes (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    nome TEXT NOT NULL,
    unidade_medida TEXT,
    quantidade REAL NOT NULL DEFAULT 0.0,
    preco_unitario REAL NOT NULL,
    criado_em TEXT NOT NULL,
    atualizado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_ingredientes_loja ON ingredientes(loja_uuid);

-- ============================================================================
-- TABELA: enderecos_loja
-- ============================================================================
CREATE TABLE IF NOT EXISTS enderecos_loja (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    cep TEXT,
    logradouro TEXT NOT NULL,
    numero TEXT NOT NULL,
    complemento TEXT,
    bairro TEXT NOT NULL,
    cidade TEXT NOT NULL,
    estado TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_enderecos_loja_loja ON enderecos_loja(loja_uuid);

-- ============================================================================
-- TABELA: enderecos_usuario
-- ============================================================================
CREATE TABLE IF NOT EXISTS enderecos_usuario (
    uuid TEXT PRIMARY KEY NOT NULL,
    usuario_uuid TEXT NOT NULL,
    cep TEXT,
    logradouro TEXT NOT NULL,
    numero TEXT NOT NULL,
    complemento TEXT,
    bairro TEXT NOT NULL,
    cidade TEXT NOT NULL,
    estado TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_enderecos_usuario_usuario ON enderecos_usuario(usuario_uuid);

-- ============================================================================
-- TABELA: enderecos_entrega
-- ============================================================================
CREATE TABLE IF NOT EXISTS enderecos_entrega (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    pedido_uuid TEXT NOT NULL,
    cep TEXT,
    logradouro TEXT NOT NULL,
    numero TEXT NOT NULL,
    complemento TEXT,
    bairro TEXT NOT NULL,
    cidade TEXT NOT NULL,
    estado TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    FOREIGN KEY (pedido_uuid) REFERENCES pedidos(uuid) ON DELETE CASCADE
);

-- ============================================================================
-- TABELA: entregadores
-- ============================================================================
CREATE TABLE IF NOT EXISTS entregadores (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    nome TEXT NOT NULL,
    telefone TEXT,
    veiculo TEXT,
    placa TEXT,
    disponivel BOOLEAN NOT NULL DEFAULT FALSE,
    criado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_entregadores_loja ON entregadores(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_entregadores_disponivel ON entregadores(disponivel);

-- ============================================================================
-- TABELA: funcionarios
-- ============================================================================
CREATE TABLE IF NOT EXISTS funcionarios (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    nome TEXT NOT NULL,
    email TEXT,
    cargo TEXT,
    salario REAL,
    data_admissao TEXT,
    criado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_funcionarios_loja ON funcionarios(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_funcionarios_email ON funcionarios(email);

-- ============================================================================
-- TABELA: horarios_funcionamento
-- ============================================================================
CREATE TABLE IF NOT EXISTS horarios_funcionamento (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    dia_semana INTEGER NOT NULL CHECK (dia_semana >= 0 AND dia_semana <= 6),
    abertura TEXT NOT NULL,
    fechamento TEXT NOT NULL,
    ativo BOOLEAN NOT NULL DEFAULT TRUE,
    criado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    UNIQUE(loja_uuid, dia_semana)
);

CREATE INDEX IF NOT EXISTS idx_horarios_loja ON horarios_funcionamento(loja_uuid);

-- ============================================================================
-- TABELA: configuracoes_pedidos_loja
-- ============================================================================
CREATE TABLE IF NOT EXISTS configuracoes_pedidos_loja (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL UNIQUE,
    max_partes INTEGER NOT NULL DEFAULT 4 CHECK (max_partes >= 1),
    tipo_calculo TEXT NOT NULL DEFAULT 'mais_caro' CHECK (tipo_calculo IN ('media_ponderada', 'mais_caro')),
    criado_em TEXT NOT NULL,
    atualizado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

-- ============================================================================
-- TABELA: pedidos
-- ============================================================================
CREATE TABLE IF NOT EXISTS pedidos (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    usuario_uuid TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'criado' CHECK (
        status IN ('criado', 'aguardando_confirmacao_de_loja', 'confirmado_pela_loja', 'em_preparo', 'pronto_para_retirada', 'saiu_para_entrega', 'entregue')
    ),
    total REAL NOT NULL DEFAULT 0.0,
    subtotal REAL NOT NULL DEFAULT 0.0,
    taxa_entrega REAL NOT NULL DEFAULT 0.0,
    desconto REAL,
    forma_pagamento TEXT NOT NULL,
    observacoes TEXT,
    tempo_estimado_min INTEGER,
    criado_em TEXT NOT NULL,
    atualizado_em TEXT NOT NULL,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_pedidos_usuario ON pedidos(usuario_uuid);
CREATE INDEX IF NOT EXISTS idx_pedidos_loja ON pedidos(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_pedidos_status ON pedidos(status);
CREATE INDEX IF NOT EXISTS idx_pedidos_criado_em ON pedidos(criado_em);

-- ============================================================================
-- TABELA: itens_pedido
-- ============================================================================
CREATE TABLE IF NOT EXISTS itens_pedido (
    uuid TEXT PRIMARY KEY NOT NULL,
    pedido_uuid TEXT NOT NULL,
    loja_uuid TEXT NOT NULL,
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
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    item_uuid TEXT,
    produto_uuid TEXT NOT NULL,
    produto_nome TEXT NOT NULL,
    preco_unitario REAL NOT NULL,
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
    uuid TEXT PRIMARY KEY NOT NULL,
    item_uuid TEXT NOT NULL,
    loja_uuid TEXT NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT NOT NULL,
    preco REAL NOT NULL,
    FOREIGN KEY (item_uuid) REFERENCES itens_pedido(uuid) ON DELETE CASCADE,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_adicionais_item ON adicionais_item_pedido(item_uuid);

-- ============================================================================
-- TABELA: avaliacoes_loja
-- ============================================================================
CREATE TABLE IF NOT EXISTS avaliacoes_loja (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    usuario_uuid TEXT NOT NULL,
    nota REAL NOT NULL CHECK (nota >= 0 AND nota <= 5),
    comentario TEXT,
    criado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_avaliacoes_loja_loja ON avaliacoes_loja(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_avaliacoes_loja_usuario ON avaliacoes_loja(usuario_uuid);

-- ============================================================================
-- TABELA: avaliacoes_produto
-- ============================================================================
CREATE TABLE IF NOT EXISTS avaliacoes_produto (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    usuario_uuid TEXT NOT NULL,
    produto_uuid TEXT NOT NULL,
    nota REAL NOT NULL CHECK (nota >= 0 AND nota <= 5),
    comentario TEXT,
    descricao TEXT NOT NULL,
    criado_em TEXT NOT NULL,
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
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    codigo TEXT NOT NULL,
    descricao TEXT NOT NULL,
    tipo_desconto TEXT NOT NULL CHECK (tipo_desconto IN ('percentual', 'valor_fixo', 'frete_gratis')),
    valor_desconto REAL,
    valor_minimo REAL,
    data_validade TEXT NOT NULL,
    limite_uso INTEGER,
    uso_atual INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'ativo' CHECK (status IN ('ativo', 'inativo', 'expirado', 'esgotado')),
    criado_em TEXT NOT NULL,
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
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    cupom_uuid TEXT NOT NULL,
    usuario_uuid TEXT NOT NULL,
    pedido_uuid TEXT NOT NULL,
    valor_desconto REAL NOT NULL,
    usado_em TEXT NOT NULL,
    FOREIGN KEY (cupom_uuid) REFERENCES cupons(uuid) ON DELETE CASCADE,
    FOREIGN KEY (usuario_uuid) REFERENCES usuarios(uuid) ON DELETE CASCADE,
    FOREIGN KEY (pedido_uuid) REFERENCES pedidos(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_uso_cupons_cupom ON uso_cupons(cupom_uuid);
CREATE INDEX IF NOT EXISTS idx_uso_cupons_usuario ON uso_cupons(usuario_uuid);

-- ============================================================================
-- TABELA: promocoes
-- ============================================================================
CREATE TABLE IF NOT EXISTS promocoes (
    uuid TEXT PRIMARY KEY NOT NULL,
    loja_uuid TEXT NOT NULL,
    nome TEXT NOT NULL,
    descricao TEXT NOT NULL,
    tipo_desconto TEXT NOT NULL CHECK (tipo_desconto IN ('percentual', 'valor_fixo', 'frete_gratis')),
    valor_desconto REAL,
    valor_minimo REAL,
    data_inicio TEXT NOT NULL,
    data_fim TEXT NOT NULL,
    dias_semana_validos TEXT,
    status TEXT NOT NULL DEFAULT 'ativo' CHECK (status IN ('ativo', 'inativo', 'expirado', 'esgotado')),
    prioridade INTEGER NOT NULL DEFAULT 1,
    criado_em TEXT NOT NULL,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_promocoes_loja ON promocoes(loja_uuid);
CREATE INDEX IF NOT EXISTS idx_promocoes_status ON promocoes(status);
CREATE INDEX IF NOT EXISTS idx_promocoes_prioridade ON promocoes(loja_uuid, prioridade);

-- ============================================================================
-- TRIGGERS: Atualização automática de atualizado_em
-- ============================================================================
CREATE TRIGGER IF NOT EXISTS trigger_usuarios_atualizado
AFTER UPDATE ON usuarios
BEGIN
    UPDATE usuarios SET atualizado_em = datetime('now') WHERE uuid = NEW.uuid;
END;

CREATE TRIGGER IF NOT EXISTS trigger_lojas_atualizado
AFTER UPDATE ON lojas
BEGIN
    UPDATE lojas SET atualizado_em = datetime('now') WHERE uuid = NEW.uuid;
END;

CREATE TRIGGER IF NOT EXISTS trigger_produtos_atualizado
AFTER UPDATE ON produtos
BEGIN
    UPDATE produtos SET atualizado_em = datetime('now') WHERE uuid = NEW.uuid;
END;

CREATE TRIGGER IF NOT EXISTS trigger_ingredientes_atualizado
AFTER UPDATE ON ingredientes
BEGIN
    UPDATE ingredientes SET atualizado_em = datetime('now') WHERE uuid = NEW.uuid;
END;

CREATE TRIGGER IF NOT EXISTS trigger_pedidos_atualizado
AFTER UPDATE ON pedidos
BEGIN
    UPDATE pedidos SET atualizado_em = datetime('now') WHERE uuid = NEW.uuid;
END;

CREATE TRIGGER IF NOT EXISTS trigger_config_pedidos_atualizado
AFTER UPDATE ON configuracoes_pedidos_loja
BEGIN
    UPDATE configuracoes_pedidos_loja SET atualizado_em = datetime('now') WHERE uuid = NEW.uuid;
END;

-- ============================================================================
-- VIEWS: Views úteis para consultas frequentes
-- ============================================================================

-- View: Resumo de pedidos por loja
CREATE VIEW IF NOT EXISTS view_resumo_pedidos_loja AS
SELECT 
    l.uuid AS loja_uuid,
    l.nome AS loja_nome,
    COUNT(p.uuid) AS total_pedidos,
    SUM(p.total) AS valor_total,
    AVG(p.total) AS ticket_medio,
    COUNT(CASE WHEN p.status = 'entregue' THEN 1 END) AS pedidos_entregues,
    COUNT(CASE WHEN p.status IN ('criado', 'em_preparo') THEN 1 END) AS pedidos_pendentes
FROM lojas l
LEFT JOIN pedidos p ON l.uuid = p.loja_uuid
GROUP BY l.uuid;

-- View: Produtos mais vendidos
CREATE VIEW IF NOT EXISTS view_produtos_mais_vendidos AS
SELECT 
    p.uuid AS produto_uuid,
    p.nome AS produto_nome,
    l.nome AS loja_nome,
    COUNT(ip.uuid) AS vezes_pedido,
    SUM(ip.quantidade) AS quantidade_total
FROM produtos p
JOIN itens_pedido ip ON p.uuid IN (
    SELECT produto_uuid FROM partes_item_pedido WHERE item_uuid = ip.uuid
)
JOIN lojas l ON p.loja_uuid = l.uuid
GROUP BY p.uuid
ORDER BY vezes_pedido DESC;

-- ============================================================================
-- DADOS INICIAIS: Inserts opcionais para teste
-- ============================================================================

-- Descomente para inserir dados de teste
-- INSERT INTO usuarios (uuid, nome, username, email, senha_hash, celular, modo_de_cadastro, criado_em, atualizado_em)
-- VALUES ('00000000-0000-0000-0000-000000000001', 'Admin', 'admin', 'admin@chickie.com', 'hashed_password', '11999999999', 'email', datetime('now'), datetime('now'));

-- ============================================================================
-- FIM DA MIGRATION
-- ============================================================================
