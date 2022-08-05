use crate::internal::{
    interfaces::to_entity_transformer::ToEntityTransfomerInterface,
    models::view_models::{
        requests::upload_file_chunk_request::UploadFileChunkRequest,
        responses::svc_task_details_repo_responses::ReconTaskResponseDetails,
    },
    shared_reconciler_rust_libraries::models::entities::{
        file_upload_chunk::FileUploadChunkSource,
        recon_tasks_models::{
            ReconFileMetaData, ReconFileType, ReconTaskDetails, ReconciliationConfigs,
        },
    },
};

use super::to_entity_transfomer::ToEntityTransfomer;

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

fn setup() -> ToEntityTransfomer {
    let to_entity_transformer = ToEntityTransfomer {};
    return to_entity_transformer;
}

fn get_dummy_upload_file_chunk_request() -> UploadFileChunkRequest {
    UploadFileChunkRequest {
        upload_request_id: String::from("TEST-UPLOAD-1"),
        chunk_sequence_number: 1,
        chunk_source: FileUploadChunkSource::ComparisonFileChunk,
        chunk_rows: vec![],
    }
}

fn get_dummy_recon_task_details() -> ReconTaskResponseDetails {
    ReconTaskResponseDetails {
        task_id: String::from("TEST-UPLOAD-1"),
        task_details: ReconTaskDetails {
            id: String::from("task-1234"),
            source_file_id: String::from("src-file-1234"),
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
        },
        source_file_metadata: ReconFileMetaData {
            id: String::from("src-file-1234"),
            file_name: String::from("src-file-1234"),
            row_count: 1000,
            column_delimiters: vec![],
            recon_file_type: ReconFileType::SourceReconFile,
            column_headers: vec![],
            file_hash: String::from("src-file-1234"),
        },
        comparison_file_metadata: ReconFileMetaData {
            id: String::from("cmp-file-1234"),
            file_name: String::from("cmp-file-1234"),
            row_count: 1000,
            column_delimiters: vec![],
            recon_file_type: ReconFileType::ComparisonReconFile,
            column_headers: vec![],
            file_hash: String::from("cmp-file-1234"),
        },
    }
}
