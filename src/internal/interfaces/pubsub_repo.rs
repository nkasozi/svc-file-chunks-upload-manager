use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError, file_upload_chunk::FileUploadChunk,
};

#[automock]
#[async_trait]
pub trait PubSubRepositoryInterface: Send + Sync {
    async fn save_file_upload_chunk_to_primary_file_queue(
        &self,
        file_upload_chunk: &FileUploadChunk,
    ) -> Result<String, AppError>;

    async fn save_file_upload_chunk_to_comparison_file_queue(
        &self,
        file_upload_chunk: &FileUploadChunk,
    ) -> Result<String, AppError>;
}
