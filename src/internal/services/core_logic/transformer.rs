use uuid::Uuid;

use crate::internal::{
    interfaces::transformer::TransformerInterface,
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::{
        entities::{
            file_upload_chunk::{
                FileUploadChunk, FileUploadChunkRow, FileUploadChunkSource, ReconStatus,
            },
            recon_tasks_models::{ComparisonPair, ReconFileMetaData},
        },
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};
use crate::internal::shared_reconciler_rust_libraries::common::utils::app_error_with_msg;
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind};
use crate::internal::shared_reconciler_rust_libraries::models::entities::file_chunk_queue::FileChunkQueue;

const FILE_CHUNK_PREFIX: &'static str = "FILE-CHUNK";

pub struct Transformer {}

impl TransformerInterface for Transformer {
    fn transform_into_file_upload_chunk(
        &self,
        upload_file_chunk_request: UploadFileChunkRequest,
        recon_task_details: ReconTaskResponseDetails,
    ) -> Result<FileUploadChunk, AppError> {
        Ok(FileUploadChunk {
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
            column_headers: Self::get_column_headers(upload_file_chunk_request.clone(), recon_task_details.clone()),
            primary_file_chunks_queue: Self::get_queue_info(&recon_task_details.primary_file_metadata.clone())?,
            comparison_file_chunks_queue: Self::get_queue_info(&recon_task_details.comparison_file_metadata.clone())?,
            result_chunks_queue: recon_task_details
                .task_details
                .recon_results_queue_info
                .clone(),
            is_last_chunk: upload_file_chunk_request.is_last_chunk.clone(),
        })
    }
}

impl Transformer {
    fn transform_into_chunk_rows(
        &self,
        upload_file_chunk_request: &mut UploadFileChunkRequest,
        recon_file_meta_data: &mut Option<ReconFileMetaData>,
        comparison_pairs: Vec<ComparisonPair>,
    ) -> Vec<FileUploadChunkRow> {
        let mut parsed_chunk_rows: Vec<FileUploadChunkRow> = vec![];

        for row_in_upload_file_chunk in &mut upload_file_chunk_request.chunk_rows {
            let columns_in_row_from_upload_file_chunk = break_up_file_row_using_delimiters(
                recon_file_meta_data,
                &mut row_in_upload_file_chunk.raw_data,
            );

            let parsed_chunk_row = parse_colum_values_from_row(
                upload_file_chunk_request.chunk_source,
                columns_in_row_from_upload_file_chunk,
                row_in_upload_file_chunk.raw_data.clone(),
                row_in_upload_file_chunk.row_number,
                comparison_pairs.clone(),
            );

            parsed_chunk_rows.push(parsed_chunk_row);
        }

        return parsed_chunk_rows;
    }

    fn generate_uuid(&self, prefix: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let full_id = String::from(format!("{}-{}", prefix, id));
        return full_id;
    }

    fn get_queue_info(file_metadata: &Option<ReconFileMetaData>) -> Result<FileChunkQueue, AppError> {
        return match file_metadata {
            None => {
                app_error_with_msg(AppErrorKind::InternalError, "no queue info found")
            }
            Some(metadata) => {
                Ok(metadata.queue_info
                    .clone())
            }
        };
    }

    fn get_column_headers(upload_file_chunk_request: UploadFileChunkRequest,
                          recon_task_details: ReconTaskResponseDetails) -> Vec<String> {
        return match upload_file_chunk_request.chunk_source {
            FileUploadChunkSource::ComparisonFileChunk => {
                Self::get_column_headers_from_metadata(recon_task_details.comparison_file_metadata)
            }
            FileUploadChunkSource::PrimaryFileChunk => {
                Self::get_column_headers_from_metadata(recon_task_details.primary_file_metadata)
            }
        };
    }

    fn get_column_headers_from_metadata(file_metadata: Option<ReconFileMetaData>) -> Vec<String> {
        return match file_metadata {
            None => {
                vec![]
            }
            Some(metadata) => {
                metadata.column_headers
                    .clone()
            }
        };
    }
}

fn parse_colum_values_from_row(
    chunk_source: FileUploadChunkSource,
    upload_file_columns_in_row: Vec<String>,
    upload_file_row: String,
    row_index: u64,
    comparison_pairs: Vec<ComparisonPair>,
) -> FileUploadChunkRow {
    //set up the parsed row in a pending state
    let mut parsed_chunk_row = FileUploadChunkRow {
        raw_data: upload_file_row.to_string(),
        parsed_columns_from_row: vec![],
        recon_result: ReconStatus::Pending,
        recon_result_reasons: vec![],
        row_number: row_index,
    };

    for comparison_pair in comparison_pairs {
        match chunk_source {
            FileUploadChunkSource::ComparisonFileChunk => {
                if comparison_pair.comparison_file_column_index > upload_file_columns_in_row.len() {
                    //skip this row because the columns we have parsed are not enough
                    let reason = format!(
                        "cant find a value in column {} of comparison file for this row {}",
                        comparison_pair.comparison_file_column_index, row_index
                    );
                    parsed_chunk_row.recon_result = ReconStatus::Failed;
                    parsed_chunk_row.recon_result_reasons.push(reason);
                    continue;
                }

                //otherwise add new row to those that have been parsed
                let row_column_value = upload_file_columns_in_row
                    .get(comparison_pair.comparison_file_column_index)
                    .unwrap();

                parsed_chunk_row
                    .parsed_columns_from_row
                    .push(row_column_value.clone());

                continue;
            }

            FileUploadChunkSource::PrimaryFileChunk => {
                if comparison_pair.primary_file_column_index > upload_file_columns_in_row.len() {
                    //skip this row because the columns we have parsed are not enough
                    let reason = format!(
                        "cant find a value in column {} of source file for this row {}",
                        comparison_pair.primary_file_column_index, row_index
                    );
                    parsed_chunk_row.recon_result = ReconStatus::Failed;
                    parsed_chunk_row.recon_result_reasons.push(reason);
                    continue;
                }

                //otherwise add new row column value to those that have been parsed
                let row_column_value = upload_file_columns_in_row
                    .get(comparison_pair.primary_file_column_index)
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
    recon_file_meta_data: &mut Option<ReconFileMetaData>,
    upload_file_row: &mut String,
) -> Vec<String> {
    let mut upload_file_columns_in_row: Vec<String> = vec![];
    match recon_file_meta_data {
        None => {}
        Some(metadata) => {
            let row_parts: Vec<String> = upload_file_row
                .split(&metadata.column_delimiters.clone()[..])
                .map(str::to_owned)
                .collect();

            upload_file_columns_in_row.extend(row_parts);
        }
    }

    upload_file_columns_in_row
}
