use serde::{Deserialize, Serialize};

use super::recon_task::{ComparisonPair, ReconciliationConfigs};

//represents a group of lines inside a file
#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct FileUploadChunk {
    pub id: String,
    pub upload_request_id: String,
    pub chunk_sequence_number: i64,
    pub chunk_source: FileUploadChunkSource,
    pub chunk_rows: Vec<FileUploadChunkRow>,
    pub date_created: i64,
    pub date_modified: i64,
    pub comparison_pairs: Vec<ComparisonPair>,
    pub recon_config: ReconciliationConfigs,
}

//represents a line in a file
#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct FileUploadChunkRow {
    pub raw_data: String,
    pub parsed_columns_from_row: Vec<String>,
    pub recon_result: ReconStatus,
    pub recon_result_reasons: Vec<String>,
}

//represents the source of a file chunk
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ReconStatus {
    Failed,
    Successful,
    Pending,
}

//represents the source of a file chunk
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FileUploadChunkSource {
    ComparisonFileChunk,
    PrimaryFileChunk,
}
