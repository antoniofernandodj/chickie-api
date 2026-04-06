use std::sync::Arc;

use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use chrono::NaiveDate;
use sea_orm::Set;

use crate::entities::loja::Model as Loja;
use crate::entities::cliente::Model as Cliente;
use crate::entities::configuracoes_pedidos_loja::Model as ConfiguracaoDePedidosLoja;
use crate::entities::entregador::Model as Entregador;
use crate::entities::funcionario::Model as Funcionario;
use crate::entities::horarios_funcionamento::Model as HorarioFuncionamento;
use crate::entities::usuario::Model as Usuario;

use crate::models::{
    ClasseUsuario,
    TipoCalculoPedido,
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
        criado_por: Uuid,
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

        let loja = Loja {
            uuid: Uuid::new_v4(),
            nome,
            slug,
            descricao,
            email,
            telefone,
            ativa: true,
            logo_url: None,
            banner_url: None,
            horario_abertura: hora_abertura,
            horario_fechamento: hora_fechamento,
            dias_funcionamento,
            tempo_preparo_min,
            taxa_entrega: taxa_entrega.and_then(|v| Decimal::from_f64(v)),
            valor_minimo_pedido: valor_minimo_pedido.and_then(|v| Decimal::from_f64(v)),
            raio_entrega_km: raio_entrega_km.and_then(|v| Decimal::from_f64(v)),
            criado_por: Some(criado_por),
            criado_em: chrono::Utc::now(),
            atualizado_em: chrono::Utc::now(),
        };

        self.loja_repo.criar(&loja).await?;
        tracing::info!("Loja criada: {:?}", loja.nome);

        // 2. Configura partes do pedido
        let config = ConfiguracaoDePedidosLoja {
            uuid: Uuid::new_v4(),
            loja_uuid: loja.uuid,
            max_partes,
            tipo_calculo: format!("{:?}", tipo_calculo),
            criado_em: chrono::Utc::now(),
            atualizado_em: chrono::Utc::now(),
        };
        self.config_repo.salvar(&config).await?;

        // 3. Horários padrão (Seg-Sex)
        for dia in 1..=5 {
            let horario = HorarioFuncionamento {
                uuid: Uuid::new_v4(),
                loja_uuid: loja.uuid,
                dia_semana: dia,
                abertura: chrono::NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
                fechamento: chrono::NaiveTime::parse_from_str("22:00", "%H:%M").unwrap(),
                ativo: true,
                criado_em: chrono::Utc::now(),
            };

            self.horario_repo.adicionar_sem_sobrescrever(&horario).await?;
        }

        // Sábado
        let sabado = HorarioFuncionamento {
            uuid: Uuid::new_v4(),
            loja_uuid: loja.uuid,
            dia_semana: 6,
            abertura: chrono::NaiveTime::parse_from_str("08:00", "%H:%M").unwrap(),
            fechamento: chrono::NaiveTime::parse_from_str("14:00", "%H:%M").unwrap(),
            ativo: true,
            criado_em: chrono::Utc::now(),
        };

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
        salario: Option<Decimal>,
        data_admissao: NaiveDate,
    ) -> Result<Funcionario, String> {

        // 1. Cria o usuário com classe "funcionario"
        let senha_hash = bcrypt::hash(&senha, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?;

        let usuario = Usuario {
            uuid: Uuid::new_v4(),
            nome,
            username,
            email,
            senha_hash,
            celular,
            telefone: None,
            classe: "Funcionario".to_string(),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
            modo_de_cadastro: "email".to_string(),
            criado_em: chrono::Utc::now(),
            atualizado_em: chrono::Utc::now(),
        };

        self.usuario_repo.criar(&usuario).await?;

        // 2. Vincula o funcionário à loja
        let funcionario = Funcionario {
            uuid: Uuid::new_v4(),
            loja_uuid,
            usuario_uuid: usuario.uuid,
            cargo,
            salario,
            data_admissao,
            criado_em: chrono::Utc::now(),
        };

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

        let usuario = Usuario {
            uuid: Uuid::new_v4(),
            nome,
            username,
            email,
            senha_hash,
            celular,
            telefone: None,
            classe: "Cliente".to_string(),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
            modo_de_cadastro: "email".to_string(),
            criado_em: chrono::Utc::now(),
            atualizado_em: chrono::Utc::now(),
        };

        self.usuario_repo.criar(&usuario).await?;

        // 2. Vincula o cliente à loja
        let cliente = Cliente {
            uuid: Uuid::new_v4(),
            usuario_uuid: usuario.uuid,
            loja_uuid,
            criado_em: chrono::Utc::now(),
        };
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

        let usuario = Usuario {
            uuid: Uuid::new_v4(),
            nome,
            username,
            email,
            senha_hash,
            celular,
            telefone: None,
            classe: "Entregador".to_string(),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
            modo_de_cadastro: "email".to_string(),
            criado_em: chrono::Utc::now(),
            atualizado_em: chrono::Utc::now(),
        };

        self.usuario_repo.criar(&usuario).await?;

        // 2. Vincula o entregador à loja
        let entregador = Entregador {
            uuid: Uuid::new_v4(),
            loja_uuid,
            usuario_uuid: usuario.uuid,
            veiculo,
            placa,
            disponivel: true,
            criado_em: chrono::Utc::now(),
        };

        self.entregador_repo.criar(&entregador).await?;

        Ok(entregador)
    }

    pub async fn listar(&self) -> Result<Vec<Loja>, String> {
        self.loja_repo.listar_todos().await
    }

    pub async fn listar_por_criador(&self, admin_uuid: Uuid) -> Result<Vec<Loja>, String> {
        self.loja_repo.buscar_por_criador(admin_uuid).await
    }

    pub async fn pesquisar(&self, termo: &str) -> Result<Vec<Loja>, String> {
        self.loja_repo.pesquisar(termo).await
    }

    pub async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Loja>, String> {
        self.loja_repo.buscar_por_uuid(uuid).await
    }

    pub async fn buscar_por_slug(&self, slug: &str) -> Result<Option<Loja>, String> {
        self.loja_repo.buscar_por_slug(slug).await
    }
}
