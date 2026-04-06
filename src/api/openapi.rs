use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use axum::Router;
use std::sync::Arc;
use crate::api::AppState;

use crate::models::{
    Usuario, Loja, Pedido, Produto, Cupom, Promocao, 
    AvaliacaoDeLoja, AvaliacaoDeProduto, EnderecoEntrega, EnderecoUsuario,
    LojaFavorita, Adicional, CategoriaProdutos, Ingrediente, Funcionario, 
    Entregador, HorarioFuncionamento, ConfiguracaoDePedidosLoja, Cliente,
    EstadoDePedido, ItemPedido, ParteDeItemPedido, AdicionalDeItemDePedido,
    EnderecoLoja, StatusCupom, UsoCupom, TipoEscopoPromocao,
    TipoCalculoPedido, ClasseUsuario,
};
use crate::api::dto::{
    CreateUsuarioRequest, CreateLojaRequest, 
    LoginRequest, AvaliarLojaRequest, AvaliarProdutoRequest,
    Claims,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Chickie API",
        description = r#"
API REST para sistema de pedidos e entregas de comida **Chickie**.

## Autenticação

A API usa **JWT (JSON Web Token)** para autenticação.

1. `POST /api/auth/signup` — cria usuário
2. `POST /api/auth/login` — autentica e recebe token  
3. Inclua o token: `Authorization: Bearer <token>`

## Classes de Usuário

| Classe | Descrição |
|--------|-----------|
| `cliente` | Padrão. Faz pedidos e avalia. |
| `administrador` | Cria e gerencia lojas, funcionários e entregadores. |
| `funcionario` | Funcionário de uma loja. |
| `entregador` | Entregador de uma loja. |
| `owner` | Dono da plataforma. Acesso total. |

## Máquina de Estados do Pedido

```
criado → aguardando_confirmacao_de_loja → confirmado_pela_loja
       → em_preparo → pronto_para_retirada → saiu_para_entrega → entregue
```

## Estrutura de um Pedido

Pedidos suportam partes personalizáveis (ex: pizza meio-a-meio):

```
Pedido
├── observacoes
├── itens[]
│   ├── quantidade
│   ├── observacoes
│   └── partes[]
│       ├── produto_nome
│       ├── preco_unitario
│       ├── posicao
│       └── adicionais[]
│           ├── nome
│           ├── descricao
│           └── preco
└── endereco_entrega
    ├── logradouro, numero, bairro, cidade, estado
    └── cep
```
"#,
        version = "0.1.0",
        contact(
            name = "Chickie Team",
        ),
        license(
            name = "MIT"
        ),
    ),
    servers(
        (url = "http://localhost:3000", description = "Local server"),
    ),
    paths(
        // Note: Path endpoints are manually defined in the OpenAPI spec
        // This section would normally reference handler functions with #[utoipa::path]
    ),
    components(
        schemas(
            // Request DTOs
            CreateUsuarioRequest,
            CreateLojaRequest,
            LoginRequest,
            AvaliarLojaRequest,
            AvaliarProdutoRequest,
            Claims,
            
            // Domain Models
            Usuario,
            Loja,
            Pedido,
            Produto,
            Cupom,
            Promocao,
            AvaliacaoDeLoja,
            AvaliacaoDeProduto,
            EnderecoEntrega,
            EnderecoUsuario,
            LojaFavorita,
            Adicional,
            CategoriaProdutos,
            Ingrediente,
            Funcionario,
            Entregador,
            HorarioFuncionamento,
            ConfiguracaoDePedidosLoja,
            Cliente,
            EstadoDePedido,
            ItemPedido,
            ParteDeItemPedido,
            AdicionalDeItemDePedido,
            EnderecoLoja,
            StatusCupom,
            UsoCupom,
            TipoEscopoPromocao,
            TipoCalculoPedido,
            ClasseUsuario,
        ),
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User registration, login, and auth"),
        (name = "Users", description = "User management"),
        (name = "Stores (Public)", description = "Public store listing and search"),
        (name = "Administration", description = "Admin-only endpoints for managing stores and staff"),
        (name = "Orders", description = "Order creation, listing, and status management"),
        (name = "Products", description = "Product CRUD operations"),
        (name = "Catalog", description = "Categories and add-ons management"),
        (name = "Marketing", description = "Coupons, reviews, and promotions"),
        (name = "Delivery Addresses", description = "Delivery addresses for orders"),
        (name = "User Addresses", description = "User saved addresses"),
        (name = "Favorites", description = "Favorite stores management"),
        (name = "Business Hours", description = "Store business hours"),
        (name = "Order Config", description = "Store order configuration"),
        (name = "Ingredients", description = "Ingredient management"),
        (name = "Employees", description = "Employee management"),
        (name = "Couriers", description = "Courier management"),
        (name = "Coupon Admin", description = "Admin coupon management"),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

pub fn swagger_router(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .merge(SwaggerUi::new("/api/docs/swagger-ui")
            .url("/api/docs/openapi.json", ApiDoc::openapi()))
        .with_state(s.clone())
}
