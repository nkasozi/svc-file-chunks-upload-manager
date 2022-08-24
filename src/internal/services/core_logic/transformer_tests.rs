use crate::internal::{
    interfaces::transformer::TransformerInterface,
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::{
        entities::{
            file_chunk_queue::FileChunkQueue,
            file_upload_chunk::FileUploadChunkSource,
            recon_tasks_models::{
                ReconFileMetaData, ReconFileType, ReconTaskDetails, ReconciliationConfigs,
            },
        },
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};

use super::transformer::Transformer;

#[actix_web::test]
async fn test_transform_into_file_upload_chunk_returns_correct_model() {
    let to_entity_transformer = setup();

    let upload_file_chunk_request = get_dummy_upload_file_chunk_request();
    let recon_task_details = get_dummy_recon_task_details();

    let actual = to_entity_transformer.transform_into_file_upload_chunk(
        upload_file_chunk_request.clone(),
        recon_task_details.clone(),
    );

    assert_eq!(
        actual.chunk_sequence_number,
        upload_file_chunk_request.chunk_sequence_number
    );
}

fn setup() -> Transformer {
    let to_entity_transformer = Transformer {};
    return to_entity_transformer;
}

fn get_dummy_upload_file_chunk_request() -> UploadFileChunkRequest {
    UploadFileChunkRequest {
        upload_request_id: String::from("TEST-UPLOAD-1"),
        chunk_sequence_number: 1,
        chunk_source: FileUploadChunkSource::ComparisonFileChunk,
        chunk_rows: vec![],
        is_last_chunk: false,
    }
}

fn get_dummy_recon_task_details() -> ReconTaskResponseDetails {
    ReconTaskResponseDetails {
        task_id: String::from("TEST-UPLOAD-1"),
        task_details: ReconTaskDetails {
            id: String::from("task-1234"),
            primary_file_id: String::from("src-file-1234"),
            comparison_file_id: String::from("cmp-file-1234"),
            is_done: false,
            has_begun: true,
            comparison_pairs: vec![],
            recon_config: ReconciliationConfigs {
                should_check_for_duplicate_records_in_comparison_file: true,
                should_reconciliation_be_case_sensitive: true,
                should_ignore_white_space: true,
                should_do_reverse_reconciliation: true,
            },
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
            column_headers: vec![],
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
            column_delimiters: vec![],
            recon_file_type: ReconFileType::ComparisonFile,
            column_headers: vec![],
            file_hash: String::from("cmp-file-1234"),
            queue_info: FileChunkQueue {
                topic_id: String::from("cmp-file-chunks-queue-1"),
                last_acknowledged_id: Option::None,
            },
        },
    }
}
