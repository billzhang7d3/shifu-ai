use axum::{
    routing::{get},
    Router,
    http::Method
};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
// use std::{
//     sync::Arc,
//     env
// };
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers;

pub async fn create_app() -> Router {
    let cors = CorsLayer::new()
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);
    // let client_arc = Arc::new(create_client().await);
    // let client_clone = Arc::clone(&client_arc);
    return Router::new()
        // .with_state(client_clone)
        .route("/api/v0/hello", get(handlers::hello::hello_handler))
        .layer(ServiceBuilder::new()
            .layer(cors));
}