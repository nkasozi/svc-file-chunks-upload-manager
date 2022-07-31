use async_trait::async_trait;
use mockall::automock;

use crate::internal::{
    models::view_models::responses::svc_task_details_repo_responses::ReconFileMetaData,
    shared_reconciler_rust_libraries::models::entities::app_errors::AppError,
};

#[automock]
#[async_trait]
pub trait ReconTasksRepositoryInterface: Send + Sync {
    async fn get_recon_task_details(&self, task_id: &String)
        -> Result<ReconFileMetaData, AppError>;
}
