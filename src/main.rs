mod api;
mod map;
mod map_formatter;
mod map_parser;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod handler_tests;

use api::{create_api_router, MapStore};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize map storage
    let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Create API router
    let app = create_api_router(map_store).layer(cors);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}


