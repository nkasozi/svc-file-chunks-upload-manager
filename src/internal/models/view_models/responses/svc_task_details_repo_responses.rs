use serde::{Deserialize, Serialize};

use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::{
    ReconFileType, ReconTaskDetails,
};

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
