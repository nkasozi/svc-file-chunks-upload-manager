use async_trait::async_trait;
use std::borrow::BorrowMut;
use uuid::Uuid;
use validator::Validate;

use crate::internal::{
    interfaces::{
        file_chunk_upload_service::FileChunkUploadServiceInterface,
        pubsub_repo::PubSubRepositoryInterface,
        recon_tasks_repo::ReconTasksDetailsRetrieverInterface,
    },
    models::view_models::{
        requests::upload_file_chunk_request::UploadFileChunkRequest,
        responses::{
            svc_task_details_repo_responses::ReconTaskResponseDetails,
            upload_file_chunk_response::UploadFileChunkResponse,
        },
    },
    shared_reconciler_rust_libraries::models::entities::recon_tasks_models::{
        ComparisonPair, ReconFileMetaData,
    },
};

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::{AppError, AppErrorKind},
    file_upload_chunk::{FileUploadChunk, FileUploadChunkRow, FileUploadChunkSource, ReconStatus},
};

const FILE_CHUNK_PREFIX: &'static str = "FILE-CHUNK";

pub struct FileChunkUploadService {
    pub file_upload_repo: Box<dyn PubSubRepositoryInterface>,
    pub recon_tasks_repo: Box<dyn ReconTasksDetailsRetrieverInterface>,
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
            .recon_tasks_repo
            .get_recon_task_details(&upload_file_chunk_request.upload_request_id)
            .await?;

        //transform into the repo model
        let file_upload_chunk =
            self.transform_into_file_upload_chunk(upload_file_chunk_request, recon_task_details);

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

impl FileChunkUploadService {
    fn transform_into_file_upload_chunk(
        &self,
        upload_file_chunk_request: UploadFileChunkRequest,
        recon_task_details: ReconTaskResponseDetails,
    ) -> FileUploadChunk {
        FileUploadChunk {
            id: self.generate_uuid(FILE_CHUNK_PREFIX),
            upload_request_id: upload_file_chunk_request.upload_request_id.clone(),
            chunk_sequence_number: upload_file_chunk_request.chunk_sequence_number.clone(),
            chunk_source: upload_file_chunk_request.chunk_source.clone(),
            chunk_rows: self.transform_into_chunk_rows(
                &mut upload_file_chunk_request.clone(),
                &mut recon_task_details.comparison_file_metadata.clone(),
                recon_task_details.task_details.comparison_pairs.clone(),
            ),
            date_created: chrono::Utc::now().timestamp(),
            date_modified: chrono::Utc::now().timestamp(),
            comparison_pairs: recon_task_details.task_details.comparison_pairs.clone(),
            recon_config: recon_task_details.task_details.recon_config.clone(),
        }
    }

    fn transform_into_chunk_rows(
        &self,
        upload_file_chunk_request: &mut UploadFileChunkRequest,
        recon_file_meta_data: &mut ReconFileMetaData,
        comparison_pairs: Vec<ComparisonPair>,
    ) -> Vec<FileUploadChunkRow> {
        let mut parsed_chunk_rows: Vec<FileUploadChunkRow> = vec![];

        let mut row_index = 1;

        for row_in_upload_file_chunk in &mut upload_file_chunk_request.chunk_rows {
            let columns_in_row_from_upload_file_chunk = break_up_file_row_using_delimiters(
                recon_file_meta_data,
                row_in_upload_file_chunk.borrow_mut(),
            );

            let parsed_chunk_row = parse_colum_values_from_row(
                upload_file_chunk_request.chunk_source,
                columns_in_row_from_upload_file_chunk,
                row_in_upload_file_chunk.clone(),
                row_index,
                comparison_pairs.clone(),
            );

            parsed_chunk_rows.push(parsed_chunk_row);

            row_index = row_index + 1;
        }

        return parsed_chunk_rows;
    }

    fn generate_uuid(&self, prefix: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let full_id = String::from(format!("{}-{}", prefix, id));
        return full_id;
    }
}

fn parse_colum_values_from_row(
    chunk_source: FileUploadChunkSource,
    upload_file_columns_in_row: Vec<String>,
    upload_file_row: String,
    row_index: i32,
    comparison_pairs: Vec<ComparisonPair>,
) -> FileUploadChunkRow {
    //set up the parsed row in a pending state
    let mut parsed_chunk_row = FileUploadChunkRow {
        raw_data: upload_file_row.to_string(),
        parsed_columns_from_row: vec![],
        recon_result: ReconStatus::Pending,
        recon_result_reasons: vec![],
    };

    for comparison_pair in comparison_pairs {
        match chunk_source {
            FileUploadChunkSource::ComparisonFileChunk => {
                if comparison_pair.comparison_column_index > upload_file_columns_in_row.len() {
                    //skip this row because the columns we have parsed are not enough
                    let reason = format!(
                        "cant find a value in column {} of comparison file for this row {}",
                        comparison_pair.comparison_column_index, row_index
                    );
                    parsed_chunk_row.recon_result = ReconStatus::Failed;
                    parsed_chunk_row.recon_result_reasons.push(reason);
                    continue;
                }

                //otherwise add new row to those that have been parsed
                let row_column_value = upload_file_columns_in_row
                    .get(comparison_pair.comparison_column_index)
                    .unwrap();

                parsed_chunk_row
                    .parsed_columns_from_row
                    .push(row_column_value.clone());

                continue;
            }

            FileUploadChunkSource::PrimaryFileChunk => {
                if comparison_pair.source_column_index > upload_file_columns_in_row.len() {
                    //skip this row because the columns we have parsed are not enough
                    let reason = format!(
                        "cant find a value in column {} of source file for this row {}",
                        comparison_pair.source_column_index, row_index
                    );
                    parsed_chunk_row.recon_result = ReconStatus::Failed;
                    parsed_chunk_row.recon_result_reasons.push(reason);
                    continue;
                }

                //otherwise add new row column value to those that have been parsed
                let row_column_value = upload_file_columns_in_row
                    .get(comparison_pair.source_column_index)
                    .unwrap();

                parsed_chunk_row
                    .parsed_columns_from_row
                    .push(row_column_value.clone());

                continue;
            }
        }
    }

    return parsed_chunk_row;
}

fn break_up_file_row_using_delimiters(
    recon_file_meta_data: &mut ReconFileMetaData,
    upload_file_row: &mut String,
) -> Vec<String> {
    let mut upload_file_columns_in_row: Vec<String> = vec![];
    for delimiter in recon_file_meta_data.column_delimiters.clone() {
        let row_parts: Vec<String> = upload_file_row
            .split(&delimiter)
            .map(str::to_owned)
            .collect();

        upload_file_columns_in_row.extend(row_parts);
    }
    upload_file_columns_in_row
}

#[cfg(test)]
mod tests {
    use crate::internal::{
        interfaces::{
            file_chunk_upload_service::FileChunkUploadServiceInterface,
            pubsub_repo::{MockPubSubRepositoryInterface, PubSubRepositoryInterface},
            recon_tasks_repo::{
                MockReconTasksDetailsRetrieverInterface, ReconTasksDetailsRetrieverInterface,
            },
        },
        models::view_models::{
            requests::upload_file_chunk_request::UploadFileChunkRequest,
            responses::svc_task_details_repo_responses::ReconTaskResponseDetails,
        },
        shared_reconciler_rust_libraries::models::entities::{
            app_errors::{AppError, AppErrorKind},
            file_upload_chunk::FileUploadChunkSource,
            recon_tasks_models::{
                ComparisonPair, ReconFileMetaData, ReconFileType, ReconTaskDetails,
                ReconciliationConfigs,
            },
        },
    };

    use super::FileChunkUploadService;

    fn setup() -> (
        Box<MockPubSubRepositoryInterface>,
        Box<MockReconTasksDetailsRetrieverInterface>,
    ) {
        let mock_file_upload_repo = Box::new(MockPubSubRepositoryInterface::new());
        let mock_recon_tasks_repo = Box::new(MockReconTasksDetailsRetrieverInterface::new());
        return (mock_file_upload_repo, mock_recon_tasks_repo);
    }

    fn dummy_success_recon_task_details() -> ReconTaskResponseDetails {
        ReconTaskResponseDetails {
            task_id: String::from("task-1234"),
            task_details: ReconTaskDetails {
                id: String::from("task-1234"),
                source_file_id: String::from("src-file-1234"),
                comparison_file_id: String::from("cmp-file-1234"),
                is_done: false,
                has_begun: true,
                comparison_pairs: vec![new_same_column_index_comparison_pair(0)],
                recon_config: default_recon_configs(),
            },
            source_file_metadata: ReconFileMetaData {
                id: String::from("src-file-1234"),
                file_name: String::from("src-file-1234"),
                row_count: 1000,
                column_delimiters: vec![],
                recon_file_type: ReconFileType::SourceReconFile,
                column_headers: vec![String::from("header1"), String::from("header2")],
                file_hash: String::from("src-file-1234"),
            },
            comparison_file_metadata: ReconFileMetaData {
                id: String::from("cmp-file-1234"),
                file_name: String::from("cmp-file-1234"),
                row_count: 1000,
                column_delimiters: vec![String::from(",")],
                recon_file_type: ReconFileType::ComparisonReconFile,
                column_headers: vec![String::from("header1"), String::from("header2")],
                file_hash: String::from("cmp-file-1234"),
            },
        }
    }

    fn dummy_valid_test_request() -> UploadFileChunkRequest {
        UploadFileChunkRequest {
            upload_request_id: String::from("1234"),
            chunk_sequence_number: 2,
            chunk_source: FileUploadChunkSource::ComparisonFileChunk,
            chunk_rows: vec![String::from("testing, 1234")],
        }
    }

    fn setup_service_under_test(
        pubsub: Box<dyn PubSubRepositoryInterface>,
        recon_tasks_repo: Box<dyn ReconTasksDetailsRetrieverInterface>,
    ) -> FileChunkUploadService {
        FileChunkUploadService {
            file_upload_repo: pubsub,
            recon_tasks_repo: recon_tasks_repo,
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
            source_column_index: column_index,
            comparison_column_index: column_index,
            is_row_identifier: true,
        }
    }

    #[actix_rt::test]
    async fn given_valid_request_calls_correct_dependencie_and_returns_success() {
        let (mut mock_file_upload_repo, mut mock_recon_tasks_repo) = setup();

        mock_recon_tasks_repo
            .expect_get_recon_task_details()
            .returning(|_y| Ok(dummy_success_recon_task_details()));

        mock_file_upload_repo
            .expect_save_file_upload_chunk_to_comparison_file_queue()
            .returning(|_y| Ok(String::from("FILE_CHUNK_1234")));

        let sut = setup_service_under_test(mock_file_upload_repo, mock_recon_tasks_repo);

        let test_request = dummy_valid_test_request();

        let actual = sut.upload_file_chunk(test_request).await;

        assert!(actual.is_ok());
    }

    #[actix_rt::test]
    async fn given_invalid_request_returns_error() {
        let (mut mock_file_upload_repo, mut mock_recon_tasks_repo) = setup();

        mock_recon_tasks_repo
            .expect_get_recon_task_details()
            .returning(|_y| Ok(dummy_success_recon_task_details()));

        mock_file_upload_repo
            .expect_save_file_upload_chunk_to_comparison_file_queue()
            .returning(|_y| Ok(String::from("FILE_CHUNK_1234")));

        let sut = setup_service_under_test(mock_file_upload_repo, mock_recon_tasks_repo);

        let mut test_request = dummy_valid_test_request();
        test_request.chunk_sequence_number = 0;

        let actual = sut.upload_file_chunk(test_request).await;

        assert!(actual.is_err());
    }

    #[actix_rt::test]
    async fn given_valid_request_but_repo_returns_error_returns_error() {
        let (mut mock_file_upload_repo, mut mock_recon_tasks_repo) = setup();

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

        let sut = setup_service_under_test(mock_file_upload_repo, mock_recon_tasks_repo);

        let test_request = dummy_valid_test_request();

        let actual = sut.upload_file_chunk(test_request).await;

        assert!(actual.is_err());
    }
}
