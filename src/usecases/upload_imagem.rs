use aws_sdk_s3::primitives::ByteStream;
use uuid::Uuid;

use crate::{
    repositories::{ProdutoRepository, Repository as _},
    models::Produto,
};

pub struct UploadImagemUsecase {
    produto_repo: ProdutoRepository,
    bucket: String,
    endpoint: Option<String>,
    region: Option<String>,
}

impl UploadImagemUsecase {
    pub fn new(
        produto_repo: ProdutoRepository,
        bucket: String,
        endpoint: Option<String>,
        region: Option<String>,
    ) -> Self {
        Self {
            produto_repo,
            bucket,
            endpoint,
            region,
        }
    }

    async fn s3_client(&self) -> aws_sdk_s3::Client {
        let access_key = std::env::var("S3_ACCESS_KEY_ID").ok();
        let secret_key = std::env::var("S3_SECRET_ACCESS_KEY").ok();

        if access_key.is_some() && secret_key.is_some() {
            let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());
            if let Some(ref ep) = self.endpoint {
                loader = loader.endpoint_url(ep);
            }
            aws_sdk_s3::Client::new(&loader.load().await)
        } else {
            aws_sdk_s3::Client::new(&aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await)
        }
    }

    pub async fn executar(
        &self,
        produto_uuid: Uuid,
        filename: String,
        content_type: String,
        data: bytes::Bytes,
    ) -> Result<String, String> {

        // Verificar se produto existe
        let _produto = self.produto_repo.buscar_por_uuid(produto_uuid).await
            .map_err(|e| format!("Erro ao buscar produto: {}", e))?
            .ok_or("Produto não encontrado".to_string())?;

        // Upload para S3
        let object_key = format!("produtos/{}-{}", produto_uuid, filename);
        let client = self.s3_client().await;

        client
            .put_object()
            .bucket(&self.bucket)
            .key(&object_key)
            .content_type(&content_type)
            .body(ByteStream::from(data))
            .send()
            .await
            .map_err(|e| format!("Failed to upload to S3: {}", e))?;

        // Construir URL do arquivo
        let file_url = if let Some(ref ep) = self.endpoint {
            format!("{}/{}/{}", ep.trim_end_matches('/'), self.bucket, object_key)
        } else {
            let region = self.region.as_deref().unwrap_or("us-east-1");
            format!("https://{}.s3.{}.amazonaws.com/{}", self.bucket, region, object_key)
        };

        Ok(file_url)
    }

    pub async fn atualizar_produto_imagem(
        &self,
        produto_uuid: Uuid,
        imagem_url: String,
    ) -> Result<Produto, String> {
        let mut produto = self.produto_repo.buscar_por_uuid(produto_uuid).await
            .map_err(|e| format!("Erro ao buscar produto: {}", e))?
            .ok_or("Produto não encontrado".to_string())?;

        produto.imagem_url = Some(imagem_url);
        self.produto_repo.atualizar(produto.clone()).await?;
        Ok(produto)
    }
}
