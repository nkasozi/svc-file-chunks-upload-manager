use std::collections::HashMap;

use async_trait::async_trait;
use dapr::{Client, dapr::dapr::proto::runtime::v1::dapr_client::DaprClient};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::pubsub_repo::PubSubRepositoryInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        file_upload_chunk::FileUploadChunk,
    },
};
use crate::internal::shared_reconciler_rust_libraries::common::utils::app_error;

const DATA_CONTENT_TYPE: &'static str = "json";

pub struct DaprPubSub {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr pub sub component name
    pub dapr_pubsub_name: String,
}

#[async_trait]
impl PubSubRepositoryInterface for DaprPubSub {
    async fn save_file_upload_chunk_to_primary_file_queue(
        &self,
        file_upload_chunk: &FileUploadChunk,
    ) -> Result<String, AppError> {
        //create a dapr client
        let mut client = self.get_dapr_connection().await?;

        //call the binding
        let pubsub_name = self.dapr_pubsub_name.clone();
        let pubsub_topic = file_upload_chunk.primary_file_chunks_queue.topic_id.clone();
        let data = serde_json::to_vec(&file_upload_chunk).unwrap();
        let metadata = None::<HashMap<String, String>>;
        let binding_response = client
            .publish_event(pubsub_name, pubsub_topic, DATA_CONTENT_TYPE.to_string(), data, metadata)
            .await;

        //handle the bindings response
        return match binding_response {
            //success
            Ok(_) => Ok(file_upload_chunk.clone().id),
            //failure
            Err(e) => app_error(AppErrorKind::InternalError, Box::new(e)),
        };
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
        let data = serde_json::to_vec(&file_upload_chunk).unwrap();
        let metadata = None::<HashMap<String, String>>;

        let binding_response = client
            .publish_event(pubsub_name, pubsub_topic, DATA_CONTENT_TYPE.to_string(), data, metadata)
            .await;

        //handle the bindings response
        return match binding_response {
            //success
            Ok(_) => Ok(file_upload_chunk.clone().id),
            //failure
            Err(e) => app_error(AppErrorKind::InternalError, Box::new(e)),
        };
    }
}

impl DaprPubSub {
    async fn get_dapr_connection(&self) -> Result<Client<DaprClient<TonicChannel>>, AppError> {
        // Create the client
        let dapr_grpc_server_address = self.dapr_grpc_server_address.clone();

        //connect to dapr
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(dapr_grpc_server_address).await;

        //handle the connection result
        return match client_connect_result {
            //connection succeeded
            Ok(s) => Ok(s),
            //connection failed
            Err(e) => Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        };
    }
}
