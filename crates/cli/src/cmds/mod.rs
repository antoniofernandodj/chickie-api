use clap::Subcommand;

use crate::app_state::AppState;
use crate::args::*;

pub mod auth;
pub mod catalog;
pub mod config;
pub mod db;
pub mod enderecos;
pub mod entregadores;
pub mod favoritos;
pub mod funcionarios;
pub mod horarios;
pub mod ingredientes;
pub mod lojas;
pub mod marketing;
pub mod pedidos;
pub mod users;

#[derive(Subcommand)]
pub enum Commands {
    // ── Database ──
    /// Aplicar migrations no banco de dados
    Migrate,
    /// Limpar todas as tabelas do banco
    Wipe,

    // ── Auth ──
    /// Registrar novo usuário
    Signup(SignupArgs),
    /// Login (gera JWT)
    Login(LoginArgs),

    // ── Usuários ──
    /// Listar todos os usuários
    ListUsers,

    // ── Lojas ──
    /// Criar nova loja
    CreateLoja(CreateLojaArgs),
    /// Listar todas as lojas
    ListLojas,
    /// Pesquisar lojas por termo
    SearchLojas(SearchLojasArgs),
    /// Buscar loja por slug
    GetLojaBySlug(GetLojaBySlugArgs),
    /// Buscar loja por UUID
    GetLojaByUuid(GetLojaByUuidArgs),

    // ── Admin Lojas ──
    /// Adicionar funcionário a uma loja
    AddFuncionario(AddFuncionarioArgs),
    /// Adicionar entregador a uma loja
    AddEntregador(AddEntregadorArgs),
    /// Adicionar cliente a uma loja
    AddCliente(AddClienteArgs),
    /// Listar minhas lojas (admin)
    ListMinhasLojas(ListMinhasLojasArgs),

    // ── Funcionários ──
    /// Listar funcionários de uma loja
    ListFuncionarios(ListFuncionariosArgs),
    /// Atualizar funcionário
    UpdateFuncionario(UpdateFuncionarioArgs),

    // ── Entregadores ──
    /// Listar entregadores de uma loja
    ListEntregadores(ListEntregadoresArgs),
    /// Atualizar entregador
    UpdateEntregador(UpdateEntregadorArgs),

    // ── Catálogo: Produtos ──
    /// Criar produto
    CreateProduto(CreateProdutoArgs),
    /// Listar todos os produtos
    ListProdutos,
    /// Listar produtos por loja
    ListProdutosPorLoja(ListProdutosPorLojaArgs),
    /// Buscar produto por UUID
    GetProduto(GetProdutoArgs),
    /// Atualizar produto
    UpdateProduto(UpdateProdutoArgs),
    /// Deletar produto
    DeleteProduto(DeleteProdutoArgs),

    // ── Catálogo: Categorias ──
    /// Criar categoria
    CreateCategoria(CreateCategoriaArgs),
    /// Listar categorias de uma loja
    ListCategorias(ListCategoriasArgs),
    /// Atualizar categoria
    UpdateCategoria(UpdateCategoriaArgs),
    /// Deletar categoria
    DeleteCategoria(DeleteCategoriaArgs),

    // ── Catálogo: Adicionais ──
    /// Criar adicional
    CreateAdicional(CreateAdicionalArgs),
    /// Listar adicionais de uma loja
    ListAdicionais(ListAdicionaisArgs),
    /// Atualizar adicional
    UpdateAdicional(UpdateAdicionalArgs),
    /// Deletar adicional
    DeleteAdicional(DeleteAdicionalArgs),

    // ── Pedidos ──
    /// Criar pedido
    CreatePedido(CreatePedidoArgs),
    /// Listar todos os pedidos
    ListPedidos,
    /// Listar meus pedidos
    ListMeusPedidos(ListMeusPedidosArgs),
    /// Listar pedidos por loja
    ListPedidosPorLoja(ListPedidosPorLojaArgs),
    /// Buscar pedido por UUID
    GetPedido(GetPedidoArgs),
    /// Buscar pedido com entrega
    GetPedidoComEntrega(GetPedidoComEntregaArgs),
    /// Atualizar status do pedido
    UpdatePedidoStatus(UpdatePedidoStatusArgs),
    /// Atribuir entregador ao pedido
    AtribuirEntregador(AtribuirEntregadorArgs),
    /// Remover entregador do pedido
    RemoverEntregador(RemoverEntregadorArgs),

    // ── Endereço de Entrega ──
    /// Criar endereço para pedido
    CreateEnderecoEntrega(CreateEnderecoEntregaArgs),
    /// Buscar endereço por pedido
    GetEnderecoEntrega(GetEnderecoEntregaArgs),
    /// Listar endereços por loja
    ListEnderecosEntrega(ListEnderecosEntregaArgs),

    // ── Endereço de Loja ──
    /// Listar endereços de uma loja
    ListEnderecosLoja(ListEnderecosLojaArgs),
    /// Criar endereço de loja
    CreateEnderecoLoja(CreateEnderecoLojaArgs),
    /// Atualizar endereço de loja
    UpdateEnderecoLoja(UpdateEnderecoLojaArgs),
    /// Deletar endereço de loja
    DeleteEnderecoLoja(DeleteEnderecoLojaArgs),

    // ── Endereço de Usuário ──
    /// Criar endereço de usuário
    CreateEnderecoUsuario(CreateEnderecoUsuarioArgs),
    /// Listar endereços do usuário
    ListEnderecosUsuario(ListEnderecosUsuarioArgs),
    /// Buscar endereço de usuário
    GetEnderecoUsuario(GetEnderecoUsuarioArgs),
    /// Atualizar endereço de usuário
    UpdateEnderecoUsuario(UpdateEnderecoUsuarioArgs),
    /// Deletar endereço de usuário
    DeleteEnderecoUsuario(DeleteEnderecoUsuarioArgs),

    // ── Marketing: Cupons ──
    /// Criar cupom
    CreateCupom(CreateCupomArgs),
    /// Listar cupons de uma loja
    ListCupons,
    /// Validar cupom por código
    ValidarCupom(ValidarCupomArgs),
    /// Atualizar cupom
    UpdateCupom(UpdateCupomArgs),
    /// Deletar cupom
    DeleteCupom(DeleteCupomArgs),

    // ── Marketing: Promoções ──
    /// Criar promoção
    CreatePromocao(CreatePromocaoArgs),
    /// Listar promoções de uma loja
    ListPromocoes(ListPromocoesArgs),
    /// Atualizar promoção
    UpdatePromocao(UpdatePromocaoArgs),
    /// Deletar promoção
    DeletePromocao(DeletePromocaoArgs),

    // ── Marketing: Avaliações ──
    /// Avaliar loja
    AvaliarLoja(AvaliarLojaArgs),
    /// Avaliar produto
    AvaliarProduto(AvaliarProdutoArgs),

    // ── Favoritos ──
    /// Adicionar loja aos favoritos
    AddFavorito(AddFavoritoArgs),
    /// Remover loja dos favoritos
    RemoveFavorito(RemoveFavoritoArgs),
    /// Listar minhas favoritas
    ListFavoritas {
        #[arg(long)]
        usuario_uuid: uuid::Uuid,
    },
    /// Verificar se loja é favorita
    VerificarFavorito(VerificarFavoritoArgs),

    // ── Horários de Funcionamento ──
    /// Listar horários de uma loja
    ListHorarios(ListHorariosArgs),
    /// Criar ou atualizar horário
    CreateHorario(CreateHorarioArgs),
    /// Ativar/desativar dia
    ToggleHorarioAtivo(ToggleHorarioAtivoArgs),
    /// Deletar horário de um dia
    DeleteHorario(DeleteHorarioArgs),

    // ── Configuração de Pedidos ──
    /// Buscar configuração de pedidos de uma loja
    GetConfigPedido(GetConfigPedidoArgs),
    /// Salvar configuração de pedidos
    SaveConfigPedido(SaveConfigPedidoArgs),

    // ── Ingredientes ──
    /// Criar ingrediente
    CreateIngrediente(CreateIngredienteArgs),
    /// Listar ingredientes de uma loja
    ListIngredientes(ListIngredientesArgs),
    /// Atualizar ingrediente
    UpdateIngrediente(UpdateIngredienteArgs),
    /// Deletar ingrediente
    DeleteIngrediente(DeleteIngredienteArgs),
}

pub async fn dispatch(command: Commands, state: &AppState) {
    match command {
        // ── Database ──
        Commands::Migrate => db::run_migrate(state).await,
        Commands::Wipe => db::run_wipe(state).await,

        // ── Auth ──
        Commands::Signup(args) => auth::run_signup(state, args).await,
        Commands::Login(args) => auth::run_login(state, args).await,

        // ── Usuários ──
        Commands::ListUsers => users::run_list_users(state).await,

        // ── Lojas ──
        Commands::CreateLoja(args) => lojas::run_create_loja(state, args).await,
        Commands::ListLojas => lojas::run_list_lojas(state).await,
        Commands::SearchLojas(args) => lojas::run_search_lojas(state, args).await,
        Commands::GetLojaBySlug(args) => lojas::run_get_loja_by_slug(state, args).await,
        Commands::GetLojaByUuid(args) => lojas::run_get_loja_by_uuid(state, args).await,

        // ── Admin Lojas ──
        Commands::AddFuncionario(args) => lojas::run_add_funcionario(state, args).await,
        Commands::AddEntregador(args) => lojas::run_add_entregador(state, args).await,
        Commands::AddCliente(args) => lojas::run_add_cliente(state, args).await,
        Commands::ListMinhasLojas(args) => lojas::run_list_minhas_lojas(state, args).await,

        // ── Funcionários ──
        Commands::ListFuncionarios(args) => funcionarios::run_list_funcionarios(state, args).await,
        Commands::UpdateFuncionario(_) => funcionarios::run_update_funcionario().await,

        // ── Entregadores ──
        Commands::ListEntregadores(args) => entregadores::run_list_entregadores(state, args).await,
        Commands::UpdateEntregador(_) => entregadores::run_update_entregador().await,

        // ── Catálogo: Produtos ──
        Commands::CreateProduto(args) => catalog::run_create_produto(state, args).await,
        Commands::ListProdutos => catalog::run_list_produtos().await,
        Commands::ListProdutosPorLoja(args) => catalog::run_list_produtos_por_loja(state, args).await,
        Commands::GetProduto(args) => catalog::run_get_produto(state, args).await,
        Commands::UpdateProduto(args) => catalog::run_update_produto(state, args).await,
        Commands::DeleteProduto(args) => catalog::run_delete_produto(state, args).await,

        // ── Catálogo: Categorias ──
        Commands::CreateCategoria(args) => catalog::run_create_categoria(state, args).await,
        Commands::ListCategorias(args) => catalog::run_list_categorias(state, args).await,
        Commands::UpdateCategoria(args) => catalog::run_update_categoria(state, args).await,
        Commands::DeleteCategoria(args) => catalog::run_delete_categoria(state, args).await,

        // ── Catálogo: Adicionais ──
        Commands::CreateAdicional(args) => catalog::run_create_adicional(state, args).await,
        Commands::ListAdicionais(args) => catalog::run_list_adicionais(state, args).await,
        Commands::UpdateAdicional(args) => catalog::run_update_adicional(state, args).await,
        Commands::DeleteAdicional(args) => catalog::run_delete_adicional(state, args).await,

        // ── Pedidos ──
        Commands::CreatePedido(args) => pedidos::run_create_pedido(state, args).await,
        Commands::ListPedidos => pedidos::run_list_pedidos().await,
        Commands::ListMeusPedidos(args) => pedidos::run_list_meus_pedidos(state, args).await,
        Commands::ListPedidosPorLoja(args) => pedidos::run_list_pedidos_por_loja(state, args).await,
        Commands::GetPedido(_) => pedidos::run_get_pedido().await,
        Commands::GetPedidoComEntrega(_) => pedidos::run_get_pedido_com_entrega().await,
        Commands::UpdatePedidoStatus(args) => pedidos::run_update_pedido_status(state, args).await,
        Commands::AtribuirEntregador(args) => pedidos::run_atribuir_entregador(state, args).await,
        Commands::RemoverEntregador(_) => pedidos::run_remover_entregador().await,

        // ── Endereço de Entrega ──
        Commands::CreateEnderecoEntrega(args) => enderecos::run_create_endereco_entrega(state, args).await,
        Commands::GetEnderecoEntrega(args) => enderecos::run_get_endereco_entrega(state, args).await,
        Commands::ListEnderecosEntrega(args) => enderecos::run_list_enderecos_entrega(state, args).await,

        // ── Endereço de Loja ──
        Commands::ListEnderecosLoja(args) => enderecos::run_list_enderecos_loja(state, args).await,
        Commands::CreateEnderecoLoja(args) => enderecos::run_create_endereco_loja(state, args).await,
        Commands::UpdateEnderecoLoja(_) => enderecos::run_update_endereco_loja().await,
        Commands::DeleteEnderecoLoja(_) => enderecos::run_delete_endereco_loja().await,

        // ── Endereço de Usuário ──
        Commands::CreateEnderecoUsuario(_) => enderecos::run_create_endereco_usuario().await,
        Commands::ListEnderecosUsuario(args) => enderecos::run_list_enderecos_usuario(state, args).await,
        Commands::GetEnderecoUsuario(_) => enderecos::run_get_endereco_usuario().await,
        Commands::UpdateEnderecoUsuario(_) => enderecos::run_update_endereco_usuario().await,
        Commands::DeleteEnderecoUsuario(_) => enderecos::run_delete_endereco_usuario().await,

        // ── Marketing: Cupons ──
        Commands::CreateCupom(args) => marketing::run_create_cupom(state, args).await,
        Commands::ListCupons => marketing::run_list_cupons().await,
        Commands::ValidarCupom(_) => marketing::run_validar_cupom().await,
        Commands::UpdateCupom(_) => marketing::run_update_cupom().await,
        Commands::DeleteCupom(args) => marketing::run_delete_cupom(state, args).await,

        // ── Marketing: Promoções ──
        Commands::CreatePromocao(args) => marketing::run_create_promocao(state, args).await,
        Commands::ListPromocoes(args) => marketing::run_list_promocoes(state, args).await,
        Commands::UpdatePromocao(_) => marketing::run_update_promocao().await,
        Commands::DeletePromocao(args) => marketing::run_delete_promocao(state, args).await,

        // ── Marketing: Avaliações ──
        Commands::AvaliarLoja(args) => marketing::run_avaliar_loja(state, args).await,
        Commands::AvaliarProduto(args) => marketing::run_avaliar_produto(state, args).await,

        // ── Favoritos ──
        Commands::AddFavorito(args) => favoritos::run_add_favorito(state, args).await,
        Commands::RemoveFavorito(args) => favoritos::run_remove_favorito(state, args).await,
        Commands::ListFavoritas { usuario_uuid } => favoritos::run_list_favoritas(state, usuario_uuid).await,
        Commands::VerificarFavorito(args) => favoritos::run_verificar_favorito(state, args).await,

        // ── Horários ──
        Commands::ListHorarios(args) => horarios::run_list_horarios(state, args).await,
        Commands::CreateHorario(args) => horarios::run_create_horario(state, args).await,
        Commands::ToggleHorarioAtivo(args) => horarios::run_toggle_horario_ativo(state, args).await,
        Commands::DeleteHorario(args) => horarios::run_delete_horario(state, args).await,

        // ── Config Pedido ──
        Commands::GetConfigPedido(args) => config::run_get_config_pedido(state, args).await,
        Commands::SaveConfigPedido(args) => config::run_save_config_pedido(state, args).await,

        // ── Ingredientes ──
        Commands::CreateIngrediente(args) => ingredientes::run_create_ingrediente(state, args).await,
        Commands::ListIngredientes(args) => ingredientes::run_list_ingredientes(state, args).await,
        Commands::UpdateIngrediente(_) => ingredientes::run_update_ingrediente().await,
        Commands::DeleteIngrediente(args) => ingredientes::run_delete_ingrediente(state, args).await,
    }
}
