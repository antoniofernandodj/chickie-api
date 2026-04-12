use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// Auth
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct SignupArgs {
    #[arg(short, long)]
    pub nome: String,
    #[arg(short, long)]
    pub username: String,
    #[arg(short, long)]
    pub senha: String,
    #[arg(short, long)]
    pub email: String,
    #[arg(short, long)]
    pub celular: String,
    #[arg(long, default_value = "email")]
    pub auth_method: String,
    #[arg(long, default_value = "cliente")]
    pub classe: String,
}

#[derive(clap::Args)]
pub struct LoginArgs {
    #[arg(short, long)]
    pub identifier: String,
    #[arg(short, long)]
    pub senha: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Lojas
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreateLojaArgs {
    #[arg(short, long)]
    pub nome: String,
    #[arg(short, long)]
    pub slug: String,
    #[arg(long)]
    pub email_contato: String,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub celular: Option<String>,
    #[arg(long)]
    pub hora_abertura: Option<String>,
    #[arg(long)]
    pub hora_fechamento: Option<String>,
    #[arg(long)]
    pub dias_funcionamento: Option<Vec<i32>>,
    #[arg(long)]
    pub tempo_medio: Option<i32>,
    #[arg(long)]
    pub nota_media: Option<f64>,
    #[arg(long)]
    pub taxa_entrega_base: Option<f64>,
    #[arg(long)]
    pub pedido_minimo: Option<f64>,
    #[arg(long)]
    pub max_partes: i32,
    #[arg(long)]
    pub criado_por: Uuid,
}

#[derive(clap::Args)]
pub struct SearchLojasArgs {
    pub termo: String,
}

#[derive(clap::Args)]
pub struct GetLojaBySlugArgs {
    pub slug: String,
}

#[derive(clap::Args)]
pub struct GetLojaByUuidArgs {
    pub uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Admin Lojas
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct AddFuncionarioArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub username: String,
    #[arg(long)]
    pub email: String,
    #[arg(long)]
    pub senha: String,
    #[arg(long)]
    pub celular: String,
    #[arg(long)]
    pub cargo: Option<String>,
    #[arg(long)]
    pub salario: f64,
    #[arg(long)]
    pub data_admissao: Option<String>,
}

#[derive(clap::Args)]
pub struct AddEntregadorArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub username: String,
    #[arg(long)]
    pub email: String,
    #[arg(long)]
    pub senha: String,
    #[arg(long)]
    pub celular: String,
    #[arg(long)]
    pub veiculo: Option<String>,
    #[arg(long)]
    pub placa: Option<String>,
}

#[derive(clap::Args)]
pub struct AddClienteArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub username: String,
    #[arg(long)]
    pub email: String,
    #[arg(long)]
    pub senha: String,
    #[arg(long)]
    pub celular: String,
}

#[derive(clap::Args)]
pub struct ListMinhasLojasArgs {
    #[arg(long)]
    pub admin_uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Funcionarios
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct ListFuncionariosArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdateFuncionarioArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub cargo: Option<String>,
    #[arg(long)]
    pub salario: Option<f64>,
    #[arg(long)]
    pub data_admissao: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Entregadores
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct ListEntregadoresArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdateEntregadorArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub veiculo: Option<String>,
    #[arg(long)]
    pub placa: Option<String>,
    #[arg(long)]
    pub disponivel: Option<bool>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Catalog: Produtos
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreateProdutoArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub categoria_uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub preco: f64,
    #[arg(long)]
    pub disponivel: bool,
    #[arg(long)]
    pub tempo_preparo_min: i32,
    #[arg(long)]
    pub destaque: bool,
}

#[derive(clap::Args)]
pub struct ListProdutosPorLojaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct GetProdutoArgs {
    pub uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdateProdutoArgs {
    pub uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub preco: f64,
    #[arg(long)]
    pub categoria_uuid: Uuid,
    #[arg(long)]
    pub tempo_preparo_min: i32,
}

#[derive(clap::Args)]
pub struct DeleteProdutoArgs {
    pub uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Catalog: Categorias
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreateCategoriaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub ordem: i32,
    #[arg(long, default_value = "false")]
    pub pizza_mode: bool,
}

#[derive(clap::Args)]
pub struct ListCategoriasArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdateCategoriaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub ordem: i32,
    #[arg(long)]
    pub pizza_mode: bool,
}

#[derive(clap::Args)]
pub struct DeleteCategoriaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Catalog: Adicionais
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreateAdicionalArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub descricao: String,
    #[arg(long)]
    pub preco: f64,
}

#[derive(clap::Args)]
pub struct ListAdicionaisArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdateAdicionalArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub descricao: String,
    #[arg(long)]
    pub preco: f64,
}

#[derive(clap::Args)]
pub struct DeleteAdicionalArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Pedidos
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreatePedidoArgs {
    #[arg(long)]
    pub usuario_uuid: Uuid,
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub taxa_entrega: f64,
    #[arg(long)]
    pub subtotal: f64,
    #[arg(long)]
    pub forma_pagamento: String,
    #[arg(long)]
    pub observacoes: Option<String>,
    #[arg(long)]
    pub cupom: Option<String>,
}

#[derive(clap::Args)]
pub struct ListMeusPedidosArgs {
    #[arg(long)]
    pub usuario_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct ListPedidosPorLojaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct GetPedidoArgs {
    pub uuid: Uuid,
}

#[derive(clap::Args)]
pub struct GetPedidoComEntregaArgs {
    pub uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdatePedidoStatusArgs {
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub novo_status: String,
}

#[derive(clap::Args)]
pub struct AtribuirEntregadorArgs {
    #[arg(long)]
    pub pedido_uuid: Uuid,
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub entregador_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct RemoverEntregadorArgs {
    #[arg(long)]
    pub pedido_uuid: Uuid,
    #[arg(long)]
    pub loja_uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Endereco de Entrega
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreateEnderecoEntregaArgs {
    #[arg(long)]
    pub pedido_uuid: Uuid,
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub logradouro: String,
    #[arg(long)]
    pub numero: String,
    #[arg(long)]
    pub bairro: String,
    #[arg(long)]
    pub cidade: String,
    #[arg(long)]
    pub estado: String,
    #[arg(long)]
    pub cep: Option<String>,
    #[arg(long)]
    pub complemento: Option<String>,
}

#[derive(clap::Args)]
pub struct GetEnderecoEntregaArgs {
    pub pedido_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct ListEnderecosEntregaArgs {
    pub loja_uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Endereco de Loja
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct ListEnderecosLojaArgs {
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct CreateEnderecoLojaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub logradouro: String,
    #[arg(long)]
    pub numero: String,
    #[arg(long)]
    pub bairro: String,
    #[arg(long)]
    pub cidade: String,
    #[arg(long)]
    pub estado: String,
    #[arg(long)]
    pub cep: Option<String>,
    #[arg(long)]
    pub complemento: Option<String>,
}

#[derive(clap::Args)]
pub struct UpdateEnderecoLojaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub logradouro: Option<String>,
    #[arg(long)]
    pub numero: Option<String>,
    #[arg(long)]
    pub bairro: Option<String>,
    #[arg(long)]
    pub cidade: Option<String>,
    #[arg(long)]
    pub estado: Option<String>,
    #[arg(long)]
    pub cep: Option<String>,
    #[arg(long)]
    pub complemento: Option<String>,
}

#[derive(clap::Args)]
pub struct DeleteEnderecoLojaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Endereco de Usuario
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreateEnderecoUsuarioArgs {
    #[arg(long)]
    pub logradouro: String,
    #[arg(long)]
    pub numero: String,
    #[arg(long)]
    pub bairro: String,
    #[arg(long)]
    pub cidade: String,
    #[arg(long)]
    pub estado: String,
    #[arg(long)]
    pub cep: Option<String>,
    #[arg(long)]
    pub complemento: Option<String>,
}

#[derive(clap::Args)]
pub struct ListEnderecosUsuarioArgs {
    pub usuario_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct GetEnderecoUsuarioArgs {
    pub uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdateEnderecoUsuarioArgs {
    pub uuid: Uuid,
    #[arg(long)]
    pub logradouro: Option<String>,
    #[arg(long)]
    pub numero: Option<String>,
    #[arg(long)]
    pub bairro: Option<String>,
    #[arg(long)]
    pub cidade: Option<String>,
    #[arg(long)]
    pub estado: Option<String>,
    #[arg(long)]
    pub cep: Option<String>,
    #[arg(long)]
    pub complemento: Option<String>,
}

#[derive(clap::Args)]
pub struct DeleteEnderecoUsuarioArgs {
    pub uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Marketing: Cupons
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreateCupomArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub codigo: String,
    #[arg(long)]
    pub descricao: String,
    #[arg(long)]
    pub tipo_desconto: String,
    #[arg(long)]
    pub valor_desconto: f64,
    #[arg(long)]
    pub valor_minimo: f64,
    #[arg(long)]
    pub data_validade: String,
    #[arg(long)]
    pub limite_uso: i32,
}

#[derive(clap::Args)]
pub struct ValidarCupomArgs {
    pub codigo: String,
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct GetCupomArgs {
    pub uuid: Uuid,
}

#[derive(clap::Args)]
pub struct ListCuponsArgs {
    #[arg(long)]
    pub loja_uuid: Option<Uuid>,
}

#[derive(clap::Args)]
pub struct UpdateCupomArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub codigo: Option<String>,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub tipo_desconto: Option<String>,
    #[arg(long)]
    pub valor_desconto: Option<f64>,
    #[arg(long)]
    pub valor_minimo: Option<f64>,
    #[arg(long)]
    pub data_validade: Option<String>,
    #[arg(long)]
    pub limite_uso: Option<i32>,
    #[arg(long)]
    pub status: Option<String>,
}

#[derive(clap::Args)]
pub struct DeleteCupomArgs {
    pub uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Marketing: Promocoes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreatePromocaoArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub descricao: String,
    #[arg(long)]
    pub tipo_desconto: String,
    #[arg(long)]
    pub valor_desconto: f64,
    #[arg(long)]
    pub valor_minimo: Option<f64>,
    #[arg(long)]
    pub data_inicio: String,
    #[arg(long)]
    pub data_fim: String,
    #[arg(long)]
    pub dias_semana: Vec<i32>,
    #[arg(long)]
    pub tipo_escopo: String,
    #[arg(long)]
    pub produto_uuid: Option<Uuid>,
    #[arg(long)]
    pub categoria_uuid: Option<Uuid>,
    #[arg(long)]
    pub prioridade: i32,
}

#[derive(clap::Args)]
pub struct ListPromocoesArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdatePromocaoArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub nome: Option<String>,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub tipo_desconto: Option<String>,
    #[arg(long)]
    pub valor_desconto: Option<f64>,
    #[arg(long)]
    pub valor_minimo: Option<f64>,
    #[arg(long)]
    pub data_inicio: Option<String>,
    #[arg(long)]
    pub data_fim: Option<String>,
    #[arg(long)]
    pub dias_semana: Option<Vec<i32>>,
    #[arg(long)]
    pub tipo_escopo: Option<String>,
    #[arg(long)]
    pub produto_uuid: Option<Uuid>,
    #[arg(long)]
    pub categoria_uuid: Option<Uuid>,
    #[arg(long)]
    pub prioridade: Option<i32>,
    #[arg(long)]
    pub status: Option<String>,
}

#[derive(clap::Args)]
pub struct DeletePromocaoArgs {
    pub uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Marketing: Avaliacoes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct AvaliarLojaArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub usuario_uuid: Uuid,
    #[arg(long)]
    pub nota: f64,
    #[arg(long)]
    pub comentario: Option<String>,
}

#[derive(clap::Args)]
pub struct AvaliarProdutoArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub usuario_uuid: Uuid,
    #[arg(long)]
    pub produto_uuid: Uuid,
    #[arg(long)]
    pub nota: f64,
    #[arg(long)]
    pub descricao: String,
    #[arg(long)]
    pub comentario: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Favoritos
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct AddFavoritoArgs {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct RemoveFavoritoArgs {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct VerificarFavoritoArgs {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
}

// ─────────────────────────────────────────────────────────────────────────────
// Horarios
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct ListHorariosArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct CreateHorarioArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub dia_semana: i32,
    #[arg(long)]
    pub abertura: String,
    #[arg(long)]
    pub fechamento: String,
}

#[derive(clap::Args)]
pub struct ToggleHorarioAtivoArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub dia_semana: i32,
    #[arg(long)]
    pub ativo: bool,
}

#[derive(clap::Args)]
pub struct DeleteHorarioArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub dia_semana: i32,
}

// ─────────────────────────────────────────────────────────────────────────────
// Config Pedido
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct GetConfigPedidoArgs {
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct SaveConfigPedidoArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub max_partes: i32,
    #[arg(long, default_value = "mais_caro")]
    pub tipo_calculo: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Ingredientes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CreateIngredienteArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub nome: String,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub preco: f64,
}

#[derive(clap::Args)]
pub struct ListIngredientesArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
}

#[derive(clap::Args)]
pub struct UpdateIngredienteArgs {
    #[arg(long)]
    pub loja_uuid: Uuid,
    #[arg(long)]
    pub uuid: Uuid,
    #[arg(long)]
    pub nome: Option<String>,
    #[arg(long)]
    pub descricao: Option<String>,
    #[arg(long)]
    pub preco: Option<f64>,
}

#[derive(clap::Args)]
pub struct DeleteIngredienteArgs {
    pub uuid: Uuid,
}
