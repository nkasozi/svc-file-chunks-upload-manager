use serde::{Deserialize, Serialize};

use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::{
    ReconFileMetaData, ReconTaskDetails,
};

#[derive(PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct ReconTaskResponseDetails {
    pub task_id: String,
    pub task_details: ReconTaskDetails,
    pub source_file_metadata: ReconFileMetaData,
    pub comparison_file_metadata: ReconFileMetaData,
}
