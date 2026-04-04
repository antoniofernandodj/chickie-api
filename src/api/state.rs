use std::sync::Arc;


use sqlx::PgPool;


use crate::{
    repositories::{
        AdicionalRepository,
        AvaliacaoDeLojaRepository,
        AvaliacaoDeProdutoRepository,
        CategoriaProdutosRepository,
        ClienteRepository,
        ConfiguracaoPedidosLojaRepository,
        CupomRepository,
        EnderecoEntregaRepository,
        EnderecoUsuarioRepository,
        EntregadorRepository,
        FuncionarioRepository,
        HorarioFuncionamentoRepository,
        IngredienteRepository,
        LojaFavoritaRepository,
        LojaRepository,
        PedidoRepository,
        ProdutoRepository,
        PromocaoRepository,
        UsuarioRepository
    },
    services::{
        CatalogoService,
        ConfiguracaoPedidosLojaService,
        EnderecoEntregaService,
        EnderecoUsuarioService,
        EntregadorService,
        FuncionarioService,
        HorarioFuncionamentoService,
        IngredienteService,
        LojaFavoritaService,
        LojaService,
        MarketingService,
        PedidoService,
        UsuarioService
    }
};

pub struct AppState {

    pub usuario_service: Arc<UsuarioService>,
    pub loja_service: Arc<LojaService>,
    pub catalogo_service: Arc<CatalogoService>,
    pub pedido_service: Arc<PedidoService>,
    pub marketing_service: Arc<MarketingService>,
    pub endereco_entrega_service: Arc<EnderecoEntregaService>,
    pub endereco_usuario_service: Arc<EnderecoUsuarioService>,
    pub loja_favorita_service: Arc<LojaFavoritaService>,
    pub ingrediente_service: Arc<IngredienteService>,
    pub horario_funcionamento_service: Arc<HorarioFuncionamentoService>,
    pub config_pedido_service: Arc<ConfiguracaoPedidosLojaService>,
    pub funcionario_service: Arc<FuncionarioService>,
    pub entregador_service: Arc<EntregadorService>,

    // Repositórios brutos para buscas simples nos handlers
    pub pedido_repo: Arc<PedidoRepository>,
    pub cupom_repo: Arc<CupomRepository>,
    pub usuario_repo: Arc<UsuarioRepository>,
    pub loja_repo: Arc<LojaRepository>,
    pub produto_repo: Arc<ProdutoRepository>,
    // Raw pool for administrative operations (e.g. wipe database)
    pub db: Arc<PgPool>,
}


impl AppState {
    pub fn new(pool: Arc<PgPool>) -> Arc<Self> {
        // 2. Inicialização dos Repositórios
        // Nota: Usamos 'static life hack aqui para simplificar, assumindo que o pool vive toda a execução.
        // Em produção, o pool geralmente é envolvido em Arc.
        let usuario_repo =
            Arc::new(UsuarioRepository::new(pool.clone()));
        let loja_repo =
            Arc::new(LojaRepository::new(pool.clone()));
        let produto_repo =
            Arc::new(ProdutoRepository::new(pool.clone()));
        let pedido_repo =
            Arc::new(PedidoRepository::new(pool.clone()));
        let cupom_repo =
            Arc::new(CupomRepository::new(pool.clone()));
        let adicional_repo =
            Arc::new(AdicionalRepository::new(pool.clone()));
        let avaliacoes_de_produto_repo =
            Arc::new(AvaliacaoDeProdutoRepository::new(pool.clone()));
        let avaliacoes_de_loja_repo =
            Arc::new(AvaliacaoDeLojaRepository::new(pool.clone()));
        let funcionario_repo =
            Arc::new(FuncionarioRepository::new(pool.clone()));
        let categorias_de_produtos_repo =
            Arc::new(CategoriaProdutosRepository::new(pool.clone()));
        let entregador_repo =
            Arc::new(EntregadorRepository::new(pool.clone()));
        let promocao_repo =
            Arc::new(PromocaoRepository::new(pool.clone()));
        let horario_repo =
            Arc::new(HorarioFuncionamentoRepository::new(pool.clone()));
        let config_partes_repo =
            Arc::new(ConfiguracaoPedidosLojaRepository::new(pool.clone()));
        let cliente_repo =
            Arc::new(ClienteRepository::new(pool.clone()));
        let endereco_entrega_repo =
            Arc::new(EnderecoEntregaRepository::new(pool.clone()));
        let endereco_usuario_repo =
            Arc::new(EnderecoUsuarioRepository::new(pool.clone()));
        let loja_favorita_repo =
            Arc::new(LojaFavoritaRepository::new(pool.clone()));

        // 3. Inicialização dos Services
        let usuario_service = Arc::new(
            UsuarioService::new(
                Arc::clone(&usuario_repo)
            )
        );

        let loja_service = Arc::new(
            LojaService::new(
                Arc::clone(&loja_repo),
                Arc::clone(&config_partes_repo),
                Arc::clone(&horario_repo),
                Arc::clone(&funcionario_repo),
                Arc::clone(&entregador_repo),
                Arc::clone(&cliente_repo),
                Arc::clone(&usuario_repo)
            )
        );

        let catalogo_service = Arc::new(
            CatalogoService::new(
                Arc::clone(&produto_repo),
                Arc::clone(&categorias_de_produtos_repo),
                Arc::clone(&adicional_repo)
            )
        );

        let pedido_service = Arc::new(
            PedidoService::new(
                Arc::clone(&pedido_repo),
                Arc::clone(&config_partes_repo),
                Arc::clone(&cupom_repo),
                Arc::clone(&promocao_repo),
                Arc::clone(&endereco_entrega_repo),
            )
        );

        let marketing_service = Arc::new(
            MarketingService::new(
                Arc::clone(&cupom_repo),
                Arc::clone(&promocao_repo),
                Arc::clone(&avaliacoes_de_loja_repo),
                Arc::clone(&avaliacoes_de_produto_repo)
            )
        );

        let endereco_entrega_service = Arc::new(
            EnderecoEntregaService::new(
                Arc::clone(&endereco_entrega_repo)
            )
        );

        let endereco_usuario_service = Arc::new(
            EnderecoUsuarioService::new(
                Arc::clone(&endereco_usuario_repo)
            )
        );

        let loja_favorita_service = Arc::new(
            LojaFavoritaService::new(
                Arc::clone(&loja_favorita_repo)
            )
        );

        let ingrediente_service = Arc::new(
            IngredienteService::new(
                Arc::new(IngredienteRepository::new(pool.clone()))
            )
        );

        let horario_funcionamento_service = Arc::new(
            HorarioFuncionamentoService::new(
                Arc::new(HorarioFuncionamentoRepository::new(pool.clone()))
            )
        );

        let config_pedido_service = Arc::new(
            ConfiguracaoPedidosLojaService::new(
                Arc::clone(&config_partes_repo)
            )
        );

        let funcionario_service = Arc::new(
            FuncionarioService::new(
                Arc::new(FuncionarioRepository::new(pool.clone())),
                Arc::clone(&usuario_repo)
            )
        );

        let entregador_service = Arc::new(
            EntregadorService::new(
                Arc::new(EntregadorRepository::new(pool.clone())),
                Arc::clone(&usuario_repo)
            )
        );


        // 4. Estado compartilhado
        let s = Arc::new(
            AppState {
                usuario_service: Arc::clone(&usuario_service),
                loja_service: Arc::clone(&loja_service),
                catalogo_service: Arc::clone(&catalogo_service),
                pedido_service: Arc::clone(&pedido_service),
                marketing_service: Arc::clone(&marketing_service),
                endereco_entrega_service: Arc::clone(&endereco_entrega_service),
                endereco_usuario_service: Arc::clone(&endereco_usuario_service),
                loja_favorita_service: Arc::clone(&loja_favorita_service),
                ingrediente_service,
                horario_funcionamento_service,
                config_pedido_service,
                funcionario_service,
                entregador_service,

                pedido_repo: Arc::clone(&pedido_repo),
                cupom_repo: Arc::clone(&cupom_repo),
                usuario_repo: Arc::clone(&usuario_repo),
                loja_repo: Arc::clone(&loja_repo),
                produto_repo: Arc::clone(&produto_repo),
                db: pool,
            }
        );

        s
    }
}