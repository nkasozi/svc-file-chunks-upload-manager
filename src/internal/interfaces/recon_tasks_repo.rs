use crate::internal::models::{
    entities::app_error::AppError, entities::recon_task::ReconFileMetaData,
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait ReconTasksRepositoryInterface: Send + Sync {
    async fn get_recon_task_details(&self, task_id: &String)
        -> Result<ReconFileMetaData, AppError>;
}
