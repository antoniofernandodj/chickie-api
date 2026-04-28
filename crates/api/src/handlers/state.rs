use std::sync::Arc;


use sqlx::PgPool;


use chickie_core::{
    ports::{
        UsuarioRepositoryPort,
        LojaRepositoryPort,
        ProdutoRepositoryPort,
        PedidoRepositoryPort,
        CupomRepositoryPort,
        AdicionalRepositoryPort,
        CategoriaRepositoryPort,
        OrdemCategoriaRepositoryPort,
        PromocaoRepositoryPort,
        AvaliacaoDeLojaRepositoryPort,
        AvaliacaoDeProdutoRepositoryPort,
        EnderecoEntregaRepositoryPort,
        EnderecoUsuarioRepositoryPort,
        EnderecoLojaRepositoryPort,
        LojaFavoritaRepositoryPort,
        IngredienteRepositoryPort,
        HorarioFuncionamentoRepositoryPort,
        ConfiguracaoPedidosLojaRepositoryPort,
        FuncionarioRepositoryPort,
        EntregadorRepositoryPort,
        ClienteRepositoryPort,
    },
    repositories::{
        AdicionalRepository,
        AvaliacaoDeLojaRepository,
        AvaliacaoDeProdutoRepository,
        CategoriaProdutosRepository,
        CategoriaOrdemRepository,
        ClienteRepository,
        ConfiguracaoPedidosLojaRepository,
        CupomRepository,
        EnderecoEntregaRepository,
        EnderecoLojaRepository,
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
        AsaasService,
        CatalogoService,
        ConfiguracaoPedidosLojaService,
        EnderecoEntregaService,
        EnderecoLojaService,
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
    },
    usecases::UploadImagemUsecase
};

pub struct AppState {

    pub asaas_service: Arc<AsaasService>,
    pub usuario_service: Arc<UsuarioService>,
    pub loja_service: Arc<LojaService>,
    pub catalogo_service: Arc<CatalogoService>,
    pub pedido_service: Arc<PedidoService>,
    pub marketing_service: Arc<MarketingService>,
    pub endereco_entrega_service: Arc<EnderecoEntregaService>,
    pub endereco_usuario_service: Arc<EnderecoUsuarioService>,
    pub endereco_loja_service: Arc<EnderecoLojaService>,
    pub loja_favorita_service: Arc<LojaFavoritaService>,
    pub ingrediente_service: Arc<IngredienteService>,
    pub horario_funcionamento_service: Arc<HorarioFuncionamentoService>,
    pub config_pedido_service: Arc<ConfiguracaoPedidosLojaService>,
    pub funcionario_service: Arc<FuncionarioService>,
    pub entregador_service: Arc<EntregadorService>,
    pub upload_imagem_usecase: Arc<UploadImagemUsecase>,

    // Repositórios brutos para buscas simples nos handlers
    pub pedido_repo: Arc<PedidoRepository>,
    pub cupom_repo: Arc<CupomRepository>,
    pub usuario_repo: Arc<UsuarioRepository>,
    pub loja_repo: Arc<LojaRepository>,
    pub produto_repo: Arc<dyn ProdutoRepositoryPort>,
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
        let categoria_ordem_repo =
            Arc::new(CategoriaOrdemRepository::new(pool.clone()));
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
        let endereco_loja_repo =
            Arc::new(EnderecoLojaRepository::new(pool.clone()));
        let loja_favorita_repo =
            Arc::new(LojaFavoritaRepository::new(pool.clone()));

        // 3. Inicialização dos Services
        let usuario_service = Arc::new(
            UsuarioService::new(
                Arc::clone(&usuario_repo) as Arc<dyn UsuarioRepositoryPort>
            )
        );

        let loja_service = Arc::new(
            LojaService::new(
                Arc::clone(&loja_repo) as Arc<dyn LojaRepositoryPort>,
                Arc::clone(&config_partes_repo) as Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>,
                Arc::clone(&horario_repo) as Arc<dyn HorarioFuncionamentoRepositoryPort>,
                Arc::clone(&funcionario_repo) as Arc<dyn FuncionarioRepositoryPort>,
                Arc::clone(&entregador_repo) as Arc<dyn EntregadorRepositoryPort>,
                Arc::clone(&cliente_repo) as Arc<dyn ClienteRepositoryPort>,
                Arc::clone(&usuario_repo) as Arc<dyn UsuarioRepositoryPort>
            )
        );

        let catalogo_service = Arc::new(
            CatalogoService::new(
                Arc::clone(&produto_repo) as Arc<dyn ProdutoRepositoryPort>,
                Arc::clone(&categorias_de_produtos_repo) as Arc<dyn CategoriaRepositoryPort>,
                Arc::clone(&adicional_repo) as Arc<dyn AdicionalRepositoryPort>,
                Arc::clone(&categoria_ordem_repo) as Arc<dyn OrdemCategoriaRepositoryPort>,
            )
        );

        let pedido_service = Arc::new(
            PedidoService::new(
                Arc::clone(&pedido_repo) as Arc<dyn PedidoRepositoryPort>,
                Arc::clone(&config_partes_repo) as Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>,
                Arc::clone(&cupom_repo) as Arc<dyn CupomRepositoryPort>,
                Arc::clone(&promocao_repo) as Arc<dyn PromocaoRepositoryPort>,
                Arc::clone(&endereco_entrega_repo) as Arc<dyn EnderecoEntregaRepositoryPort>,
            )
        );

        let marketing_service = Arc::new(
            MarketingService::new(
                Arc::clone(&cupom_repo) as Arc<dyn CupomRepositoryPort>,
                Arc::clone(&promocao_repo) as Arc<dyn PromocaoRepositoryPort>,
                Arc::clone(&avaliacoes_de_loja_repo) as Arc<dyn AvaliacaoDeLojaRepositoryPort>,
                Arc::clone(&avaliacoes_de_produto_repo) as Arc<dyn AvaliacaoDeProdutoRepositoryPort>
            )
        );

        let endereco_entrega_service = Arc::new(
            EnderecoEntregaService::new(
                Arc::clone(&endereco_entrega_repo) as Arc<dyn EnderecoEntregaRepositoryPort>
            )
        );

        let endereco_usuario_service = Arc::new(
            EnderecoUsuarioService::new(
                Arc::clone(&endereco_usuario_repo) as Arc<dyn EnderecoUsuarioRepositoryPort>
            )
        );

        let endereco_loja_service = Arc::new(
            EnderecoLojaService::new(
                Arc::clone(&endereco_loja_repo) as Arc<dyn EnderecoLojaRepositoryPort>
            )
        );

        let loja_favorita_service = Arc::new(
            LojaFavoritaService::new(
                Arc::clone(&loja_favorita_repo) as Arc<dyn LojaFavoritaRepositoryPort>
            )
        );

        let ingrediente_repo = Arc::new(IngredienteRepository::new(pool.clone()));
        let ingrediente_service = Arc::new(
            IngredienteService::new(
                Arc::clone(&ingrediente_repo) as Arc<dyn IngredienteRepositoryPort>
            )
        );

        let horario_repo = Arc::new(HorarioFuncionamentoRepository::new(pool.clone()));
        let horario_funcionamento_service = Arc::new(
            HorarioFuncionamentoService::new(
                Arc::clone(&horario_repo) as Arc<dyn HorarioFuncionamentoRepositoryPort>
            )
        );

        let config_pedido_service = Arc::new(
            ConfiguracaoPedidosLojaService::new(
                Arc::clone(&config_partes_repo) as Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>
            )
        );

        let funcionario_repo_svc = Arc::new(FuncionarioRepository::new(pool.clone()));
        let funcionario_service = Arc::new(
            FuncionarioService::new(
                Arc::clone(&funcionario_repo_svc) as Arc<dyn FuncionarioRepositoryPort>,
                Arc::clone(&usuario_repo) as Arc<dyn UsuarioRepositoryPort>
            )
        );

        let entregador_repo_svc = Arc::new(EntregadorRepository::new(pool.clone()));
        let entregador_service = Arc::new(
            EntregadorService::new(
                Arc::clone(&entregador_repo_svc) as Arc<dyn EntregadorRepositoryPort>,
                Arc::clone(&usuario_repo) as Arc<dyn UsuarioRepositoryPort>
            )
        );

        // S3 configuration from environment (client created lazily in usecase)
        let bucket = std::env::var("S3_BUCKET_NAME_1")
            .expect("Var env não encontrada: S3_BUCKET_NAME_1");
        let endpoint = std::env::var("S3_ENDPOINT")
            .expect("Var env não encontrada: S3_ENDPOINT");

        let upload_imagem_usecase = Arc::new(
            UploadImagemUsecase::new(
                Arc::clone(&produto_repo) as Arc<dyn ProdutoRepositoryPort>,
                bucket,
                endpoint.clone(),
            )
        );


        let asaas_service = Arc::new(AsaasService::new());

        // 4. Estado compartilhado
        let s = Arc::new(
            AppState {
                asaas_service,
                usuario_service: Arc::clone(&usuario_service),
                loja_service: Arc::clone(&loja_service),
                catalogo_service: Arc::clone(&catalogo_service),
                pedido_service: Arc::clone(&pedido_service),
                marketing_service: Arc::clone(&marketing_service),
                endereco_entrega_service: Arc::clone(&endereco_entrega_service),
                endereco_usuario_service: Arc::clone(&endereco_usuario_service),
                endereco_loja_service: Arc::clone(&endereco_loja_service),
                loja_favorita_service: Arc::clone(&loja_favorita_service),
                ingrediente_service,
                horario_funcionamento_service,
                config_pedido_service,
                funcionario_service,
                entregador_service,
                upload_imagem_usecase,

                pedido_repo: Arc::clone(&pedido_repo),
                cupom_repo: Arc::clone(&cupom_repo),
                usuario_repo: Arc::clone(&usuario_repo),
                loja_repo: Arc::clone(&loja_repo),
                produto_repo: Arc::clone(&produto_repo) as Arc<dyn ProdutoRepositoryPort>,
                db: pool,
            }
        );

        s
    }
}