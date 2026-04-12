mod jobs;

use anyhow::Result;
use chrono::Utc;
use cron::Schedule;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use futures::future::join_all;
use std::sync::Arc;
use tokio::signal;
use tokio::time::{sleep, Duration};
use std::io::{self, Write};

use jobs::{
    CronJob,
    backup::BackupJob,
    cleanup::CleanupJob,
    health_check::HealthCheckJob,
    soft_delete_cleanup::SoftDeleteCleanupJob,
};
use axum::{response::Json, routing::get, Router};
use serde_json::json;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
struct JobSchedule {
    name: String,
    schedule: String,
    enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Config {
    jobs: Vec<JobSchedule>,
}

fn log_msg(level: &str, msg: &str) {
    let now = Utc::now().to_rfc3339();
    eprintln!("[{} {}] {}", now, level, msg);
    let _ = io::stderr().flush();
}

fn log_info(msg: &str) { log_msg("INFO", msg); }
fn log_error(msg: &str) { log_msg("ERROR", msg); }
fn log_warn(msg: &str) { log_msg("WARN", msg); }

fn get_config_path() -> String {
    if let Ok(path) = env::var("CONFIG_PATH") {
        return path;
    }
    if std::path::Path::new("/app/scheduler.toml").exists() {
        return "/app/scheduler.toml".to_string();
    }
    "scheduler.toml".to_string()
}

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Não foi possível ler o arquivo '{}': {}", path, e))?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Erro ao fazer parse do TOML: {}", e))?;
    
    Ok(config)
}

fn create_job_registry() -> HashMap<&'static str, Arc<dyn CronJob>> {
    let mut registry: HashMap<&'static str, Arc<dyn CronJob>> = HashMap::new();
    registry.insert("backup_job", Arc::new(BackupJob));
    registry.insert("cleanup_job", Arc::new(CleanupJob));
    registry.insert("health_check_job", Arc::new(HealthCheckJob));
    registry.insert("soft_delete_cleanup_job", Arc::new(SoftDeleteCleanupJob));
    registry
}

async fn run_scheduled_job(
    job: Arc<dyn CronJob>,
    schedule: Schedule,
) -> Result<()> {
    let job_name = job.name();
    log_info(&format!("⏰ Job '{}' registrado com schedule: {}", job_name, schedule));

    loop {
        let now = Utc::now();
        let next = schedule
            .upcoming(Utc)
            .next()
            .expect("Erro ao calcular próxima execução");

        let duration = (next - now)
            .to_std()
            .unwrap_or(Duration::from_secs(0));

        log_info(&format!("⏳ Job '{}' aguardando até {} (em {:?})", job_name, next, duration));
        sleep(duration).await;

        log_info(&format!("🚀 Executando job '{}'...", job_name));

        let start = std::time::Instant::now();
        match job.execute().await {
            Ok(_) => {
                let elapsed = start.elapsed();
                log_info(&format!("✅ Job '{}' concluído em {:?}", job_name, elapsed));
            }
            Err(e) => {
                log_error(&format!("❌ Job '{}' falhou: {}", job_name, e));
            }
        }
    }
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "chickie-scheduler",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

async fn start_health_server() -> Result<()> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(health_check));

    let port = env::var("SCHEDULER_PORT")
        .unwrap_or_else(|_| "8080".to_string());
    
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    log_info(&format!("🌐 Health check server listening on http://{}", addr));

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app
    )
    .await
    .map_err(|e| anyhow::anyhow!("Health server error: {}", e))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Imediatamente escreve algo para garantir que o log funcione (stdout e stderr)
    let _ = io::stdout().write_all(b"--- CHICKIE SCHEDULER BOOTSTRAP STDOUT ---\n");
    let _ = io::stdout().flush();
    eprintln!("--- CHICKIE SCHEDULER BOOTSTRAP STDERR ---");
    let _ = io::stderr().flush();

    // Adiciona um panic hook para capturar falhas catastróficas
    std::panic::set_hook(Box::new(|info| {
        let now = Utc::now().to_rfc3339();
        eprintln!("[{} PANIC] {:?}", now, info);
        let _ = io::stderr().flush();
    }));

    log_info("🔧 Chickie Scheduler iniciando...");
    log_info(&format!("📋 PID: {}", std::process::id()));
    log_info(&format!("📂 Executável: {:?}", std::env::current_exe().unwrap_or_default()));
    log_info(&format!("👤 Usuário: {:?}", env::var("USER").unwrap_or_else(|_| "desconhecido".to_string())));
    
    let current_dir = std::env::current_dir().unwrap_or_default();
    log_info(&format!("📁 Working directory: {}", current_dir.display()));
    
    // Lista arquivos no diretório atual para conferir se o scheduler.toml está lá
    if let Ok(entries) = std::fs::read_dir(&current_dir) {
        let files: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        log_info(&format!("🗄️  Arquivos em {}: {:?}", current_dir.display(), files));
    }

    let config_path = get_config_path();
    log_info(&format!("📄 Carregando config de: {}", config_path));

    let config = match load_config(&config_path) {
        Ok(c) => c,
        Err(e) => {
            log_error(&format!("❌ Falha crítica ao carregar config '{}': {}", config_path, e));
            std::process::exit(1);
        }
    };

    log_info(&format!("📋 Config carregada com {} jobs definidos", config.jobs.len()));

    let registry = create_job_registry();
    let mut futures = vec![];

    for job_config in config.jobs {
        log_info(&format!("🔄 Processando job config: name='{}', enabled={:?}", 
              job_config.name, job_config.enabled));
        
        if job_config.enabled == Some(false) {
            log_info(&format!("⏭️  Job '{}' desabilitado, ignorando...", job_config.name));
            continue;
        }

        match (Schedule::from_str(&job_config.schedule), registry.get(job_config.name.as_str())) {
            (Ok(schedule), Some(job)) => {
                let job_clone = Arc::clone(job);
                log_info(&format!("🚀 Spawnando job '{}' com schedule: {}", job_config.name, job_config.schedule));
                let handle = tokio::spawn(run_scheduled_job(job_clone, schedule));
                futures.push(handle);
                log_info(&format!("✅ '{}' agendado com sucesso", job_config.name));
            }
            (Err(e), _) => log_error(&format!("❌ Schedule inválido para '{}': {}", job_config.name, e)),
            (_, None) => log_error(&format!("⚠️  Job '{}' não registrado no código", job_config.name)),
        }
    }

    if futures.is_empty() {
        log_warn("⚠️  Nenhum job ativo.");
        log_info("Aguardando sinal de parada...");
        signal::ctrl_c().await?;
        return Ok(());
    }

    log_info(&format!("🎯 {} jobs rodando. Pressione Ctrl+C para parar.", futures.len()));

    let health_handle = tokio::spawn(start_health_server());
    log_info("🏥 Health check server iniciado");

    tokio::select! {
        _ = signal::ctrl_c() => {
            log_info("🛑 Sinal de parada recebido. Encerrando...");
        }
        res = join_all(futures) => {
            log_info(&format!("🏁 Todas as jobs finalizaram (inesperado). {:?}", res));
        }
        res = health_handle => {
            log_warn(&format!("🏥 Health check server encerrou (inesperado). {:?}", res));
        }
    }

    Ok(())
}
