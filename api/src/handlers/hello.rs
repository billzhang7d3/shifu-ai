use http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::extract::State;
use axum::Json;
use mongodb::Database;
use std::sync::Arc;
use mongodb::bson::{doc, DateTime, Document};
use std::time::SystemTime;
use futures_util::TryStreamExt;
use serde_json::Value;

pub async fn hello_handler(
    headers: HeaderMap
) -> impl IntoResponse {
    (
        StatusCode::OK,
        "{\"result\": \"你好\"}"
    ).into_response()
}

pub async fn sayheykid_get_handler(
    State(db): State<Arc<Database>>
) -> impl IntoResponse {
    let collection = db.collection::<mongodb::bson::Document>("greetings");
    
    match collection.find(doc! {}).await {
        Ok(mut cursor) => {
            let mut documents = Vec::new();
            
            while let Ok(Some(doc)) = cursor.try_next().await {
                documents.push(doc);
            }
            
            let response = serde_json::json!({
                "result": "success",
                "count": documents.len(),
                "documents": documents
            });
            (
                StatusCode::OK,
                axum::Json(response)
            ).into_response()
        }
        Err(e) => {
            let response = serde_json::json!({
                "result": "error",
                "message": format!("Failed to retrieve documents from MongoDB: {}", e)
            });
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(response)
            ).into_response()
        }
    }
}

pub async fn sayheykid_post_handler(
    State(db): State<Arc<Database>>,
    Json(payload): Json<Value>
) -> impl IntoResponse {
    let collection = db.collection::<Document>("greetings");
    
    let mut greeting_doc = doc! {
        "timestamp": DateTime::from_system_time(SystemTime::now()),
    };
    
    // Convert the JSON payload to BSON and merge it into the document
    if let Ok(bson_value) = mongodb::bson::to_bson(&payload) {
        if let mongodb::bson::Bson::Document(mut incoming_doc) = bson_value {
            incoming_doc.insert("timestamp", greeting_doc.get("timestamp").unwrap().clone());
            greeting_doc = incoming_doc;
        }
    }
    
    match collection.insert_one(greeting_doc).await {
        Ok(insert_result) => {
            let response = serde_json::json!({
                "result": "success",
                "message": "Document inserted successfully",
                "id": insert_result.inserted_id.to_string()
            });
            (
                StatusCode::CREATED,
                Json(response)
            ).into_response()
        }
        Err(e) => {
            let response = serde_json::json!({
                "result": "error",
                "message": format!("Failed to insert into MongoDB: {}", e)
            });
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(response)
            ).into_response()
        }
    }
}