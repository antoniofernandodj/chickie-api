use std::sync::Arc;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::models::{
    Cliente,
    ConfiguracaoDePedidosLoja,
    Entregador,
    Funcionario,
    HorarioFuncionamento,
    Loja,
    TipoCalculoPedido,
    Usuario
};

use crate::repositories::{
    LojaRepository,
    ConfiguracaoPedidosLojaRepository,
    HorarioFuncionamentoRepository,
    FuncionarioRepository,
    EntregadorRepository,
    ClienteRepository,
    Repository as _
};

pub struct LojaService {
    loja_repo: Arc<LojaRepository>,
    config_repo: Arc<ConfiguracaoPedidosLojaRepository>,
    horario_repo: Arc<HorarioFuncionamentoRepository>,
    funcionario_repo: Arc<FuncionarioRepository>,
    entregador_repo: Arc<EntregadorRepository>,
    cliente_repo: Arc<ClienteRepository>
}

impl LojaService {
    pub fn new(
        loja_repo: Arc<LojaRepository>,
        config_repo: Arc<ConfiguracaoPedidosLojaRepository>,
        horario_repo: Arc<HorarioFuncionamentoRepository>,
        funcionario_repo: Arc<FuncionarioRepository>,
        entregador_repo: Arc<EntregadorRepository>,
        cliente_repo: Arc<ClienteRepository>
    ) -> Self {

        Self {
            loja_repo,
            config_repo,
            horario_repo,
            funcionario_repo,
            entregador_repo,
            cliente_repo
        }
    }

    pub async fn criar_loja_completa(
        &self,
        nome: String,
        slug: String,
        email: String,
        descricao: Option<String>,
        telefone: Option<String>,
        horario_abertura: Option<String>,
        horario_fechamento: Option<String>,
        dias_funcionamento: Option<String>,
        tempo_preparo_min: Option<i32>,
        taxa_entrega: Option<f64>,
        valor_minimo_pedido: Option<f64>,
        raio_entrega_km: Option<f64>,
        max_partes: i32,
        tipo_calculo: TipoCalculoPedido
    ) -> Result<Loja, String> {
        // 1. Cria a loja

        let loja = Loja::new(
            nome,
            slug,
            email,
            descricao,
            telefone,
            horario_abertura,
            horario_fechamento,
            dias_funcionamento,
            tempo_preparo_min,
            taxa_entrega,
            valor_minimo_pedido,
            raio_entrega_km,
        );

        self.loja_repo.criar(&loja).await?;
        println!("Loja criada: {:?}", loja.nome);

        // 2. Configura partes do pedido
        let config = ConfiguracaoDePedidosLoja::new(
            loja.uuid,
            max_partes,
            tipo_calculo
        )
        .expect("Falha ao criar config de pedidos");
        self.config_repo.salvar(&config).await?;

        // 3. Horários padrão (Seg-Sex)
        for dia in 1..=5 {
            let horario = HorarioFuncionamento::new(
                loja.uuid,
                dia,
                "08:00".into(),
                "22:00".into()
            ).unwrap();

            self.horario_repo.adicionar_sem_sobrescrever(&horario).await?;
        }

        // Sábado
        let sabado = HorarioFuncionamento::new(
            loja.uuid,
            6,
            "08:00".into(),
            "14:00".into()
        ).unwrap();

        self.horario_repo.adicionar_sem_sobrescrever(&sabado).await?;

        Ok(loja)
    }

    pub async fn adicionar_funcionario(
        &self,
        loja_uuid: Uuid,
        nome: String,
        email: Option<String>,
        cargo: Option<String>,
        salario: Option<f64>,
        data_admissao: String,
    ) -> Result<Funcionario, String> {

        let funcionario: Funcionario = Funcionario::new(
            loja_uuid,
            nome,
            email,
            cargo,
            salario,
            data_admissao,
        );

        self.funcionario_repo.criar(&funcionario).await?;

        Ok(funcionario)
    }

    pub async fn adicionar_cliente(
        &self,
        usuario: &Usuario,
        loja: &Loja
    ) -> Result<(), String> {

        let cliente: Cliente = Cliente::new(usuario.uuid, loja.uuid);

        println!("cliente: {:?}", cliente);

        self.cliente_repo.criar(&cliente).await;

        Ok(())
    }

    pub async fn adicionar_entregador(
        &self,
        nome: String,
        loja_uuid: Uuid,
        telefone: Option<String>,
        veiculo: Option<String>,
        placa: Option<String>,
    ) -> Result<Entregador, String> {

        let entregador: Entregador = Entregador::new(
            nome,
            loja_uuid,
            telefone,
            veiculo,
            placa,
        );

        self.entregador_repo.criar(&entregador).await?;

        Ok(entregador)
    }
    
    pub async fn listar(&self) -> Result<Vec<Loja>, String> {
        self.loja_repo.listar_todos().await
    }
}