use async_trait::async_trait;
use validator::Validate;

use crate::internal::{
    interfaces::{
        file_chunk_upload_service::FileChunkUploadServiceInterface,
        pubsub_repo::PubSubRepositoryInterface,
        recon_tasks_repo::ReconTasksDetailsRetrieverInterface, transformer::TransformerInterface,
    },
    models::view_models::{
        requests::upload_file_chunk_request::UploadFileChunkRequest,
        responses::upload_file_chunk_response::UploadFileChunkResponse,
    },
};
use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::{AppError, AppErrorKind},
    file_upload_chunk::FileUploadChunkSource,
};

pub struct FileChunkUploadService {
    pub file_upload_repo: Box<dyn PubSubRepositoryInterface>,
    pub recon_tasks_retriever: Box<dyn ReconTasksDetailsRetrieverInterface>,
    pub to_entity_transformer: Box<dyn TransformerInterface>,
}

#[async_trait]
impl FileChunkUploadServiceInterface for FileChunkUploadService {
    /**
    uploads a file chunk to the repository

    # Errors

    This function will return an error if the request fails validation or fails to be uploaded.
     */
    async fn upload_file_chunk(
        &self,
        upload_file_chunk_request: UploadFileChunkRequest,
    ) -> Result<UploadFileChunkResponse, AppError> {
        //validate request
        match upload_file_chunk_request.validate() {
            Ok(_) => (),
            Err(e) => {
                return Err(AppError::new(
                    AppErrorKind::BadClientRequest,
                    e.to_string().replace("\n", " , "),
                ));
            }
        }

        //get recon file metadata
        let recon_task_details = self
            .recon_tasks_retriever
            .get_recon_task_details(&upload_file_chunk_request.upload_request_id)
            .await?;

        //transform into the repo model
        let file_upload_chunk = self
            .to_entity_transformer
            .transform_into_file_upload_chunk(upload_file_chunk_request, recon_task_details)?;

        let file_save_result;

        //save it to the repository
        match file_upload_chunk.chunk_source {
            FileUploadChunkSource::ComparisonFileChunk => {
                file_save_result = self
                    .file_upload_repo
                    .save_file_upload_chunk_to_comparison_file_queue(&file_upload_chunk)
                    .await;
            }

            FileUploadChunkSource::PrimaryFileChunk => {
                file_save_result = self
                    .file_upload_repo
                    .save_file_upload_chunk_to_primary_file_queue(&file_upload_chunk)
                    .await;
            }
        }

        match file_save_result {
            Ok(_) => Ok(UploadFileChunkResponse {
                file_chunk_id: file_upload_chunk.id,
            }),
            Err(e) => Err(e),
        }
    }
}
