use crate::internal::{
    models::view_models::{
        requests::upload_file_chunk_request::UploadFileChunkRequest,
        responses::upload_file_chunk_response::UploadFileChunkResponse,
    },
    shared_reconciler_rust_libraries::models::entities::app_errors::AppError,
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait FileChunkUploadServiceInterface: Send + Sync {
    async fn upload_file_chunk(
        &self,
        file_upload_chunk: UploadFileChunkRequest,
    ) -> Result<UploadFileChunkResponse, AppError>;
}
