use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::{
    entities::app_errors::AppError,
    view_models::recon_task_response_details::ReconTaskResponseDetails,
};

#[automock]
#[async_trait]
pub trait ReconTasksDetailsRetrieverInterface: Send + Sync {
    async fn get_recon_task_details(
        &self,
        task_id: &String,
    ) -> Result<ReconTaskResponseDetails, AppError>;
}
