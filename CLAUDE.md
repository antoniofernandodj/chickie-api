# Chickie — CLAUDE.md

## Visão geral

API REST em Rust (Axum + Tokio) para o projeto Chickie, Um sistema de pedidos e entregas
Banco de dados PostgreSQL via `sqlx`.
Deploy via Docker no Dokploy.

## Stack

- Rust 1.88, edição 2024
- Axum 0.8 — framework HTTP
- Tokio — runtime async
- sqlx — pool de conexões
- tracing + tracing-subscriber — logging
- serde / serde_json — serialização

## Estrutura de módulos

- `main.rs` — bootstrap, inicialização do tracing, bind do servidor
- `database.rs` — criação do pool PostgreSQL
- `models/` — structs de domínio com Serialize/Deserialize
- `repositories/` — acesso direto ao banco (queries SQL)
- `services/` — regras de negócio, orquestra repositories
- `use_cases` — casos de uso, injetam services e usuario
- `api/` — handlers Axum, rotas, AppState

## Microserviços

- Chickie: sistema de pedidos e entregas
- ChickieSupplyChain: sistema de relacionamento com fornecedores
- ChickieAnalytics: sistema de análise de dados
- ChickieAdmin: sistema de administração e gerenciamento
- ChickieAuth: sistema de autenticação e autorização
- ChickiePayment: sistema de pagamento
- ChickiePushNotification: sistema de notificações push
- ChickieWorker: sistema de tarefas assíncronas
- ChickieUI: sistema de interface do usuário

## Arquitetura

- Arquitetura Hexagonal
- Arquitetura Limpa
- Arquitetura de Microserviços
- Arquitetura de Componentes
- Arquitetura de Domínio
- Arquitetura de Dados
- Arquitetura de Testes

## Convenções

### Logging

Usar sempre `tracing` — nunca `println!` ou `eprintln!` fora do main.
Níveis: `info!` para fluxo normal, `warn!` para situações recuperáveis,
`error!` para falhas, `debug!` para detalhes de desenvolvimento.

### Erros

Handlers retornam `impl IntoResponse` com tupla `(StatusCode, Json)`.
Nunca fazer `.unwrap()` em código de produção fora do main bootstrap.

### Rotas

Todas as rotas da API vivem sob `/api` — configurado no nest do main.
Rota raiz `/` é apenas health check (handler_ok).
404 genérico via fallback (handler_404).

### AppState

Estado global compartilhado via `Arc<AppState>`.
Injetado nos handlers via `State(s): State<Arc<AppState>>`.

## Variáveis de ambiente

APP_PORT: padrão 3000
DATABASE_URL: string de conexão PostgreSQL interna ao Dokploy
RUST_LOG=info   # debug em desenvolvimento

## Comandos úteis

cargo run                  # rodar localmente
cargo test                 # testes
cargo build --release      # build de produção
docker build -t chickie .  # build da imagem

## O que evitar

- Não adicionar estado mutável global fora do AppState
- Não expor rotas sem passar pelo nest /api (exceto / e fallback)
- Não usar `unwrap()` em código de produção fora do main bootstrap

## Dominio da aplicação

A aplicação se destina a modelar um sistema de pedidos e entregas de comida e posteriormente um
sistema de relacionamento com fornecedores (supply chain).

### Entidades

- Adicional: Representam ingredientes que podem ser adicionados
à produtos gerais, como queijo, cebola, etc.
- AvaliacaoDeLoja: Sempre realizado por um usuario.
- AvaliacaoDeProduto: Sempre realizado por um usuario.
- CategoriaProdutos: Categorias de produtos, como bebidas, pizzas, hambúrgueres, etc.
- Cliente: Representa um usuario que segue uma loja.
- EnderecoEntrega: Endereço a ser usado em uma entrega de pedido.
- Entregador:
- HorarioDeFuncionamento:
- Ingrediente: Representa um ingrediente que pode ser usado em produtos gerais.
- Loja: Representa uma loja que vende produtos, um tenant.
- ParteDoPedido: Representa uma parte do pedido, como uma fatia de pizza de um sabor específico.
Essa entidade foi pensada para casos onde pizzarias vendem pizzas de vários sabores, e um pedido pode ter várias fatias de pizzas de sabores diferentes.
- ItemDoPedido: Representa um item do pedido, como uma pizza, um hambúrguer, etc. Cada item pode ter várias partes, caso seja aplicável.
- Pedido: Representa um pedido de um cliente para uma loja, a ser entregue ou em um endereço
específico ou retirado em loja.
- Produto: Representa um produto que pode ser vendido por uma loja, como uma pizza, um hambúrguer, etc.
- Cupom: Representa um cupom de desconto que pode ser usado em um pedido.
- UsoCupom: Representa o uso de um cupom em um pedido.
- Promocao: Representa uma promoção que pode ser aplicada a um
produto ou categoria de produtos para dias da semana específicos. precisa corrigir, o modelo atual
aplica para toda a loja
- Usuario: Representa um usuario do sistema, que pode ser um cliente, entregador ou administrador. um administrador é permitido cadastrar uma ou mais lojas

### Relacionamentos

- A definir depois

### Fluxo de uso

- Um usuario se cadastra como administrador. Este administrador cadasta a sua loja,
 inserindo dados cadastrais, logo, catálogo de categorias de produtos, produtos, ingredientes, adicionais, horários de funcionamento, promoções, slug, etc.
- produtos podem ser removidos ou inativados por um administrador de uma loja. pedidos inativos não são exibidos para o usuario.
- adicionais podem ser removidos ou inativados por um administrador de uma loja. adicionais inativos não são exibidos para o usuario.
- Um usuario se cadastra como cliente para uma certa loja para fazer tracking de lojas preferidas, e pode acessar as lojas cadastradas, e fazer pedidos.
- Um usuario acessa a página da loja e pode acessar catálogos e fazer pedidos
- O pedido, no ato da sua criação, pode ser fornececido um cupom, que, caso válido, computará um desconto numérico.
- Um pedido assim que criado é disponibilizado para a cozinha, de modo a poder ser preparado. Neste momento, o mesmo entra em estado EM_PREPARO
- Assim que o preparo for finalizado, o entregador será atribuído ao pedido, e o mesmo entra em estado A_CAMINHO
- O entregador chega ao local de entrega e o pedido entra em estado ENTREGUE
- O usuario pode avaliar o pedido, a loja e o entregador.
- O usuario pode avaliar o produto somente atravém de um pedido em situação de autenticado,
 de modo a evitar que um usuario avalie um produto que não consumiu.
- O usuario pode usar um cupom de desconto em um pedido
- O usuario pode adicionar um adicional a uma parte de pedido. por exemplo, adicionar cebola à
 uma fatia de pizza portuguesa mas não à fatia de mussarela
- O usuario não pode adicinonar ingredientes, os ingredientes são usados apenas de forma a
 descrever o produto. o usuario pode adicionar comentários à produtos e ao pedido no geral
- O usuario é solicitado a adicionar um endereço de entrega no ato do cadastro, mas também pode adicionar um endereço de entrega no ato do pedido, caso o endereço de entrega cadastrado não seja o desejado
- O entregador pode ser cadastrado por um administrador de uma loja, ou por um administrador do sistema, que pode cadastrar entregadores para várias lojas
- O entregador pode ser desativado por um administrador de uma loja, ou por um administrador do sistema, que pode desativar entregadores para várias lojas
- O entregador pode ser reativado por um administrador de uma loja, ou por um administrador do sistema, que pode reativar entregadores para várias lojas
- O entregador pode ser removido por um administrador de uma loja, ou por um administrador do sistema, que pode remover entregadores para várias lojas
- um usuario pode atualizar seus dados cadastrais ou remover a sua conta. remover a conta não remove de fato, marca ela como a_remover=datetime.now() + um_mes, e quando isso é feito, após um mês um scheduler inativa a conta com excluída=true para não mais ser possível um login ser realizado. o login é "sujo" de modo ao email poder ser reutilizado alterado depois
- o mesmo para uma loja

- Como o pedido deve ser:
 Pedido {
    observacoes: String,
    itens: [
        ItemDoPedido {
            produto: Produto,
            partes: [
                ParteDoItemDoPedido {
                    adicionais: [
                        Adicional
                    ],
                    observacoes: String
                }
            ]
        }
    ]
 }