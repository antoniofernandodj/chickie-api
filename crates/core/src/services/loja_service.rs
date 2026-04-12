use std::sync::Arc;

use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use chrono::NaiveDate;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

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

use crate::ports::{
    LojaRepositoryPort,
    ConfiguracaoPedidosLojaRepositoryPort,
    HorarioFuncionamentoRepositoryPort,
    FuncionarioRepositoryPort,
    EntregadorRepositoryPort,
    ClienteRepositoryPort,
    UsuarioRepositoryPort,
};

pub struct LojaService {
    loja_repo: Arc<dyn LojaRepositoryPort>,
    config_repo: Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>,
    horario_repo: Arc<dyn HorarioFuncionamentoRepositoryPort>,
    funcionario_repo: Arc<dyn FuncionarioRepositoryPort>,
    entregador_repo: Arc<dyn EntregadorRepositoryPort>,
    cliente_repo: Arc<dyn ClienteRepositoryPort>,
    usuario_repo: Arc<dyn UsuarioRepositoryPort>,
}

impl LojaService {
    pub fn new(
        loja_repo: Arc<dyn LojaRepositoryPort>,
        config_repo: Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>,
        horario_repo: Arc<dyn HorarioFuncionamentoRepositoryPort>,
        funcionario_repo: Arc<dyn FuncionarioRepositoryPort>,
        entregador_repo: Arc<dyn EntregadorRepositoryPort>,
        cliente_repo: Arc<dyn ClienteRepositoryPort>,
        usuario_repo: Arc<dyn UsuarioRepositoryPort>,
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
        celular: Option<String>,
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

        let loja = Loja::new(
            nome,
            slug,
            email,
            descricao,
            celular,
            hora_abertura,
            hora_fechamento,
            dias_funcionamento,
            tempo_preparo_min,
            Decimal::from_f64(taxa_entrega.unwrap_or_default()),
            Decimal::from_f64(valor_minimo_pedido.unwrap_or_default()),
            Decimal::from_f64(raio_entrega_km.unwrap_or_default()),
            Some(criado_por),
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
        salario: Option<Decimal>,
        data_admissao: NaiveDate,
    ) -> Result<Funcionario, String> {

        // 1. Cria o usuário com classe "funcionario"
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let senha_hash = argon2
            .hash_password(senha.as_bytes(), &salt)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?
            .to_string();

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
    ) -> Result<Cliente, String> {

        // 1. Cria o usuário com classe "cliente"
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let senha_hash = argon2
            .hash_password(senha.as_bytes(), &salt)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?
            .to_string();

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
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let senha_hash = argon2
            .hash_password(senha.as_bytes(), &salt)
            .map_err(|e| format!("Erro ao criptografar senha: {}", e))?
            .to_string();

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
        self.loja_repo.listar_todos().await.map_err(|e| e.to_string())
    }

    pub async fn listar_por_criador(&self, admin_uuid: Uuid) -> Result<Vec<Loja>, String> {
        self.loja_repo.buscar_por_criador(admin_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn pesquisar(&self, termo: &str) -> Result<Vec<Loja>, String> {
        self.loja_repo.pesquisar(termo).await.map_err(|e| e.to_string())
    }

    pub async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Loja>, String> {
        self.loja_repo.buscar_por_uuid(uuid).await.map_err(|e| e.to_string())
    }

    pub async fn buscar_por_slug(&self, slug: &str) -> Result<Option<Loja>, String> {
        self.loja_repo.buscar_por_slug(slug).await.map_err(|e| e.to_string())
    }

    pub async fn verificar_slug_disponivel(&self, slug: &str) -> Result<bool, String> {
        let existente = self.loja_repo.buscar_por_slug(slug).await?;
        Ok(existente.is_none())
    }

    // ===========================================================================
    // Soft Delete
    // ===========================================================================

    /// Marca a loja para remoção. Após 30 dias, o scheduler marcará como deletado=true.
    pub async fn marcar_para_remocao(&self, uuid: Uuid) -> Result<(), String> {
        let loja = self.loja_repo.buscar_por_uuid(uuid).await
            .map_err(|e| e.to_string())?
            .ok_or("Loja não encontrada")?;

        if loja.esta_deletada() {
            return Err("Loja já está permanentemente deletada".to_string());
        }

        if loja.esta_marcada_para_remocao() {
            return Err("Loja já está marcada para remoção".to_string());
        }

        self.loja_repo.marcar_para_remocao(uuid).await.map_err(|e| e.to_string())
    }

    /// Desmarca a remoção pendente
    pub async fn desmarcar_remocao(&self, uuid: Uuid) -> Result<(), String> {
        self.loja_repo.desmarcar_remocao(uuid).await.map_err(|e| e.to_string())
    }

    /// Alterna o status ativo da loja (bloqueio/desbloqueio administrativo)
    pub async fn alternar_ativo(&self, uuid: Uuid, ativo: bool) -> Result<(), String> {
        let loja = self.loja_repo.buscar_por_uuid(uuid).await
            .map_err(|e| e.to_string())?
            .ok_or("Loja não encontrada")?;

        if loja.esta_deletada() {
            return Err("Não é possível alterar status de loja deletada".to_string());
        }

        self.loja_repo.alterar_ativo(uuid, ativo).await.map_err(|e| e.to_string())
    }

    /// Deleta permanentemente todas as lojas marcadas para remoção há mais de 30 dias.
    /// Retorna o número de lojas deletadas.
    pub async fn deletar_pendentes_antigas(&self) -> Result<u64, String> {
        let thirty_days_ago = chrono::Utc::now() - chrono::Duration::days(30);
        self.loja_repo.deletar_pendentes_antigas(thirty_days_ago).await.map_err(|e| e.to_string())
    }
}