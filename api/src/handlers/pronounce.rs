use http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::extract::State;
use axum::Json;
use mongodb::Database;
use mongodb::bson::{doc, DateTime, Document};
use std::sync::Arc;
use std::time::SystemTime;
use serde_json::Value;
use futures_util::TryStreamExt;

pub async fn pronounce_get_handler(
    headers: HeaderMap,
    State(db): State<Arc<Database>>
) -> impl IntoResponse {
    if headers.get("username").is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "{\"result\": \"error\", \"message\": \"username is required\"}"
        ).into_response();
    }

    let username = headers.get("username").unwrap().to_str().unwrap();

    let collection = db.collection::<Document>("pronounce");
    match collection.find(doc! {}).await {
        Ok(mut cursor) => {
            let mut documents = Vec::new();
            
            while let Ok(Some(doc)) = cursor.try_next().await {
                println!("doc result: {:?}", doc);
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

// payload has the following fields:
// pinyin: string, correct: boolean, timestamp: timestamp
pub async fn pronounce_post_handler(
    headers: HeaderMap,
    State(db): State<Arc<Database>>,
    Json(payload): Json<Value>
) -> impl IntoResponse {
    if headers.get("username").is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "{\"result\": \"error\", \"message\": \"username is required\"}"
        ).into_response();
    }

    let username = headers.get("username").unwrap().to_str().unwrap();
    
    let collection = db.collection::<Document>("pronounce");

    let pinyin = payload.get("pinyin")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let correct = payload.get("correct")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let timestamp = DateTime::from_system_time(SystemTime::now());

    // Build the filter to find existing document by username
    let filter = doc! {
        "username": username
    };

    // Determine initial values for new documents and increment operations for existing ones
    let correct_init = if correct { (1, 0) } else { (0, 1) };

    // Check if document exists first
    match collection.find_one(filter.clone()).await {
        Ok(Some(_existing_doc)) => {
            // Document exists - increment the appropriate field
            let mut update_ops = doc! {
                "$set": {
                    "pinyin": pinyin,
                    "timestamp": timestamp,
                }
            };
            
            if correct {
                update_ops.insert("$inc", doc! { "correct": 1 });
            } else {
                update_ops.insert("$inc", doc! { "incorrect": 1 });
            }

            match collection.update_one(filter, update_ops).await {
                Ok(_update_result) => {
                    (
                        StatusCode::OK,
                        "{\"result\": \"success\", \"message\": \"Pronounce updated successfully\"}"
                    ).into_response()
                }
                Err(e) => {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("{{\"result\": \"error\", \"message\": \"Failed to update MongoDB: {:?}\"}}", e)
                    ).into_response()
                }
            }
        }
        Ok(None) => {
            // Document doesn't exist - insert new one
            let insert_doc = doc! {
                "username": username,
                "pinyin": pinyin,
                "correct": correct_init.0,
                "incorrect": correct_init.1,
                "timestamp": timestamp,
            };

            match collection.insert_one(insert_doc).await {
                Ok(_insert_result) => {
                    (
                        StatusCode::CREATED,
                        "{\"result\": \"success\", \"message\": \"Pronounce inserted successfully\"}"
                    ).into_response()
                }
                Err(e) => {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("{{\"result\": \"error\", \"message\": \"Failed to insert into MongoDB: {:?}\"}}", e)
                    ).into_response()
                }
            }
        }
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{{\"result\": \"error\", \"message\": \"Failed to query MongoDB: {:?}\"}}", e)
            ).into_response()
        }
    }
}