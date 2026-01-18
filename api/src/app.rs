use axum::{
    routing::{get, post},
    Router,
    http::Method
};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use mongodb::{Client, Database};
use std::{
    sync::Arc,
    env
};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers;

async fn create_mongodb_client() -> Client {
    let mongodb_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://admin:password@localhost:27017/?authSource=admin".to_string());
    
    Client::with_uri_str(&mongodb_uri)
        .await
        .expect("Failed to create MongoDB client")
}

pub async fn create_app() -> Router {
    let cors = CorsLayer::new()
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);
    
    let client = create_mongodb_client().await;
    let db_name = env::var("MONGODB_DATABASE").unwrap_or_else(|_| "shifu".to_string());
    let database: Database = client.database(&db_name);
    let db_arc = Arc::new(database);
    
    return Router::new()
        .route("/api/v0/hello", get(handlers::hello::hello_handler))
        .route("/api/v0/sayheykid", get(handlers::hello::sayheykid_handler))
        .route("/api/v0/sayheykid", post(handlers::hello::sayheykid_post_handler))
        .with_state(db_arc)
        .layer(ServiceBuilder::new()
            .layer(cors));
}


// API endpoints:
// - /api/v0/hello: Hello world endpoint
// - /api/v0/pronounce: Pronounce endpoint
//     - GET: Get a list of recommended Chinese characters
//             - userId: string
//     - POST: Send the user's pronunciation of a character
//         - userId: string
//         - character: string
//         - timestamp: timestamp


// the good characters are stored in a mongo database, where the key is the userId and the value is a list of characters.
// The value is a list of characters that the user has pronounced correctly.
// The list of characters is sorted by the timestamp of the last pronunciation.
// The list of characters is updated when the user pronounces a character correctly.
// The list of characters is updated when the user pronounces a character incorrectly.
// The list of characters is updated when the user pronounces a character correctly.
// The list of characters is updated when the user pronounces a character correctly.
