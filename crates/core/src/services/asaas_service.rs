use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc};

#[derive(Clone)]
pub struct AsaasService {
    client: Client,
    auth_token: String,
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
        let auth_token = std::env::var("TOKEN_DE_AUTENTICACAO_ASAAS")
            .expect("Variável de ambiente TOKEN_DE_AUTENTICACAO_ASAAS não definida");

        let api_key = std::env::var("ASAAS_API_KEY")
            .expect("Variável de ambiente ASAAS_API_KEY não definida");

        let base_url = std::env::var("ASAAS_BASE_URL")
            .unwrap_or_else(|_| "https://api-sandbox.asaas.com/v3".to_string());

        tracing::info!(base_url = %base_url, "asaas_service: inicializado");

        let client = Client::builder()
            .user_agent("chickie-api/1.0")
            .build()
            .expect("Falha ao criar cliente HTTP");
        Self { client, auth_token, api_key, base_url }
    }

    /// Verifica se o authToken recebido no webhook corresponde ao token configurado.
    pub fn verificar_webhook_token(&self, token: &str) -> bool {
        self.auth_token == token
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
        tracing::debug!(url = %url, cpf = %cpf, "asaas_service: GET customers — buscando cliente por CPF");

        let resp = self.client
            .get(&url)
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                tracing::error!(url = %url, erro = %e, "asaas_service: falha na requisição GET customers");
                format!("Erro ao buscar cliente no Asaas: {}", e)
            })?;

        let status = resp.status();
        tracing::debug!(url = %url, status = %status, "asaas_service: resposta GET customers");

        if resp.status().is_success() {
            let body: AsaasListagem<AsaasCliente> = resp
                .json()
                .await
                .map_err(|e| {
                    tracing::error!(erro = %e, "asaas_service: falha ao deserializar listagem de clientes");
                    format!("Erro ao deserializar listagem Asaas: {}", e)
                })?;

            tracing::debug!(cpf = %cpf, total = body.total_count, "asaas_service: listagem de clientes recebida");

            if let Some(cliente) = body.data.into_iter().next() {
                tracing::info!(cpf = %cpf, asaas_customer_id = %cliente.id, "asaas_service: cliente encontrado por CPF");
                return Ok(cliente.id);
            }

            tracing::info!(cpf = %cpf, "asaas_service: nenhum cliente encontrado para CPF — criando novo");
        } else {
            let body = resp.text().await.unwrap_or_default();
            tracing::warn!(status = %status, body = %body, cpf = %cpf, "asaas_service: GET customers retornou erro — tentando criar cliente mesmo assim");
        }

        // 2. Criar novo customer
        let url = format!("{}/customers", self.base_url);
        let payload = CriarClientePayload {
            name: nome.to_string(),
            cpf_cnpj: cpf.to_string(),
            external_reference: None,
        };

        tracing::debug!(url = %url, nome = %nome, cpf = %cpf, "asaas_service: POST customers — criando novo cliente");

        let resp = self.client
            .post(&url)
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(url = %url, nome = %nome, erro = %e, "asaas_service: falha na requisição POST customers");
                format!("Erro ao criar cliente no Asaas: {}", e)
            })?;

        let status = resp.status();
        tracing::debug!(url = %url, status = %status, "asaas_service: resposta POST customers");

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            tracing::error!(status = %status, body = %body, nome = %nome, cpf = %cpf, "asaas_service: falha ao criar cliente no Asaas");
            return Err(format!("Asaas retornou {} ao criar cliente: {}", status, body));
        }

        let cliente: AsaasCliente = resp
            .json()
            .await
            .map_err(|e| {
                tracing::error!(erro = %e, "asaas_service: falha ao deserializar cliente criado");
                format!("Erro ao deserializar cliente criado: {}", e)
            })?;

        tracing::info!(asaas_customer_id = %cliente.id, nome = %nome, cpf = %cpf, "asaas_service: cliente criado com sucesso");
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
            .map_err(|_| {
                tracing::error!(valor = %valor, "asaas_service: falha ao converter Decimal para f64");
                "Erro ao converter valor para f64".to_string()
            })?;

        let url = format!("{}/payments", self.base_url);
        let payload = CriarCobrancaPayload {
            customer: asaas_customer_id.to_string(),
            billing_type: "PIX".to_string(),
            value: valor_f64,
            due_date: due_date.clone(),
            external_reference: pedido_uuid.to_string(),
        };

        tracing::info!(
            url = %url,
            asaas_customer_id = %asaas_customer_id,
            valor = valor_f64,
            due_date = %due_date,
            pedido_uuid = %pedido_uuid,
            "asaas_service: POST payments — criando cobrança PIX"
        );

        let resp = self.client
            .post(&url)
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(url = %url, pedido_uuid = %pedido_uuid, erro = %e, "asaas_service: falha na requisição POST payments");
                format!("Erro ao criar cobrança PIX: {}", e)
            })?;

        let status = resp.status();
        tracing::debug!(url = %url, status = %status, pedido_uuid = %pedido_uuid, "asaas_service: resposta POST payments");

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            tracing::error!(
                status = %status,
                body = %body,
                pedido_uuid = %pedido_uuid,
                asaas_customer_id = %asaas_customer_id,
                "asaas_service: falha ao criar cobrança PIX"
            );
            return Err(format!("Asaas retornou {} ao criar cobrança: {}", status, body));
        }

        let cobranca: AsaasCobranca = resp
            .json()
            .await
            .map_err(|e| {
                tracing::error!(pedido_uuid = %pedido_uuid, erro = %e, "asaas_service: falha ao deserializar cobrança criada");
                format!("Erro ao deserializar cobrança: {}", e)
            })?;

        tracing::info!(
            payment_id = %cobranca.id,
            pedido_uuid = %pedido_uuid,
            "asaas_service: cobrança criada — buscando QR Code PIX"
        );

        // Buscar QR Code PIX
        let qr_url = format!("{}/payments/{}/pixQrCode", self.base_url, cobranca.id);
        tracing::debug!(url = %qr_url, payment_id = %cobranca.id, "asaas_service: GET pixQrCode");

        let qr_resp = self.client
            .get(&qr_url)
            .header("access_token", &self.api_key)
            .header("accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                tracing::error!(url = %qr_url, payment_id = %cobranca.id, erro = %e, "asaas_service: falha na requisição GET pixQrCode");
                format!("Erro ao buscar QR Code PIX: {}", e)
            })?;

        let qr_status = qr_resp.status();
        tracing::debug!(url = %qr_url, status = %qr_status, payment_id = %cobranca.id, "asaas_service: resposta GET pixQrCode");

        if !qr_resp.status().is_success() {
            let body = qr_resp.text().await.unwrap_or_default();
            tracing::error!(
                status = %qr_status,
                body = %body,
                payment_id = %cobranca.id,
                pedido_uuid = %pedido_uuid,
                "asaas_service: falha ao buscar QR Code PIX"
            );
            return Err(format!("Asaas retornou {} ao buscar QR code: {}", qr_status, body));
        }

        let qr: AsaasPixQrCode = qr_resp
            .json()
            .await
            .map_err(|e| {
                tracing::error!(payment_id = %cobranca.id, erro = %e, "asaas_service: falha ao deserializar QR Code");
                format!("Erro ao deserializar QR Code: {}", e)
            })?;

        tracing::info!(
            payment_id = %cobranca.id,
            pedido_uuid = %pedido_uuid,
            vencimento = qr.expiration_date.as_deref().unwrap_or(&due_date),
            "asaas_service: QR Code PIX obtido com sucesso"
        );

        Ok(PagamentoCriado {
            payment_id: cobranca.id,
            qr_code_image: qr.encoded_image,
            pix_copia_cola: qr.payload,
            vencimento: qr.expiration_date.unwrap_or(due_date),
        })
    }
}
