use aws_sdk_s3::{config::Credentials, primitives::ByteStream};
use uuid::Uuid;

use crate::{
    repositories::{ProdutoRepository, Repository as _},
    models::Produto,
};

pub struct UploadImagemUsecase {
    produto_repo: ProdutoRepository,
    bucket: String,
    endpoint: String,
}

impl UploadImagemUsecase {
    pub fn new(
        produto_repo: ProdutoRepository,
        bucket: String,
        endpoint: String,
    ) -> Self {
        Self {
            produto_repo,
            bucket,
            endpoint,
        }
    }

    async fn s3_client(&self) -> aws_sdk_s3::Client {
        let access_key = std::env::var("S3_ACCESS_KEY_ID").ok();
        let secret_key = std::env::var("S3_SECRET_ACCESS_KEY").ok();
        let credentials_provider = Credentials::new(
            access_key.expect("Var env não encontrada: S3_ACCESS_KEY_ID"),
            secret_key.expect("Var env não encontrada: S3_SECRET_ACCESS_KEY"),
            None,
            None,
            "static"
        );

  
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .credentials_provider(credentials_provider)
            .endpoint_url(self.endpoint.clone());

        aws_sdk_s3::Client::new(&config.load().await)

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
        let file_url =  {
            format!("{}/{}/{}", self.endpoint.trim_end_matches('/'), self.bucket, object_key)
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
