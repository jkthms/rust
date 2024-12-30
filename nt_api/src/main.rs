mod handler;
mod route;
mod schema;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;

pub struct AppState {
    // Use Arc<Mutex<_>> to allow mutable access across threads
    pub fibonacci_cache: Arc<Mutex<HashMap<i32, i32>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            fibonacci_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

// Multi-threaded runtime API using 2 worker threads to handle requests
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app_state = AppState::new();

    let app = create_router(Arc::new(app_state)).layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}