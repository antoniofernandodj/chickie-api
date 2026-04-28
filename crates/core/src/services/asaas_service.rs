use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc};

#[derive(Clone)]
pub struct AsaasService {
    client: Client,
    api_key: String,
    base_url: String,
}

// ─── Payloads de saída ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct CriarClientePayload {
    name: String,
    #[serde(rename = "cpfCnpj")]
    cpf_cnpj: String,
    #[serde(rename = "externalReference", skip_serializing_if = "Option::is_none")]
    external_reference: Option<String>,
}

#[derive(Serialize)]
struct CriarCobrancaPayload {
    customer: String,
    #[serde(rename = "billingType")]
    billing_type: String,
    value: f64,
    #[serde(rename = "dueDate")]
    due_date: String,
    #[serde(rename = "externalReference")]
    external_reference: String,
}

// ─── Payloads de entrada (respostas Asaas) ────────────────────────────────────

#[derive(Deserialize)]
struct AsaasCliente {
    id: String,
}

#[derive(Deserialize)]
struct AsaasListagem<T> {
    data: Vec<T>,
    #[serde(rename = "totalCount")]
    #[allow(dead_code)]
    total_count: i32,
}

#[derive(Deserialize)]
struct AsaasCobranca {
    id: String,
}

#[derive(Deserialize)]
struct AsaasPixQrCode {
    #[serde(rename = "encodedImage")]
    encoded_image: String,
    payload: String,
    #[serde(rename = "expirationDate")]
    expiration_date: Option<String>,
}

// ─── Resposta pública ─────────────────────────────────────────────────────────

pub struct PagamentoCriado {
    pub payment_id: String,
    pub qr_code_image: String,
    pub pix_copia_cola: String,
    pub vencimento: String,
}

// ─── Implementação ────────────────────────────────────────────────────────────

impl AsaasService {
    pub fn new() -> Self {
        let api_key = std::env::var("TOKEN_DE_AUTENTICACAO_ASAAS")
            .expect("Variável de ambiente TOKEN_DE_AUTENTICACAO_ASAAS não definida");
        let base_url = std::env::var("ASAAS_BASE_URL")
            .unwrap_or_else(|_| "https://api-sandbox.asaas.com/v3".to_string());
        let client = Client::builder()
            .user_agent("chickie-api/1.0")
            .build()
            .expect("Falha ao criar cliente HTTP");
        Self { client, api_key, base_url }
    }

    /// Busca customer no Asaas pelo CPF; cria um novo se não existir.
    /// Retorna o `id` do customer no Asaas.
    pub async fn cadastrar_ou_buscar_usuario_no_asaas(
        &self,
        nome: &str,
        cpf: &str,
    ) -> Result<String, String> {
        // 1. Buscar por CPF
        let url = format!("{}/customers?cpfCnpj={}", self.base_url, cpf);
        let resp = self.client
            .get(&url)
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Erro ao buscar cliente no Asaas: {}", e))?;

        if resp.status().is_success() {
            let body: AsaasListagem<AsaasCliente> = resp
                .json()
                .await
                .map_err(|e| format!("Erro ao deserializar listagem Asaas: {}", e))?;
            if let Some(cliente) = body.data.into_iter().next() {
                tracing::info!("Asaas: cliente encontrado para CPF={}", cpf);
                return Ok(cliente.id);
            }
        }

        // 2. Criar novo customer
        let url = format!("{}/customers", self.base_url);
        let payload = CriarClientePayload {
            name: nome.to_string(),
            cpf_cnpj: cpf.to_string(),
            external_reference: None,
        };

        let resp = self.client
            .post(&url)
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Erro ao criar cliente no Asaas: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Asaas retornou {} ao criar cliente: {}", status, body));
        }

        let cliente: AsaasCliente = resp
            .json()
            .await
            .map_err(|e| format!("Erro ao deserializar cliente criado: {}", e))?;

        tracing::info!("Asaas: cliente criado id={}", cliente.id);
        Ok(cliente.id)
    }

    /// Cria uma cobrança PIX e retorna o QR Code.
    pub async fn criar_cobranca_pix(
        &self,
        asaas_customer_id: &str,
        valor: Decimal,
        pedido_uuid: Uuid,
    ) -> Result<PagamentoCriado, String> {
        let due_date = (Utc::now() + Duration::days(1))
            .format("%Y-%m-%d")
            .to_string();

        let valor_f64: f64 = valor
            .to_string()
            .parse()
            .map_err(|_| "Erro ao converter valor para f64".to_string())?;

        let url = format!("{}/payments", self.base_url);
        let payload = CriarCobrancaPayload {
            customer: asaas_customer_id.to_string(),
            billing_type: "PIX".to_string(),
            value: valor_f64,
            due_date: due_date.clone(),
            external_reference: pedido_uuid.to_string(),
        };

        let resp = self.client
            .post(&url)
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Erro ao criar cobrança PIX: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Asaas retornou {} ao criar cobrança: {}", status, body));
        }

        let cobranca: AsaasCobranca = resp
            .json()
            .await
            .map_err(|e| format!("Erro ao deserializar cobrança: {}", e))?;

        tracing::info!("Asaas: cobrança criada id={} pedido={}", cobranca.id, pedido_uuid);

        // Buscar QR Code PIX
        let qr_url = format!("{}/payments/{}/pixQrCode", self.base_url, cobranca.id);
        let qr_resp = self.client
            .get(&qr_url)
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Erro ao buscar QR Code PIX: {}", e))?;

        if !qr_resp.status().is_success() {
            let status = qr_resp.status();
            let body = qr_resp.text().await.unwrap_or_default();
            return Err(format!("Asaas retornou {} ao buscar QR code: {}", status, body));
        }

        let qr: AsaasPixQrCode = qr_resp
            .json()
            .await
            .map_err(|e| format!("Erro ao deserializar QR Code: {}", e))?;

        Ok(PagamentoCriado {
            payment_id: cobranca.id,
            qr_code_image: qr.encoded_image,
            pix_copia_cola: qr.payload,
            vencimento: qr.expiration_date.unwrap_or(due_date),
        })
    }
}
