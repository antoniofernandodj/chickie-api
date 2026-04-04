use std::sync::Arc;

use uuid::Uuid;

use crate::models::{
    Cliente,
    ClasseUsuario,
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
    UsuarioRepository,
    Repository as _
};

pub struct LojaService {
    loja_repo: Arc<LojaRepository>,
    config_repo: Arc<ConfiguracaoPedidosLojaRepository>,
    horario_repo: Arc<HorarioFuncionamentoRepository>,
    funcionario_repo: Arc<FuncionarioRepository>,
    entregador_repo: Arc<EntregadorRepository>,
    cliente_repo: Arc<ClienteRepository>,
    usuario_repo: Arc<UsuarioRepository>,
}

impl LojaService {
    pub fn new(
        loja_repo: Arc<LojaRepository>,
        config_repo: Arc<ConfiguracaoPedidosLojaRepository>,
        horario_repo: Arc<HorarioFuncionamentoRepository>,
        funcionario_repo: Arc<FuncionarioRepository>,
        entregador_repo: Arc<EntregadorRepository>,
        cliente_repo: Arc<ClienteRepository>,
        usuario_repo: Arc<UsuarioRepository>,
    ) -> Self {

        Self {
            loja_repo,
            config_repo,
            horario_repo,
            funcionario_repo,
            entregador_repo,
            cliente_repo,
            usuario_repo,
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
        dias_funcionamento: Option<Vec<i32>>,
        tempo_preparo_min: Option<i32>,
        taxa_entrega: Option<f64>,
        valor_minimo_pedido: Option<f64>,
        raio_entrega_km: Option<f64>,
        max_partes: i32,
        tipo_calculo: TipoCalculoPedido
    ) -> Result<Loja, String> {
        // 1. Cria a loja — converte String para NaiveTime
        let hora_abertura = horario_abertura
            .map(|h| chrono::NaiveTime::parse_from_str(&h, "%H:%M")
                .map_err(|e| format!("horario_abertura inválido '{}': {}", h, e)))
            .transpose()?;

        let hora_fechamento = horario_fechamento
            .map(|h| chrono::NaiveTime::parse_from_str(&h, "%H:%M")
                .map_err(|e| format!("horario_fechamento inválido '{}': {}", h, e)))
            .transpose()?;

        let loja = Loja::new(
            nome,
            slug,
            email,
            descricao,
            telefone,
            hora_abertura,
            hora_fechamento,
            dias_funcionamento,
            tempo_preparo_min,
            taxa_entrega,
            valor_minimo_pedido,
            raio_entrega_km,
        );

        self.loja_repo.criar(&loja).await?;
        tracing::info!("Loja criada: {:?}", loja.nome);

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
        username: String,
        email: String,
        senha: String,
        celular: String,
        cargo: Option<String>,
        salario: Option<f64>,
        data_admissao: String,
    ) -> Result<Funcionario, String> {

        // 1. Cria o usuário com classe "funcionario"
        let senha_hash = bcrypt::hash(&senha, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?;

        let usuario = Usuario::new(
            nome,
            username,
            email,
            senha_hash,
            celular,
            "email".to_string(),
            ClasseUsuario::Funcionario,
        );

        self.usuario_repo.criar(&usuario).await?;

        // 2. Vincula o funcionário à loja
        let funcionario = Funcionario::new(
            loja_uuid,
            usuario.uuid,
            cargo,
            salario,
            data_admissao,
        );

        self.funcionario_repo.criar(&funcionario).await?;

        Ok(funcionario)
    }

    pub async fn adicionar_cliente(
        &self,
        loja_uuid: Uuid,
        nome: String,
        username: String,
        email: String,
        senha: String,
        celular: String,
        // telefone: Option<String>,
    ) -> Result<Cliente, String> {

        // 1. Cria o usuário com classe "cliente"
        let senha_hash = bcrypt::hash(senha, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?;

        let usuario = Usuario::new(
            nome,
            username,
            email.clone(),
            senha_hash,
            celular,
            "email".to_string(),
            ClasseUsuario::Cliente,
        );

        self.usuario_repo.criar(&usuario).await?;

        // 2. Vincula o cliente à loja
        let cliente = Cliente::new(usuario.uuid, loja_uuid);
        self.cliente_repo.criar(&cliente).await?;

        Ok(cliente)
    }

    pub async fn adicionar_entregador(
        &self,
        loja_uuid: Uuid,
        nome: String,
        username: String,
        email: String,
        senha: String,
        celular: String,
        veiculo: Option<String>,
        placa: Option<String>,
    ) -> Result<Entregador, String> {

        // 1. Cria o usuário com classe "entregador"
        let senha_hash = bcrypt::hash(&senha, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?;

        let usuario = Usuario::new(
            nome,
            username,
            email,
            senha_hash,
            celular,
            "email".to_string(),
            ClasseUsuario::Entregador,
        );

        self.usuario_repo.criar(&usuario).await?;

        // 2. Vincula o entregador à loja
        let entregador = Entregador::new(
            loja_uuid,
            usuario.uuid,
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