use anyhow::Result;
use chrono::Utc;
use std::env;
use std::io::{self, Write};
use crate::Config;


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

pub fn load_config() -> Result<Config> {
    let path = get_config_path(); 
    log_info(&format!("📄 Carregando config de: {}", &path));
    let content = std::fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Não foi possível ler o arquivo '{}': {}", path, e))?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Erro ao fazer parse do TOML: {}", e))?;
    
    Ok(config)
}