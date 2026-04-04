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
        LojaFavoritaRepository,
        LojaRepository,
        PedidoRepository,
        ProdutoRepository,
        PromocaoRepository,
        UsuarioRepository
    },
    services::{
        CatalogoService,
        EnderecoEntregaService,
        EnderecoUsuarioService,
        LojaFavoritaService,
        LojaService,
        MarketingService,
        PedidoService,
        UsuarioService
    }
};

pub struct AppState {
    pub usuario_service: UsuarioService,
    pub loja_service: LojaService,
    pub catalogo_service: CatalogoService,
    pub pedido_service: PedidoService,
    pub marketing_service: MarketingService,
    pub endereco_entrega_service: EnderecoEntregaService,
    pub endereco_usuario_service: EnderecoUsuarioService,
    pub loja_favorita_service: LojaFavoritaService,
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
        let usuario_service = UsuarioService::new(
            Arc::clone(&usuario_repo)
        );

        let loja_service = LojaService::new(
            Arc::clone(&loja_repo),
            Arc::clone(&config_partes_repo),
            Arc::clone(&horario_repo),
            Arc::clone(&funcionario_repo),
            Arc::clone(&entregador_repo),
            Arc::clone(&cliente_repo),
            Arc::clone(&usuario_repo)
        );

        let catalogo_service = CatalogoService::new(
            Arc::clone(&produto_repo),
            Arc::clone(&categorias_de_produtos_repo),
            Arc::clone(&adicional_repo)
        );

        let pedido_service = PedidoService::new(
            Arc::clone(&pedido_repo),
            Arc::clone(&config_partes_repo),
            Arc::clone(&cupom_repo),
            Arc::clone(&promocao_repo),
            Arc::clone(&endereco_entrega_repo),
        );

        let marketing_service = MarketingService::new(
            Arc::clone(&cupom_repo),
            Arc::clone(&promocao_repo),
            Arc::clone(&avaliacoes_de_loja_repo),
            Arc::clone(&avaliacoes_de_produto_repo)
        );

        let endereco_entrega_service = EnderecoEntregaService::new(
            Arc::clone(&endereco_entrega_repo)
        );

        let endereco_usuario_service = EnderecoUsuarioService::new(
            Arc::clone(&endereco_usuario_repo)
        );

        let loja_favorita_service = LojaFavoritaService::new(
            Arc::clone(&loja_favorita_repo)
        );


        // 4. Estado compartilhado
        let s = Arc::new(AppState {
            usuario_service,
            loja_service,
            catalogo_service,
            pedido_service,
            marketing_service,
            endereco_entrega_service,
            endereco_usuario_service,
            loja_favorita_service,
            pedido_repo: Arc::clone(&pedido_repo),
            cupom_repo: Arc::clone(&cupom_repo),
            usuario_repo: Arc::clone(&usuario_repo),
            loja_repo: Arc::clone(&loja_repo),
            produto_repo: Arc::clone(&produto_repo),
            db: pool,
        });

        s
    }
}