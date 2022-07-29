use crate::internal::{
    interfaces::recon_tasks_repo::ReconTasksRepositoryInterface,
    models::entities::{
        app_error::AppError, app_error::AppErrorKind, recon_task::ReconFileMetaData,
    },
};
use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use tonic::transport::Channel as TonicChannel;

pub struct ReconTasksRepositoryManager {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub recon_tasks_service_name: String,
}

#[async_trait]
impl ReconTasksRepositoryInterface for ReconTasksRepositoryManager {
    async fn get_recon_task_details(
        &self,
        task_id: &String,
    ) -> Result<ReconFileMetaData, AppError> {
        //create a dapr client
        let mut client = self.get_dapr_connection().await?;

        //call the binding
        let url = format!("/get-recon-file-metadata/{}", task_id.clone());
        let binding_response = client
            .invoke_service(self.recon_tasks_service_name.clone(), url, Option::None)
            .await;

        //handle the bindings response
        match binding_response {
            //successs
            Ok(succesful_response) => {
                let response_data = succesful_response.data.unwrap().value;
                let unmarshal_result: Result<ReconFileMetaData, _> =
                    serde_json::from_slice(&response_data);

                match unmarshal_result {
                    Ok(file_metadata) => return Ok(file_metadata),
                    Err(e) => {
                        return Err(AppError::new(
                            AppErrorKind::ServiceResponseError,
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

impl ReconTasksRepositoryManager {
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
