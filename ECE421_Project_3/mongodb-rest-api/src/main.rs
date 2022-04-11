use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use api_service::ApiService;

// External modules reference
mod api_router;
mod api_service;

// Api Service Constructor
pub struct ServiceManager {
    api: ApiService,
}

// Api Service Implementation
impl ServiceManager {
    pub fn new(api: ApiService) -> Self {
        ServiceManager { api }
    }
}

// Service Manager Constructor
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

    // set databse url
    let database_url = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(&database_url).unwrap();

    // get reference of mongodb
    let client = Client::with_options(client_options).unwrap();

    // get the reference to the database
    let db = client.database("main");

    // get the reference to the Collection
    let user_collection = db.collection("users");
    let match_collection = db.collection("match_history");

    let server_url = "localhost:4000";

    // start the rest server
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

        // init http server
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
