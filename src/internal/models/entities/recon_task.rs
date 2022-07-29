use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct ReconTaskDetails {
    pub id: String,
    pub source_file_id: String,
    pub comparison_file_id: String,
    pub is_done: bool,
    pub has_begun: bool,
    pub comparison_pairs: Vec<ComparisonPair>,
    pub recon_config: ReconciliationConfigs,
}

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct ReconFileMetaData {
    pub id: String,
    pub file_name: String,
    pub row_count: u64,
    pub column_delimiters: Vec<String>,
    pub recon_file_type: ReconFileType,
    pub column_headers: Vec<String>,
    pub file_hash: String,
    pub recon_task_details: ReconTaskDetails,
}

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct ComparisonPair {
    pub source_column_index: usize,
    pub comparison_column_index: usize,
    pub is_row_identifier: bool,
}

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct ReconciliationConfigs {
    pub should_check_for_duplicate_records_in_comparison_file: bool,
    pub should_reconciliation_be_case_sensitive: bool,
    pub should_ignore_white_space: bool,
    pub should_do_reverse_reconciliation: bool,
}

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub enum ReconFileType {
    SourceReconFile,
    ComparisonReconFile,
}
