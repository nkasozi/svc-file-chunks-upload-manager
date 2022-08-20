use crate::internal::{
    interfaces::pubsub_repo::PubSubRepositoryInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        file_upload_chunk::FileUploadChunk,
    },
};
use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use std::collections::HashMap;
use tonic::transport::Channel as TonicChannel;

pub struct DaprPubSubRepositoryManager {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr pub sub component name
    pub dapr_pubsub_name: String,

    //the dapr pub sub topic for the primary file chunks
    pub dapr_pubsub_primary_file_topic: String,

    //the dapr pub sub topic for the comparison file chunks
    pub dapr_pubsub_comparison_file_topic: String,
}

#[async_trait]
impl PubSubRepositoryInterface for DaprPubSubRepositoryManager {
    async fn save_file_upload_chunk_to_primary_file_queue(
        &self,
        file_upload_chunk: &FileUploadChunk,
    ) -> Result<String, AppError> {
        //create a dapr client
        let mut client = self.get_dapr_connection().await?;

        //call the binding
        let pubsub_name = self.dapr_pubsub_name.clone();
        let pubsub_topic = file_upload_chunk.primary_file_chunks_queue.topic_id.clone();
        let data_content_type = "json".to_string();
        let data = serde_json::to_vec(&file_upload_chunk).unwrap();
        let metadata = None::<HashMap<String, String>>;
        let binding_response = client
            .publish_event(pubsub_name, pubsub_topic, data_content_type, data, metadata)
            .await;

        //handle the bindings response
        match binding_response {
            //success
            Ok(_) => Ok("".to_owned()),
            //failure
            Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        }
    }

    async fn save_file_upload_chunk_to_comparison_file_queue(
        &self,
        file_upload_chunk: &FileUploadChunk,
    ) -> Result<String, AppError> {
        //create a dapr client
        let mut client = self.get_dapr_connection().await?;

        //call the binding
        let pubsub_name = self.dapr_pubsub_name.clone();
        let pubsub_topic = file_upload_chunk
            .comparison_file_chunks_queue
            .topic_id
            .clone();
        let data_content_type = "json".to_string();
        let data = serde_json::to_vec(&file_upload_chunk).unwrap();
        let metadata = None::<HashMap<String, String>>;
        let binding_response = client
            .publish_event(pubsub_name, pubsub_topic, data_content_type, data, metadata)
            .await;

        //handle the bindings response
        match binding_response {
            //success
            Ok(_) => Ok("".to_owned()),
            //failure
            Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        }
    }
}

impl DaprPubSubRepositoryManager {
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
