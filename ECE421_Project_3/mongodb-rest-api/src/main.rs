// External imports
use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use api_service::ApiService;

// External modules reference
mod api_router;
mod api_service;

// Api Service constructor
pub struct ServiceManager {
    api: ApiService,
}

// Api Servie Implementation
impl ServiceManager {
    pub fn new(api: ApiService) -> Self {
        ServiceManager { api }
    }
}

// Service Manager constructor
pub struct AppState {
    service_manager: ServiceManager,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // init env
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Parse a connection string into an options struct.
    let database_url = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(&database_url).unwrap();

    // Get the reference to Mongo DB
    let client = Client::with_options(client_options).unwrap();

    // get the reference to the Data Base
    // let database_name = env::var("DATABASE_NAME").expect("DATABASE NAME is not in .env file");
    let db = client.database("main");

    // get the reference to the Collection
    // let user_collection_name = env::var("USER_COLLECTION_NAME").expect("COLLECTION NAME is not in .env file");
    let user_collection = db.collection("users");
    let match_collection = db.collection("match_history");

    // Gte the server URL
    let server_url = "localhost:4000";

    // Start the server
    HttpServer::new(move || {
        let user_service_worker = ApiService::new(user_collection.clone(), match_collection.clone());
        let service_manager = ServiceManager::new(user_service_worker);

        // cors
        let cors_middleware = Cors::new()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .finish();

        // Init http server
        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .data(AppState { service_manager })
            .configure(api_router::init)
    })
    .bind(server_url)?
    .run()
    .await
}
