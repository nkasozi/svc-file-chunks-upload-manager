use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::recon_tasks_repo::ReconTasksDetailsRetrieverInterface,
    models::view_models::responses::svc_task_details_repo_responses::ReconTaskResponseDetails,
    shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind},
};

pub struct ReconTasksDetailsRetriever {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub recon_tasks_service_name: String,
}

#[async_trait]
impl ReconTasksDetailsRetrieverInterface for ReconTasksDetailsRetriever {
    async fn get_recon_task_details(
        &self,
        task_id: &String,
    ) -> Result<ReconTaskResponseDetails, AppError> {
        //create a dapr client
        let mut client = self.get_dapr_connection().await?;

        //call the binding
        let url = format!("/recon-task/{}", task_id.clone());
        let binding_response = client
            .invoke_service(self.recon_tasks_service_name.clone(), url, Option::None)
            .await;

        //handle the bindings response
        match binding_response {
            //successs
            Ok(succesful_response) => {
                let response_data = succesful_response.data.unwrap().value;
                let unmarshal_result: Result<ReconTaskResponseDetails, _> =
                    serde_json::from_slice(&response_data);

                match unmarshal_result {
                    Ok(file_metadata) => return Ok(file_metadata),
                    Err(e) => {
                        return Err(AppError::new(
                            AppErrorKind::ResponseUnmarshalError,
                            e.to_string(),
                        ))
                    }
                };
            }
            //failure
            Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        }
    }
}

impl ReconTasksDetailsRetriever {
    async fn get_dapr_connection(&self) -> Result<Client<DaprClient<TonicChannel>>, AppError> {
        // Create the client
        let dapr_grpc_server_address = self.dapr_grpc_server_address.clone();

        //connect to dapr
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(dapr_grpc_server_address).await;

        //handle the connection result
        match client_connect_result {
            //connection succeeded
            Ok(s) => return Ok(s),
            //connection failed
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }
    }
}
