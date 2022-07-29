use crate::internal::models::{
    entities::app_error::AppError,
    view_models::{
        upload_file_chunk_request::UploadFileChunkRequest,
        upload_file_chunk_response::UploadFileChunkResponse,
    },
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
