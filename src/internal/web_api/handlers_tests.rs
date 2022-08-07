use actix_web::{
    test::{self, TestRequest},
    web::Data,
    App,
};

use crate::internal::{
    interfaces::file_chunk_upload_service::{
        FileChunkUploadServiceInterface, MockFileChunkUploadServiceInterface,
    },
    models::view_models::{
        requests::upload_file_chunk_request::UploadFileChunkRequest,
        responses::upload_file_chunk_response::UploadFileChunkResponse,
    },
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        file_upload_chunk::FileUploadChunkSource,
    },
    web_api::handlers::upload_file_chunk,
};

#[actix_web::test]
async fn test_upload_file_chunk_calls_correct_dependecies_and_returns_success() {
    let mut app = test::init_service((move || {
        // Create some global state prior to running the handler thread
        let mut mock_service = Box::new(MockFileChunkUploadServiceInterface::new());

        mock_service.expect_upload_file_chunk().returning(|_y| {
            Ok(UploadFileChunkResponse {
                file_chunk_id: String::from("FILE-CHUNK-1"),
            })
        });

        let service: Box<dyn FileChunkUploadServiceInterface> = mock_service;

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(upload_file_chunk)
    })())
    .await;

    let request = get_dummy_request();

    let resp = TestRequest::post()
        .uri(&format!("/upload-file-chunk"))
        .set_json(request)
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_upload_file_chunk_when_invalid_request_returns_bad_request() {
    let mut app = test::init_service((move || {
        // Create some global state prior to running the handler thread
        let mut mock_service = Box::new(MockFileChunkUploadServiceInterface::new());

        mock_service.expect_upload_file_chunk().returning(|_y| {
            Err(AppError::new(
                AppErrorKind::BadClientRequest,
                "invalid request".to_string(),
            ))
        });

        let service: Box<dyn FileChunkUploadServiceInterface> = mock_service;

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(upload_file_chunk)
    })())
    .await;

    let request = get_dummy_request();

    let resp = TestRequest::post()
        .uri(&format!("/upload-file-chunk"))
        .set_json(request)
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_upload_file_chunk_when_service_returns_error_returns_internal_error() {
    let mut app = test::init_service((move || {
        // Create some global state prior to running the handler thread
        let mut mock_service = Box::new(MockFileChunkUploadServiceInterface::new());

        mock_service.expect_upload_file_chunk().returning(|_y| {
            Err(AppError::new(
                AppErrorKind::InternalError,
                "Internal server error".to_string(),
            ))
        });
        let service: Box<dyn FileChunkUploadServiceInterface> = mock_service;

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(upload_file_chunk)
    })())
    .await;

    let request = get_dummy_request();

    let resp = TestRequest::post()
        .uri(&format!("/upload-file-chunk"))
        .set_json(request)
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_server_error());
}

fn get_dummy_request() -> UploadFileChunkRequest {
    UploadFileChunkRequest {
        upload_request_id: String::from("TEST-UPLOAD-1"),
        chunk_sequence_number: 1,
        chunk_source: FileUploadChunkSource::ComparisonFileChunk,
        chunk_rows: vec![],
    }
}
