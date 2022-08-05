use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};

use crate::internal::{
    interfaces::file_chunk_upload_service::FileChunkUploadServiceInterface,
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::entities::app_errors::AppErrorKind,
};

#[post("/upload-file-chunk")]
async fn upload_file_chunk(
    task_details: web::Json<UploadFileChunkRequest>,
    service: Data<Box<dyn FileChunkUploadServiceInterface>>,
) -> HttpResponse {
    let recon_task_details = service.upload_file_chunk(task_details.0).await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),

        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}
