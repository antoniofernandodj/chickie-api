# Chickie - Projeto Context

## Visão Geral
Sistema de delivery/restaurante desenvolvido em Rust com Axum framework.

## Stack Tecnológica
- **Linguagem**: Rust 2024
- **Framework Web**: Axum 0.8.8
- **Banco de Dados**: PostgreSQL 15.2 + SQLx 0.8.6
- **Runtime**: Tokio
- **Autenticação**: JWT + bcrypt
- **Serialização**: Serde + serde_json

## Estrutura do Projeto
```
src/
├── main.rs              # Entry point, servidor Axum na porta 3000
├── database.rs          # Conexão PostgreSQL e pool
├── models/              # Entidades de domínio (21 modelos)
│   ├── usuario.rs, cliente.rs, funcionario.rs, entregador.rs
│   ├── loja.rs, produto.rs, categoria.rs, ingrediente.rs
│   ├── pedido.rs, parte_de_pedido.rs, pagamento.rs, metodo_de_pagamento.rs
│   ├── endereco.rs, avaliacao.rs, promocoes.rs, adicionais.rs
│   └── horarios_de_funcionamento.rs, dados_cadastro.rs, auth.rs
├── api/                 # Camada de API REST
│   ├── auth.rs          # Autenticação
│   ├── routers.rs       # Definição de rotas
│   ├── state.rs         # AppState compartilhado
│   ├── dto/             # Data Transfer Objects
│   ├── usecases/        # Casos de uso
│   └── [cupom, loja, pedido, produto, usuario]/
├── services/            # Lógica de negócio
└── repositories.rs      # Acesso a dados
```

## Domínio (Entidades Principais)
1. **Usuários**: usuario, cliente, funcionario, entregador
2. **Estabelecimento**: loja, categoria, produto, ingrediente, adicional
3. **Pedidos**: pedido, parte_de_pedido, pagamento, metodo_de_pagamento
4. **Operações**: endereco, avaliacao, promocoes (cupons), horarios_de_funcionamento

## Comandos Úteis
```bash
# Desenvolvimento
cargo run                    # Rodar servidor (porta 3000)
cargo test                   # Rodar testes
cargo check                  # Check rápido

# Docker
docker-compose up -d         # Subir PostgreSQL
docker-compose down          # Parar containers

# Variáveis de Ambiente
APP_PORT=3000                # Porta do servidor
DATABASE_URL=postgres://...  # URL do PostgreSQL
```

## Configuração Atual
- **Porta padrão**: 3000 (configurável via `APP_PORT`)
- **CORS**: Permissivo (todos os origins)
- **Database**: PostgreSQL 15.2 via Docker (user: myuser, pass: mypassword, db: mydatabase)

## Features Implementadas
- [x] Usuários e autenticação JWT
- [x] Lojas e produtos
- [x] Pedidos
- [x] Avaliações
- [x] Entregadores
- [x] Pagamentos
- [x] Endereços
- [x] Promoções/Cupons
- [x] Horários de funcionamento

## Possíveis Gaps (do README)
- [ ] Variações de produto (tamanhos P/M/G)
- [ ] Disponibilidade de produtos em tempo real
- [ ] Rastreamento de entrega em tempo real
- [ ] Notificações push
- [ ] Múltiplos endereços por cliente

## Padrões de Desenvolvimento
- Seguir arquitetura em camadas: `api` → `services` → `repositories`
- Usar `AppState` compartilhado via `Arc`
- Logs com `tracing` + `tracing-subscriber`
- Tratamento de erros com `anyhow`
- Migracões em `migrations/*.sql`
