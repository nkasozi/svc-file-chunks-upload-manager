use mockall::automock;

use crate::internal::{
    models::view_models::{
        requests::upload_file_chunk_request::UploadFileChunkRequest,
        responses::svc_task_details_repo_responses::ReconTaskResponseDetails,
    },
    shared_reconciler_rust_libraries::models::entities::file_upload_chunk::FileUploadChunk,
};

#[automock]
pub trait TransformerInterface: Send + Sync {
    fn transform_into_file_upload_chunk(
        &self,
        upload_file_chunk_request: UploadFileChunkRequest,
        recon_task_details: ReconTaskResponseDetails,
    ) -> FileUploadChunk;
}
