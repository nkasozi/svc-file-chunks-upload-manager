use actix_web::{
    HttpResponse,
    post,
    web::{self, Data},
};

use crate::internal::{
    interfaces::file_chunk_upload_service::FileChunkUploadServiceInterface,
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
};
use crate::internal::shared_reconciler_rust_libraries::web_api::utils::ok_or_error;

#[post("/upload-file-chunk")]
pub(crate) async fn upload_file_chunk(
    task_details: web::Json<UploadFileChunkRequest>,
    service: Data<Box<dyn FileChunkUploadServiceInterface>>,
) -> HttpResponse {
    let recon_task_details = service.upload_file_chunk(task_details.0).await;

    return ok_or_error(recon_task_details);
}
