use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::{Ingrediente, HorarioFuncionamento, ConfiguracaoDePedidosLoja, Funcionario, Entregador, Cupom, Usuario},
    services::{
        IngredienteService,
        HorarioFuncionamentoService,
        ConfiguracaoPedidosLojaService,
        FuncionarioService,
        EntregadorService,
        MarketingService
    }
};

pub struct AdminUsecase {
    ingrediente_service: Arc<IngredienteService>,
    horario_service: Arc<HorarioFuncionamentoService>,
    config_service: Arc<ConfiguracaoPedidosLojaService>,
    funcionario_service: Arc<FuncionarioService>,
    entregador_service: Arc<EntregadorService>,
    marketing_service: Arc<MarketingService>,
    pub usuario: Usuario,
    pub loja_uuid: Uuid,
}

impl AdminUsecase {
    pub fn new(
        ingrediente_service: Arc<IngredienteService>,
        horario_service: Arc<HorarioFuncionamentoService>,
        config_service: Arc<ConfiguracaoPedidosLojaService>,
        funcionario_service: Arc<FuncionarioService>,
        entregador_service: Arc<EntregadorService>,
        marketing_service: Arc<MarketingService>,
        usuario: Usuario,
        loja_uuid: Uuid,
    ) -> Self {
        Self {
            ingrediente_service,
            horario_service,
            config_service,
            funcionario_service,
            entregador_service,
            marketing_service,
            usuario,
            loja_uuid,
        }
    }

    // ─── Ingredientes ───
    pub async fn criar_ingrediente(&self, nome: String, unidade_medida: Option<String>, preco_unitario: f64) -> Result<Ingrediente, String> {
        self.ingrediente_service.criar(self.loja_uuid, nome, unidade_medida, preco_unitario).await
    }
    pub async fn listar_ingredientes(&self) -> Result<Vec<Ingrediente>, String> {
        self.ingrediente_service.listar_por_loja(self.loja_uuid).await
    }
    pub async fn atualizar_ingrediente(&self, uuid: Uuid, nome: String, unidade_medida: Option<String>, quantidade: f64, preco_unitario: f64) -> Result<(), String> {
        self.ingrediente_service.atualizar(uuid, nome, unidade_medida, quantidade, preco_unitario).await
    }
    pub async fn deletar_ingrediente(&self, uuid: Uuid) -> Result<(), String> {
        self.ingrediente_service.deletar(uuid).await
    }

    // ─── Horários ───
    pub async fn listar_horarios(&self) -> Result<Vec<HorarioFuncionamento>, String> {
        self.horario_service.listar_por_loja(self.loja_uuid).await
    }
    pub async fn criar_ou_atualizar_horario(&self, horario: &HorarioFuncionamento) -> Result<(), String> {
        self.horario_service.criar_ou_atualizar(horario).await
    }
    pub async fn definir_horario_ativo(&self, dia_semana: i32, ativo: bool) -> Result<(), String> {
        self.horario_service.definir_ativo(self.loja_uuid, dia_semana, ativo).await
    }
    pub async fn deletar_horario_dia(&self, dia_semana: i32) -> Result<(), String> {
        self.horario_service.deletar_por_dia(self.loja_uuid, dia_semana).await
    }

    // ─── Configurações de Pedido ───
    pub async fn buscar_config_pedido(&self) -> Result<Option<ConfiguracaoDePedidosLoja>, String> {
        self.config_service.buscar(self.loja_uuid).await
    }
    pub async fn salvar_config_pedido(&self, config: &ConfiguracaoDePedidosLoja) -> Result<(), String> {
        self.config_service.salvar(config).await
    }

    // ─── Funcionários ───
    pub async fn listar_funcionarios(&self) -> Result<Vec<Funcionario>, String> {
        self.funcionario_service.listar_por_loja(self.loja_uuid).await
    }
    pub async fn atualizar_funcionario(&self, uuid: Uuid, usuario_uuid: Uuid, nome: Option<String>, email: Option<String>, senha: Option<String>, celular: Option<String>, telefone: Option<String>, cargo: Option<String>, salario: Option<f64>, data_admissao: String) -> Result<(), String> {
        self.funcionario_service.atualizar(uuid, usuario_uuid, nome, email, senha, celular, telefone, cargo, salario, data_admissao).await
    }
    pub async fn funcionario_trocar_email_senha(&self, usuario_uuid: Uuid, novo_email: Option<String>, nova_senha: Option<String>) -> Result<(), String> {
        self.funcionario_service.trocar_email_senha(usuario_uuid, novo_email, nova_senha).await
    }
    pub async fn deletar_funcionario(&self, uuid: Uuid) -> Result<(), String> {
        self.funcionario_service.deletar(uuid).await
    }

    // ─── Entregadores ───
    pub async fn listar_entregadores(&self) -> Result<Vec<Entregador>, String> {
        self.entregador_service.listar_por_loja(self.loja_uuid).await
    }
    pub async fn atualizar_entregador(&self, uuid: Uuid, usuario_uuid: Uuid, nome: Option<String>, celular: Option<String>, telefone: Option<String>, veiculo: Option<String>, placa: Option<String>) -> Result<(), String> {
        self.entregador_service.atualizar(uuid, usuario_uuid, nome, celular, telefone, veiculo, placa).await
    }
    pub async fn entregador_trocar_email_senha(&self, usuario_uuid: Uuid, novo_email: Option<String>, nova_senha: Option<String>) -> Result<(), String> {
        self.entregador_service.trocar_email_senha(usuario_uuid, novo_email, nova_senha).await
    }
    pub async fn definir_entregador_disponivel(&self, uuid: Uuid, disponivel: bool) -> Result<(), String> {
        self.entregador_service.definir_disponivel(uuid, disponivel).await
    }
    pub async fn deletar_entregador(&self, uuid: Uuid) -> Result<(), String> {
        self.entregador_service.deletar(uuid).await
    }

    // ─── Cupons ───
    pub async fn atualizar_cupom(&self, uuid: Uuid, codigo: String, descricao: String, tipo_desconto: String, valor_desconto: Option<f64>, valor_minimo: Option<f64>, data_validade: String, limite_uso: Option<i32>) -> Result<(), String> {
        self.marketing_service.atualizar_cupom(uuid, self.loja_uuid, codigo, descricao, tipo_desconto, valor_desconto, valor_minimo, data_validade, limite_uso).await
    }
    pub async fn deletar_cupom(&self, uuid: Uuid) -> Result<(), String> {
        self.marketing_service.deletar_cupom(uuid).await
    }
}
