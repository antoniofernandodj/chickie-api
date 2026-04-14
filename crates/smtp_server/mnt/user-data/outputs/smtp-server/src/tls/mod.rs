use anyhow::Result;
use std::fs;
use tokio::net::TcpStream;
use tokio_rustls::rustls::{self, ServerConfig};
use tracing::info;

pub type TlsAcceptor = tokio_rustls::TlsAcceptor;

/// Carrega a configuração TLS a partir dos arquivos de certificado e chave
pub async fn load_tls_config(config: &crate::Config) -> Result<TlsAcceptor> {
    let cert_path = &config.tls_cert_path;
    let key_path = &config.tls_key_path;

    info!("Carregando certificado TLS de: {}", cert_path);
    info!("Carregando chave privada de: {}", key_path);

    let cert_data = fs::read(cert_path)
        .map_err(|e| anyhow::anyhow!("Erro ao ler certificado '{}': {}", cert_path, e))?;

    let key_data = fs::read(key_path)
        .map_err(|e| anyhow::anyhow!("Erro ao ler chave privada '{}': {}", key_path, e))?;

    // Parseia certificados PEM
    let certs: Vec<rustls::pki_types::CertificateDer> =
        rustls_pemfile::certs(&mut cert_data.as_slice())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| anyhow::anyhow!("Erro ao parsear certificado: {}", e))?;

    if certs.is_empty() {
        return Err(anyhow::anyhow!("Nenhum certificado encontrado em '{}'", cert_path));
    }

    // Parseia chave privada PEM (suporta RSA e EC)
    let key = rustls_pemfile::private_key(&mut key_data.as_slice())
        .map_err(|e| anyhow::anyhow!("Erro ao parsear chave privada: {}", e))?
        .ok_or_else(|| anyhow::anyhow!("Nenhuma chave privada encontrada em '{}'", key_path))?;

    // Constrói a configuração TLS do servidor
    let server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| anyhow::anyhow!("Erro na configuração TLS: {}", e))?;

    Ok(TlsAcceptor::from(std::sync::Arc::new(server_config)))
}

/// Gera certificados auto-assinados para desenvolvimento
/// Requer que o binário `openssl` esteja disponível no sistema
pub async fn generate_self_signed_certs(cert_dir: &str) -> Result<()> {
    use std::process::Command;

    fs::create_dir_all(cert_dir)?;

    let cert_path = format!("{}/cert.pem", cert_dir);
    let key_path = format!("{}/key.pem", cert_dir);

    let output = Command::new("openssl")
        .args([
            "req", "-x509", "-newkey", "rsa:4096",
            "-keyout", &key_path,
            "-out", &cert_path,
            "-days", "365",
            "-nodes",
            "-subj", "/C=BR/ST=RJ/L=Rio de Janeiro/O=smtp-server/CN=localhost",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Falha ao gerar certificados: {}", stderr));
    }

    info!("Certificados auto-assinados gerados em '{}'", cert_dir);
    Ok(())
}
