use super::file_upload_service::FileChunkUploadService;

#[actix_rt::test]
async fn given_valid_request_calls_correct_dependencie_and_returns_success() {
    let (mut mock_file_upload_repo, mut mock_recon_tasks_repo, mut mock_to_entity_transformer) =
        setup_dependencies();

    mock_recon_tasks_repo
        .expect_get_recon_task_details()
        .returning(|_y| Ok(dummy_success_recon_task_details()));

    mock_file_upload_repo
        .expect_save_file_upload_chunk_to_comparison_file_queue()
        .returning(|_y| Ok(String::from("FILE_CHUNK_1234")));

    mock_to_entity_transformer
        .expect_transform_into_file_upload_chunk()
        .returning(|_, _| dummy_valid_file_chunk());

    let sut = setup_service_under_test(
        mock_file_upload_repo,
        mock_recon_tasks_repo,
        mock_to_entity_transformer,
    );

    let test_request = dummy_valid_test_request();

    let actual = sut.upload_file_chunk(test_request).await;

    assert!(actual.is_ok());
}

#[actix_rt::test]
async fn given_invalid_request_returns_error() {
    let (mut mock_file_upload_repo, mut mock_recon_tasks_repo, mut mock_to_entity_transformer) =
        setup_dependencies();

    mock_recon_tasks_repo
        .expect_get_recon_task_details()
        .returning(|_y| Ok(dummy_success_recon_task_details()));

    mock_file_upload_repo
        .expect_save_file_upload_chunk_to_comparison_file_queue()
        .returning(|_y| Ok(String::from("FILE_CHUNK_1234")));

    mock_to_entity_transformer
        .expect_transform_into_file_upload_chunk()
        .returning(|_, _| dummy_valid_file_chunk());

    let sut = setup_service_under_test(
        mock_file_upload_repo,
        mock_recon_tasks_repo,
        mock_to_entity_transformer,
    );

    let mut test_request = dummy_valid_test_request();
    test_request.chunk_sequence_number = 0;

    let actual = sut.upload_file_chunk(test_request).await;

    assert!(actual.is_err());
}

#[actix_rt::test]
async fn given_valid_request_but_repo_returns_error_returns_error() {
    let (mut mock_file_upload_repo, mut mock_recon_tasks_repo, mut mock_to_entity_transformer) =
        setup_dependencies();

    mock_recon_tasks_repo
        .expect_get_recon_task_details()
        .returning(|_y| Ok(dummy_success_recon_task_details()));

    mock_file_upload_repo
        .expect_save_file_upload_chunk_to_comparison_file_queue()
        .returning(|_y| {
            Err(AppError::new(
                AppErrorKind::ConnectionError,
                "unable to connect".to_string(),
            ))
        });

    mock_to_entity_transformer
        .expect_transform_into_file_upload_chunk()
        .returning(|_, _| dummy_valid_file_chunk());

    let sut = setup_service_under_test(
        mock_file_upload_repo,
        mock_recon_tasks_repo,
        mock_to_entity_transformer,
    );

    let test_request = dummy_valid_test_request();

    let actual = sut.upload_file_chunk(test_request).await;

    assert!(actual.is_err());
}

use crate::internal::{
    interfaces::{
        file_chunk_upload_service::FileChunkUploadServiceInterface,
        pubsub_repo::{MockPubSubRepositoryInterface, PubSubRepositoryInterface},
        recon_tasks_repo::{
            MockReconTasksDetailsRetrieverInterface, ReconTasksDetailsRetrieverInterface,
        },
        transformer::{MockTransformerInterface, TransformerInterface},
    },
    models::view_models::requests::upload_file_chunk_request::{FileRow, UploadFileChunkRequest},
    shared_reconciler_rust_libraries::models::{
        entities::{
            app_errors::{AppError, AppErrorKind},
            file_chunk_queue::FileChunkQueue,
            file_upload_chunk::{FileUploadChunk, FileUploadChunkSource},
            recon_tasks_models::{
                ComparisonPair, ReconFileMetaData, ReconFileType, ReconTaskDetails,
                ReconciliationConfigs,
            },
        },
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};

fn setup_dependencies() -> (
    Box<MockPubSubRepositoryInterface>,
    Box<MockReconTasksDetailsRetrieverInterface>,
    Box<MockTransformerInterface>,
) {
    let mock_file_upload_repo = Box::new(MockPubSubRepositoryInterface::new());
    let mock_recon_tasks_repo = Box::new(MockReconTasksDetailsRetrieverInterface::new());
    let mock_to_entity_transformer = Box::new(MockTransformerInterface::new());
    return (
        mock_file_upload_repo,
        mock_recon_tasks_repo,
        mock_to_entity_transformer,
    );
}

fn dummy_success_recon_task_details() -> ReconTaskResponseDetails {
    ReconTaskResponseDetails {
        task_id: String::from("task-1234"),
        task_details: ReconTaskDetails {
            id: String::from("task-1234"),
            primary_file_id: String::from("src-file-1234"),
            comparison_file_id: String::from("cmp-file-1234"),
            is_done: false,
            has_begun: true,
            comparison_pairs: vec![new_same_column_index_comparison_pair(0)],
            recon_config: default_recon_configs(),
            recon_results_queue_info: FileChunkQueue {
                topic_id: String::from("recon-results-queue-1"),
                last_acknowledged_id: Option::None,
            },
        },
        primary_file_metadata: ReconFileMetaData {
            id: String::from("src-file-1234"),
            file_name: String::from("src-file-1234"),
            row_count: 1000,
            column_delimiters: vec![],
            recon_file_type: ReconFileType::PrimaryFile,
            column_headers: vec![String::from("header1"), String::from("header2")],
            file_hash: String::from("src-file-1234"),
            queue_info: FileChunkQueue {
                topic_id: String::from("src-file-chunks-queue-1"),
                last_acknowledged_id: Option::None,
            },
        },
        comparison_file_metadata: ReconFileMetaData {
            id: String::from("cmp-file-1234"),
            file_name: String::from("cmp-file-1234"),
            row_count: 1000,
            column_delimiters: vec![String::from(",")],
            recon_file_type: ReconFileType::ComparisonFile,
            column_headers: vec![String::from("header1"), String::from("header2")],
            file_hash: String::from("cmp-file-1234"),
            queue_info: FileChunkQueue {
                topic_id: String::from("cmp-file-chunks-queue-1"),
                last_acknowledged_id: Option::None,
            },
        },
    }
}

fn dummy_valid_test_request() -> UploadFileChunkRequest {
    UploadFileChunkRequest {
        upload_request_id: String::from("1234"),
        chunk_sequence_number: 2,
        chunk_source: FileUploadChunkSource::ComparisonFileChunk,
        chunk_rows: vec![FileRow {
            raw_data: String::from("testing, 1234"),
            row_number: 1,
        }],
    }
}

fn dummy_valid_file_chunk() -> FileUploadChunk {
    FileUploadChunk {
        id: String::from("src-file-1234"),
        upload_request_id: String::from("file-1234"),
        chunk_sequence_number: 1,
        chunk_source: FileUploadChunkSource::ComparisonFileChunk,
        chunk_rows: vec![],
        date_created: chrono::Utc::now().timestamp(),
        date_modified: chrono::Utc::now().timestamp(),
        comparison_pairs: vec![new_same_column_index_comparison_pair(0)],
        column_headers: vec![],
        recon_config: default_recon_configs(),
        primary_file_chunks_queue: FileChunkQueue {
            topic_id: String::from("src-file-chunks-queue-1"),
            last_acknowledged_id: Option::None,
        },
        comparison_file_chunks_queue: FileChunkQueue {
            topic_id: String::from("cmp-file-chunks-queue-1"),
            last_acknowledged_id: Option::None,
        },
        result_chunks_queue: FileChunkQueue {
            topic_id: String::from("results-file-chunks-queue-1"),
            last_acknowledged_id: Option::None,
        },
    }
}

fn setup_service_under_test(
    pubsub: Box<dyn PubSubRepositoryInterface>,
    recon_tasks_repo: Box<dyn ReconTasksDetailsRetrieverInterface>,
    to_entity_transformer: Box<dyn TransformerInterface>,
) -> FileChunkUploadService {
    FileChunkUploadService {
        file_upload_repo: pubsub,
        recon_tasks_retriever: recon_tasks_repo,
        to_entity_transformer: to_entity_transformer,
    }
}

fn default_recon_configs() -> ReconciliationConfigs {
    ReconciliationConfigs {
        should_check_for_duplicate_records_in_comparison_file: true,
        should_reconciliation_be_case_sensitive: true,
        should_ignore_white_space: true,
        should_do_reverse_reconciliation: true,
    }
}

fn new_same_column_index_comparison_pair(column_index: usize) -> ComparisonPair {
    ComparisonPair {
        primary_file_column_index: column_index,
        comparison_file_column_index: column_index,
        is_row_identifier: true,
    }
}
