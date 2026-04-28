CREATE TABLE pre_cadastro (
    token TEXT PRIMARY KEY,
    dados JSONB NOT NULL,
    expira_em TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_pre_cadastro_expira_em ON pre_cadastro (expira_em);
