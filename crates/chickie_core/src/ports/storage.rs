use async_trait::async_trait;
use crate::domain::errors::DomainResult;

/// Port for image storage — abstracts away AWS S3 or any other provider.
#[async_trait]
pub trait ImageStoragePort: Send + Sync {
    /// Upload an image and return the URL
    async fn upload(&self, file_name: &str, bytes: &[u8], content_type: &str) -> DomainResult<String>;
}
