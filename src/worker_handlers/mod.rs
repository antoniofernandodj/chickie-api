use tracing::info;
use anyhow::Result;


pub async fn handle_email(body: String) -> Result<()> {
    info!("📧 Processando email: {body}");
    // sua lógica aqui...
    Ok(())
}

pub async fn handle_notification(body: String) -> Result<()> {
    info!("🔔 Processando notificação: {body}");
    Ok(())
}

pub async fn handle_report(body: String) -> Result<()> {
    info!("📊 Gerando relatório: {body}");
    Ok(())
}

pub mod register;