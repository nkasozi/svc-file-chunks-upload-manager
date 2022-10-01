use async_trait::async_trait;

use crate::internal::{
    interfaces::recon_tasks_repo::ReconTasksDetailsRetrieverInterface,
    shared_reconciler_rust_libraries::models::{
        entities::app_errors::AppError,
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::interfaces::recon_tasks_microservice::ReconTasksMicroserviceClientInterface;

pub struct ReconTasksServiceConnector {
    recon_tasks_microservice_client: Box<dyn ReconTasksMicroserviceClientInterface>,
}

#[async_trait]
impl ReconTasksDetailsRetrieverInterface for ReconTasksServiceConnector {
    async fn get_recon_task_details(
        &self,
        task_id: &String,
    ) -> Result<ReconTaskResponseDetails, AppError> {
        let result = self.recon_tasks_microservice_client.get_recon_task(task_id).await;
        return result;
    }
}

impl ReconTasksServiceConnector {
    pub fn new(ms_client: Box<dyn ReconTasksMicroserviceClientInterface>) -> ReconTasksServiceConnector {
        return ReconTasksServiceConnector {
            recon_tasks_microservice_client: ms_client
        };
    }
}
