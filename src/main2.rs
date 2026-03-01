mod models;
mod database;
mod utils;
mod repositories;
mod services;

use models::{
    Pedido,
    Ingrediente,
    EnderecoLoja,
};
// use uuid::Uuid;

use repositories::{
    UsuarioRepository,
    LojaRepository,
    ProdutoRepository,
    PedidoRepository,
    CupomRepository,
    AdicionalRepository,
    CategoriaProdutosRepository,
    FuncionarioRepository,
    AvaliacaoDeLojaRepository,
    AvaliacaoDeProdutoRepository,
    EntregadorRepository,
    HorarioFuncionamentoRepository,
    PromocaoRepository,
    Repository as _
};
use uuid::Uuid;

use crate::{
    models::{
        ParteDeItemPedido,
        TipoCalculoPedido,
        calcular_preco_por_partes
    },
    repositories::{
        ClienteRepository, ConfiguracaoPedidosLojaRepository,
    }, services::{CatalogoService, LojaService, MarketingService, PedidoService, UsuarioService}
};


// use crate::repositories::Repository as _;


#[tokio::main]
async fn main() -> Result<(), String> {

    let pool = database::criar_pool().await.unwrap();

    // Inicializar repositórios
    let usuario_repo =
        UsuarioRepository::new(&pool);
    let loja_repo =
        LojaRepository::new(&pool);
    let produto_repo =
        ProdutoRepository::new(&pool);
    let pedido_repo =
        PedidoRepository::new(&pool);
    let cupom_repo =
        CupomRepository::new(&pool);
    let adicional_repo =
        AdicionalRepository::new(&pool);
    let avaliacoes_de_produto_repo =
        AvaliacaoDeProdutoRepository::new(&pool);
    let avaliacoes_de_loja_repo =
        AvaliacaoDeLojaRepository::new(&pool);
    let funcionario_repo =
        FuncionarioRepository::new(&pool);
    let categorias_de_produtos_repo = 
        CategoriaProdutosRepository::new(&pool);
    let entregador_repo = 
        EntregadorRepository::new(&pool);
    let promocao_repo = 
        PromocaoRepository::new(&pool);
    let horario_repo =
        HorarioFuncionamentoRepository::new(&pool);
    let config_partes_repo =
        ConfiguracaoPedidosLojaRepository::new(&pool);
    let cliente_repo =
        ClienteRepository::new(&pool);


    // 2. Inicialização dos Services
    let usuario_service = UsuarioService::new(
        &usuario_repo
    );

    let loja_service = LojaService::new(
        &loja_repo, 
        &config_partes_repo, 
        &horario_repo, 
        &funcionario_repo, 
        &entregador_repo,
        &cliente_repo
    );

    let catalogo_service = CatalogoService::new(
        &produto_repo,
        &categorias_de_produtos_repo,
        &adicional_repo
    );

    let pedido_service = PedidoService::new(
        &pedido_repo,
        &config_partes_repo,
        &cupom_repo,
        &promocao_repo,
    );

    let marketing_service = MarketingService::new(
        &cupom_repo, 
        &promocao_repo, 
        &avaliacoes_de_loja_repo, 
        &avaliacoes_de_produto_repo
    );

    // --- Usuário ---
    println!("--- Criando Usuário ---");
    let usuario = usuario_service.registrar(
        "Antonio Silva".into(),
        "antonio".into(), 
        "antonio@email.com".into(),
        "11999999999".into(),
        "email".into()
    )
    .await?;

    // Buscar usuário por email
    if let Some(u) = usuario_repo.buscar_por_email("antonio@email.com").await.unwrap() {
        println!("Usuário Corretamente cadastrado: {:?}", u);
    }

    // Service orquestra criação da loja + config + horários iniciais
    let loja = loja_service.criar_loja_completa(
        String::from("Padaria Central"),
        String::from("padaria-central"),
        String::from("contato@padaria.com"),
        Some(String::from("A melhor padaria")),
        Some(String::from("11988887777")),
        Some(String::from("07:00")),
        Some(String::from("22:00")),
        Some(String::from("1,2,3,4,5")),
        Some(30),
        Some(5.0),
        Some(20.0),
        Some(10.0),
        4,
        TipoCalculoPedido::MaisCaro
    ).await?;

    println!("loja: {:?}", loja);

    // Desativar domingo sem deletar
    horario_repo.definir_ativo(loja.uuid, 0, false).await.ok();

    loja_service.adicionar_cliente(&usuario, &loja).await?;

    // --- Catálogo (Produtos/Categorias/Adicionais) ---
    println!("\n--- Populando Catálogo ---");
    let adicional_bacon = catalogo_service.criar_adicional(
        "Bacon".into(),
        loja.uuid,
        "Bacon crocante".into(),
        5.0
    ).await?;
    let adicional_cheddar = catalogo_service.criar_adicional(
        "Cheddar".into(),
        loja.uuid,
        "Cheddar amarelo".into(),
        3.0
    ).await?;

    println!("adicional: {:?}, {:?}", adicional_bacon, adicional_cheddar);

    let categoria_pizzas = catalogo_service.criar_categoria(
        String::from("Pizzas"),
        Some(String::from("Produtos da categoria de pizzas")),
        loja.uuid,
        Some(2)
    ).await?;

    let categoria_hamburgueres = catalogo_service.criar_categoria(
        String::from("Hambúrgueres"),
        Some(String::from("Produtos da categoria de hamburgueres")),
        loja.uuid,
        Some(3)
    ).await?;

    let categoria_bebidas = catalogo_service.criar_categoria(
        String::from("Bebidas"),
        Some(String::from("Produtos da categoria de bebidas")),
        loja.uuid,
        Some(1)
    ).await?;

    let produto_pizza_calabresa = catalogo_service.criar_produto(
        "Pizza Calabresa".into(),
        Some(String::from("Pizza de calabresa com cebola")),
        39.90,
        categoria_pizzas.uuid,
        loja.uuid,
        Some(30)
    ).await?;

    let produto_pizza_catupiry = catalogo_service.criar_produto(
        "Pizza Catupiry".into(),
        Some(String::from("Pizza de calabresa com cebola")),
        39.90,
        categoria_pizzas.uuid,
        loja.uuid,
        Some(30)
    ).await?;

    let produto_pizza_quatro_queijos = catalogo_service.criar_produto(
        "Pizza Quatro Queijos".into(),
        Some(String::from("Pizza de calabresa com cebola")),
        39.90,
        categoria_pizzas.uuid,
        loja.uuid,
        Some(30)
    ).await?;

    let produto_pizza_portuguesa = catalogo_service.criar_produto(
        "Pizza Portuguesa".into(),
        Some(String::from("Pizza de portuguesa com cebola")),
        39.90,
        categoria_pizzas.uuid,
        loja.uuid,
        Some(30)
    ).await?;

    let produto_coca_cola = catalogo_service.criar_produto(
        "Coca-Cola Lata".into(),
        Some(String::from("Refrigerante 350ml")),
        6.5,
        categoria_pizzas.uuid,
        loja.uuid,
        Some(30)
    ).await?;

    let produto_hamburger = catalogo_service.criar_produto(
        "Hambúrguer artesanal".into(),
        Some(String::from("Pão brioche, carne 180g, queijo")),
        32.00,
        categoria_hamburgueres.uuid,
        loja.uuid,
        Some(30)
    ).await?;

    println!("categoria: {:?}", categoria_bebidas);
    println!("categoria: {:?}", categoria_pizzas);

    let mut pedido_1: Pedido = Pedido::new(
        usuario.uuid,
        loja.uuid,
        8.50,
        2.0,
        "PIX".to_string(),
        Some(String::from("Sem cebola, por favor")),
    );

    pedido_1.adicionar_item(
        2,
        Some(String::from("Sem açúcar")),
        vec![
            ParteDeItemPedido::new(&produto_coca_cola, 1)
        ]
    );

    println!("pedido: {:?}", pedido_1);

    let ingrediente: Ingrediente = Ingrediente::new(
        String::from("Tomate"),
        loja.uuid,
        None,
        12.5,
    );

    println!("ingrediente: {:?}", ingrediente);

    let endereco_loja: EnderecoLoja = EnderecoLoja::new(
        loja.uuid,
        Some("01000-000".into()),
        "Rua das Flores".into(),
        "100".into(),
        Some("Apto 12".into()),
        "Centro".into(),
        "São Paulo".into(),
        "SP".into(),
        None,
        None
    );

    println!("endereco_loja: {:?}", endereco_loja);

    let entregador = loja_service.adicionar_entregador(
        String::from("Carlos Lima"),
        loja.uuid,
        Some(String::from("11988887777")),
        Some(String::from("Moto")),
        Some(String::from("ABC1D23")),
    ).await?;

    println!("entregador: {:?}", entregador);

    let funcionario = loja_service.adicionar_funcionario(
        loja.uuid,
        String::from("Maria Silva"),
        Some(String::from("maria@email.com")),
        Some(String::from("Atendente")),
        Some(1800.0),
        None,
    ).await?;

    println!("funcionario: {:?}", funcionario);

    let mut pedido_2: Pedido = Pedido::new(
        usuario.uuid,
        loja.uuid,
        12.5,
        2.0,
        "Cartão".to_string(),
        Some(String::from("Entregar na portaria por favor")),
    );

    println!("{:?}", pedido_2);

    let pedido_2_uuid = pedido_2.adicionar_item(
        1,
        Some("Sem cebola".into()),
        vec![
            ParteDeItemPedido::new(&produto_pizza_portuguesa, 1)
        ]
    );

    for item in pedido_2.itens.iter_mut() {

        println!("item criado: {:?}", item);

        if item.uuid == pedido_2_uuid {

            for parte in item.partes.iter_mut() {  // ← MUDANÇA AQUI

                if parte.produto_uuid == produto_hamburger.uuid {
                    let adicional_1_uuid = parte
                        .adicionar_adicional(&adicional_bacon)
                        .expect("Não foi possível adicionar");

                    println!("UUID adicional 1: {:?}", adicional_1_uuid);
                }

                if parte.produto_uuid == produto_pizza_catupiry.uuid {
                    let adicional_2_uuid = parte
                        .adicionar_adicional(&adicional_cheddar)
                        .expect("Não foi possível adicionar");

                    println!("UUID adicional 2: {:?}", adicional_2_uuid);
                }
            }
        }
    }


    pedido_repo.criar(&pedido_1).await.unwrap();
    pedido_repo.criar(&pedido_2).await.unwrap();

    let avaliacao_produto = marketing_service.avaliar_produto(
        usuario.uuid,
        loja.uuid,
        produto_hamburger.uuid,
        Some(String::from("Produto excelente")),
        4.8,
        "teste".into()
    ).await?;

    let avaliacao_loja = marketing_service.avaliar_loja(
        loja.uuid,
        usuario.uuid,
        4.5,
        Some(String::from("Loja muito boa")),
    ).await?;

    println!("avaliacoes_de_loja: {:?}", avaliacao_loja);
    println!("avaliacoes_de_produto: {:?}", avaliacao_produto);

    let cupom = marketing_service.criar_cupom(
        loja.uuid,
        "BEMVINDO".into(),
        "20% de desconto".into(),
        "percentual".into(),
        Some(20.0),
        Some(30.0),
        "2026-12-31T23:59:59Z".into(),
        Some(100),
    ).await?;


    let promo_happy_hour = marketing_service.criar_promocao(
        loja.uuid,
        "Happy Hour".into(),
        "15% de desconto das 18h às 20h".into(),
        "percentual".into(),
        Some(15.0),
        Some(25.0),
        "2026-01-01T00:00:00Z".into(),
        "2026-12-31T23:59:59Z".into(),
        Some(vec![1, 2, 3, 4, 5]),
        1,
    ).await?;

    println!("{:?}", promo_happy_hour);
    println!("{:?}", cupom);

    // Montar um pedido de pizza com 3 partes
    let mut pedido_pizza = Pedido::new(
        usuario.uuid,
        loja.uuid, 0.0,
        5.0,
        "PIX".into(), None
    );

    pedido_pizza.adicionar_item(
        1,
        Some(String::from("Deixar na portaria")),
        vec![
            ParteDeItemPedido::new(&produto_pizza_calabresa, 1),
            ParteDeItemPedido::new(&produto_pizza_portuguesa, 2),
            ParteDeItemPedido::new(&produto_pizza_catupiry, 3),
            ParteDeItemPedido::new(&produto_pizza_quatro_queijos, 4),
        ]
    );

    pedido_repo.criar(&pedido_pizza).await.unwrap();

    // Busca config da loja e salva partes (valida max_partes internamente)
    let config_loja = config_partes_repo
        .buscar_por_loja(loja.uuid).await.unwrap().unwrap();


    for item_pedido in pedido_pizza.itens {
        // Verificação de cálculo sem banco (testes unitários):
        let preco_media  = calcular_preco_por_partes(
            &item_pedido.partes,
            &TipoCalculoPedido::MediaPonderada
        );

        let preco_caro   = calcular_preco_por_partes(
            &item_pedido.partes,
            &TipoCalculoPedido::MaisCaro
        );

        let preco_loja   = calcular_preco_por_partes(
            &item_pedido.partes,
            &config_loja.tipo_calculo
        );

        println!("Média: {:.2} | Mais caro: {:.2}", preco_media, preco_caro);
        println!("Loja: {:.2}", preco_loja);
    }

    let mut pedido_com_cupom = Pedido::new(
        usuario.uuid,
        loja.uuid, 
        0.0, // subtotal será calculado
        5.0, 
        "PIX".into(), 
        None
    );

    // Adiciona itens caros para atingir o mínimo do cupom
    pedido_com_cupom.adicionar_item(2, None, vec![
        ParteDeItemPedido::new(&produto_pizza_calabresa, 1),
        ParteDeItemPedido::new(&produto_pizza_portuguesa, 2),
    ]);

    // Processa: calcula subtotal, verifica promoções (Happy Hour) e aplica cupom "BEMVINDO"
    let result = pedido_service.processar_e_finalizar_pedido(
        &mut pedido_com_cupom,
        Some(String::from("BEMVINDO"))
    ).await;

    match result {
        Ok(_) => {
            println!("Pedido processado com sucesso!");
            println!("Pedido Final Salvo: Total R$ {:.2}", pedido_com_cupom.total);
        },
        Err(e) => println!("Erro ao processar: {}", e),
    }

    // Pedido único completo
    if let Some(pedido) = pedido_repo.buscar_completo(pedido_pizza.uuid).await.unwrap() {
        println!("Pedido: {}", pedido.uuid);
        for item in &pedido.itens {
            for a in &item.adicionais {
                println!("    + Adicional: {} R${:.2}", a.nome, a.preco);
            }
            for s in &item.partes {
                println!("    ~ Parte {}: {} R${:.2}", s.posicao, s.produto_nome, s.preco_unitario);
            }
        }
    }

    // Pedido único completo
    if let Some(pedido) = pedido_repo.buscar_completo(pedido_com_cupom.uuid).await.unwrap() {
        println!("Pedido: {}", pedido.uuid);
        for item in &pedido.itens {
            for a in &item.adicionais {
                println!("    + Adicional: {} R${:.2}", a.nome, a.preco);
            }
            for s in &item.partes {
                println!("    ~ Parte {}: {} R${:.2}", s.posicao, s.produto_nome, s.preco_unitario);
            }
        }
    }

    // Todos os pedidos de uma loja, já completos
    let pedidos = pedido_repo
        .buscar_completos_por_loja(loja.uuid)
        .await.unwrap();

    println!("Pedidos: {:?}", pedidos);

    // Buscar cupom por código
    if let Some(c) = cupom_repo.buscar_por_codigo("BEMVINDO").await.unwrap() {
        println!("Cupom: {} - {}", c.codigo, c.descricao);
    }

    // // Listar todos os cupons ativos da loja
    let cupons_ativos = cupom_repo.buscar_ativos(loja.uuid).await.unwrap();
    println!("Cupons ativos: {}", cupons_ativos.len());

    for (n, i) in usuario_repo.listar_todos().await.unwrap().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    for (n, i) in loja_repo.listar_todos().await.unwrap().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    for (n, i) in produto_repo.listar_todos().await.unwrap().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    for (n, i) in pedido_repo.listar_todos().await.unwrap().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    for (n, i) in cupom_repo.listar_todos().await.unwrap().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    Ok(())
}
