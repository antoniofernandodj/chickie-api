CREATE TABLE ordem_categorias_de_produtos (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loja_uuid UUID NOT NULL,
    categoria_uuid UUID NOT NULL,
    ordem INTEGER NOT NULL,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_ocp_loja_categoria UNIQUE (loja_uuid, categoria_uuid),
    CONSTRAINT uq_ocp_loja_ordem UNIQUE (loja_uuid, ordem) DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (loja_uuid) REFERENCES lojas(uuid) ON DELETE CASCADE,
    FOREIGN KEY (categoria_uuid) REFERENCES categorias_produtos(uuid) ON DELETE CASCADE
);

INSERT INTO ordem_categorias_de_produtos (loja_uuid, categoria_uuid, ordem)
SELECT loja_uuid, uuid, ordem
FROM categorias_produtos
WHERE loja_uuid IS NOT NULL;

ALTER TABLE categorias_produtos DROP COLUMN ordem;
