use crate::{
    external::{
        pubsub::dapr_pubsub::DaprPubSubRepositoryManager,
        repositories::recon_tasks_retriever::ReconTasksDetailsRetriever,
    },
    internal::{
        web_api::handlers,
        interfaces::file_chunk_upload_service::FileChunkUploadServiceInterface,
        services::{
            core_logic::to_entity_transfomer::ToEntityTransfomer,
            file_upload_service::FileChunkUploadService,
        },
    },
};
use actix_web::{web::Data, App, HttpServer};

// constants
const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5005";
const DEFAULT_DAPR_PUBSUB_NAME: &'static str = "FileChunksQueue";
const DEFAULT_DAPR_PRIMARY_FILE_PUBSUB_TOPIC: &'static str = "PrimaryFileQueue";
const DEFAULT_DAPR_COMPARISON_FILE_PUBSUB_TOPIC: &'static str = "ComparisonFileQueue";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8080;

#[derive(Clone, Debug)]
struct AppSettings {
    pub app_port: String,

    pub app_ip: String,

    pub dapr_pubsub_name: String,

    pub dapr_pubsub_comparison_file_topic: String,

    pub dapr_pubsub_primary_file_topic: String,

    pub dapr_grpc_server_address: String,

    pub recon_tasks_service_name: String,
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
    let service: Box<dyn FileChunkUploadServiceInterface> = Box::new(FileChunkUploadService {
        file_upload_repo: Box::new(DaprPubSubRepositoryManager {
            dapr_grpc_server_address: app_settings.dapr_grpc_server_address.clone(),
            dapr_pubsub_name: app_settings.dapr_pubsub_name.clone(),
            dapr_pubsub_comparison_file_topic: app_settings
                .dapr_pubsub_comparison_file_topic
                .clone(),
            dapr_pubsub_primary_file_topic: app_settings.dapr_pubsub_primary_file_topic.clone(),
        }),

        recon_tasks_retriever: Box::new(ReconTasksDetailsRetriever {
            dapr_grpc_server_address: app_settings.dapr_grpc_server_address.clone(),
            recon_tasks_service_name: app_settings.recon_tasks_service_name.clone(),
        }),

        to_entity_transformer: Box::new(ToEntityTransfomer {}),
    });
    service
}

fn read_app_settings() -> AppSettings {
    AppSettings {
        app_port: std::env::var("APP_PORT").unwrap_or(DEFAULT_APP_LISTEN_PORT.to_string()),

        app_ip: std::env::var("APP_IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string()),

        dapr_pubsub_name: std::env::var("DAPR_PUBSUB_NAME")
            .unwrap_or(DEFAULT_DAPR_PUBSUB_NAME.to_string()),

        dapr_pubsub_primary_file_topic: std::env::var("DAPR_PUBSUB_PRIMRY_FILE_TOPIC")
            .unwrap_or(DEFAULT_DAPR_PRIMARY_FILE_PUBSUB_TOPIC.to_string()),

        dapr_pubsub_comparison_file_topic: std::env::var("DAPR_PUBSUB_COMPARISION_FILE_TOPIC")
            .unwrap_or(DEFAULT_DAPR_COMPARISON_FILE_PUBSUB_TOPIC.to_string()),

        dapr_grpc_server_address: std::env::var("DAPR_IP")
            .unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string()),

        recon_tasks_service_name: std::env::var("RECON_TASKS_SERVICE_NAME")
            .unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string()),
    }
}
