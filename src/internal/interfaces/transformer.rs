use mockall::automock;

use crate::internal::{
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::{
        entities::file_upload_chunk::FileUploadChunk,
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};

#[automock]
pub trait TransformerInterface: Send + Sync {
    fn transform_into_file_upload_chunk(
        &self,
        upload_file_chunk_request: UploadFileChunkRequest,
        recon_task_details: ReconTaskResponseDetails,
    ) -> FileUploadChunk;
}
