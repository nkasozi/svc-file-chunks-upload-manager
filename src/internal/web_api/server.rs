use actix_web::{App, HttpServer, web::Data};

use crate::{
    external::{
        connectors::recon_tasks_service_connector::ReconTasksServiceConnector,
        pubsub::dapr_pubsub::DaprPubSub,
    },
    internal::{
        interfaces::file_chunk_upload_service::FileChunkUploadServiceInterface,
        services::{
            core_logic::transformer::Transformer, file_upload_service::FileChunkUploadService,
        },
        web_api::handlers,
    },
};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::interfaces::recon_tasks_microservice::ReconTasksMicroserviceClientInterface;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::recon_tasks_microservice::ReconTasksMicroserviceClient;

// constants
const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5006";
const DEFAULT_DAPR_PUBSUB_NAME: &'static str = "pubsub";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8084;
const DEFAULT_RECON_TASKS_CONNECTION_URL: &'static str = "http://localhost:3600";
const DEFAULT_RECON_TASKS_SERVICE_ID: &'static str = "svc-task-details-repository-manager";

#[derive(Clone, Debug)]
struct AppSettings {
    pub app_port: String,

    pub app_ip: String,

    pub dapr_pubsub_name: String,

    pub dapr_pubsub_server_address: String,

    pub recon_tasks_service_name: String,

    pub recon_tasks_connection_url: String,
}

pub async fn run_async() -> Result<(), std::io::Error> {
    //retrieve app settings from the env variables
    let app_settings = read_app_settings();

    let app_listen_url = format!("{}:{}", app_settings.app_ip, app_settings.app_port);

    //just for logging purposes
    println!("App is listening on: {:?}", app_listen_url);

    HttpServer::new(move || {
        // Create some global state prior to running the handler threads
        let service = setup_service(app_settings.clone());

        // add shared state and routing
        App::new()
            .app_data(Data::new(service))
            .service(handlers::upload_file_chunk)
    })
        .bind(app_listen_url)?
        .run()
        .await
}

fn setup_service(app_settings: AppSettings) -> Box<dyn FileChunkUploadServiceInterface> {
    let recon_tasks_ms_client: Box<dyn ReconTasksMicroserviceClientInterface> = Box::new(ReconTasksMicroserviceClient {
        host: app_settings.recon_tasks_connection_url.clone(),
        recon_tasks_service_app_id: app_settings.recon_tasks_service_name.clone(),
    });
    let service: Box<dyn FileChunkUploadServiceInterface> = Box::new(FileChunkUploadService {
        file_upload_repo: Box::new(DaprPubSub {
            dapr_grpc_server_address: app_settings.dapr_pubsub_server_address.clone(),
            dapr_pubsub_name: app_settings.dapr_pubsub_name.clone(),
        }),

        recon_tasks_retriever: Box::new(ReconTasksServiceConnector::new(recon_tasks_ms_client)),
        to_entity_transformer: Box::new(Transformer {}),
    });
    service
}

fn read_app_settings() -> AppSettings {
    AppSettings {
        app_port: std::env::var("APP_PORT").unwrap_or(DEFAULT_APP_LISTEN_PORT.to_string()),

        app_ip: std::env::var("APP_IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string()),

        dapr_pubsub_name: std::env::var("DAPR_PUBSUB_NAME")
            .unwrap_or(DEFAULT_DAPR_PUBSUB_NAME.to_string()),

        dapr_pubsub_server_address: std::env::var("DAPR_IP")
            .unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string()),

        recon_tasks_service_name: std::env::var("RECON_TASKS_SERVICE_NAME")
            .unwrap_or(DEFAULT_RECON_TASKS_SERVICE_ID.to_string()),

        recon_tasks_connection_url: std::env::var("RECON_TASKS_SERVICE_HOST")
            .unwrap_or(DEFAULT_RECON_TASKS_CONNECTION_URL.to_string()),
    }
}
