use std::sync::Arc;

use chickie_core::{
    repositories::*,
    services::*,
};

// ─────────────────────────────────────────────────────────────────────────────
// AppState (espelhado do API para injeção de dependências)
// ─────────────────────────────────────────────────────────────────────────────

pub struct AppState {
    pub pool: Arc<sqlx::PgPool>,
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
    #[allow(dead_code)]
    pub pedido_repo: Arc<PedidoRepository>,
    pub cupom_repo: Arc<CupomRepository>,
    pub promocao_repo: Arc<PromocaoRepository>,
}

impl AppState {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        let usuario_repo = Arc::new(UsuarioRepository::new(pool.clone()));
        let loja_repo = Arc::new(LojaRepository::new(pool.clone()));
        let produto_repo = Arc::new(ProdutoRepository::new(pool.clone()));
        let pedido_repo = Arc::new(PedidoRepository::new(pool.clone()));
        let cupom_repo = Arc::new(CupomRepository::new(pool.clone()));
        let adicional_repo = Arc::new(AdicionalRepository::new(pool.clone()));
        let avaliacoes_de_produto_repo = Arc::new(AvaliacaoDeProdutoRepository::new(pool.clone()));
        let avaliacoes_de_loja_repo = Arc::new(AvaliacaoDeLojaRepository::new(pool.clone()));
        let funcionario_repo = Arc::new(FuncionarioRepository::new(pool.clone()));
        let categorias_de_produtos_repo = Arc::new(CategoriaProdutosRepository::new(pool.clone()));
        let categoria_ordem_repo = Arc::new(CategoriaOrdemRepository::new(pool.clone()));
        let entregador_repo = Arc::new(EntregadorRepository::new(pool.clone()));
        let promocao_repo = Arc::new(PromocaoRepository::new(pool.clone()));
        let horario_repo = Arc::new(HorarioFuncionamentoRepository::new(pool.clone()));
        let config_partes_repo = Arc::new(ConfiguracaoPedidosLojaRepository::new(pool.clone()));
        let cliente_repo = Arc::new(ClienteRepository::new(pool.clone()));
        let endereco_entrega_repo = Arc::new(EnderecoEntregaRepository::new(pool.clone()));
        let endereco_usuario_repo = Arc::new(EnderecoUsuarioRepository::new(pool.clone()));
        let endereco_loja_repo = Arc::new(EnderecoLojaRepository::new(pool.clone()));
        let favorito_repo = Arc::new(LojaFavoritaRepository::new(pool.clone()));
        let ingrediente_repo = Arc::new(IngredienteRepository::new(pool.clone()));

        Self {
            pool,
            usuario_service: Arc::new(UsuarioService::new(usuario_repo.clone())),
            loja_service: Arc::new(LojaService::new(
                loja_repo,
                config_partes_repo.clone(),
                horario_repo.clone(),
                funcionario_repo.clone(),
                entregador_repo.clone(),
                cliente_repo,
                usuario_repo.clone(),
            )),
            catalogo_service: Arc::new(CatalogoService::new(
                produto_repo,
                categorias_de_produtos_repo,
                adicional_repo,
                categoria_ordem_repo,
            )),
            pedido_service: Arc::new(PedidoService::new(
                pedido_repo.clone(),
                config_partes_repo.clone(),
                cupom_repo.clone(),
                promocao_repo.clone(),
                endereco_entrega_repo.clone(),
            )),
            marketing_service: Arc::new(MarketingService::new(
                cupom_repo.clone(),
                promocao_repo.clone(),
                avaliacoes_de_loja_repo,
                avaliacoes_de_produto_repo,
            )),
            endereco_entrega_service: Arc::new(EnderecoEntregaService::new(endereco_entrega_repo.clone())),
            endereco_usuario_service: Arc::new(EnderecoUsuarioService::new(endereco_usuario_repo)),
            endereco_loja_service: Arc::new(EnderecoLojaService::new(endereco_loja_repo)),
            loja_favorita_service: Arc::new(LojaFavoritaService::new(favorito_repo)),
            ingrediente_service: Arc::new(IngredienteService::new(ingrediente_repo)),
            horario_funcionamento_service: Arc::new(HorarioFuncionamentoService::new(horario_repo)),
            config_pedido_service: Arc::new(ConfiguracaoPedidosLojaService::new(config_partes_repo.clone())),
            funcionario_service: Arc::new(FuncionarioService::new(funcionario_repo, usuario_repo.clone())),
            entregador_service: Arc::new(EntregadorService::new(entregador_repo, usuario_repo)),
            pedido_repo,
            cupom_repo,
            promocao_repo,
        }
    }
}
