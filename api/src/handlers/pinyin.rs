use axum::extract::State;
use axum::response::IntoResponse;
use futures_util::TryStreamExt;
use http::{HeaderMap, StatusCode};
use mongodb::bson::{doc, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::sync::Arc;

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIResponseMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Serialize)]
struct PinyinStats {
    pinyin: String,
    correct: i32,
    incorrect: i32,
    total_attempts: i32,
    accuracy: f64,
}

fn validate_pinyin_response(content: &str) -> Option<Value> {
    // Try to parse and validate the response has pinyin and character fields
    if let Ok(parsed) = serde_json::from_str::<Value>(content) {
        if parsed.get("pinyin").is_some() && parsed.get("character").is_some() {
            return Some(parsed);
        }
    }
    None
}

pub async fn pinyin_recommend_handler(
    headers: HeaderMap,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    // Get username from header
    let username = match headers.get("username") {
        Some(u) => match u.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    axum::Json(serde_json::json!({
                        "result": "error",
                        "message": "Invalid username header"
                    })),
                ).into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({
                    "result": "error",
                    "message": "username header is required"
                })),
            ).into_response();
        }
    };

    // Query MongoDB for user's pinyin history
    let collection = db.collection::<Document>("pronounce");
    let filter = doc! { "username": &username };

    let pinyin_stats: Vec<PinyinStats> = match collection.find(filter).await {
        Ok(mut cursor) => {
            let mut stats = Vec::new();
            while let Ok(Some(doc)) = cursor.try_next().await {
                let pinyin = doc.get_str("pinyin").unwrap_or_default().to_string();
                let correct = doc.get_i32("correct").unwrap_or(0);
                let incorrect = doc.get_i32("incorrect").unwrap_or(0);
                let total_attempts = correct + incorrect;

                // Only include pinyin with at least 10 attempts
                if total_attempts >= 10 {
                    let accuracy = if total_attempts > 0 {
                        (correct as f64 / total_attempts as f64) * 100.0
                    } else {
                        0.0
                    };

                    stats.push(PinyinStats {
                        pinyin,
                        correct,
                        incorrect,
                        total_attempts,
                        accuracy,
                    });
                }
            }
            stats
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(serde_json::json!({
                    "result": "error",
                    "message": format!("Failed to query MongoDB: {}", e)
                })),
            ).into_response();
        }
    };

    // Get OpenAI API key
    let openai_api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(serde_json::json!({
                    "result": "error",
                    "message": "OPENAI_API_KEY environment variable not set"
                })),
            ).into_response();
        }
    };

    // Format pinyin stats for the prompt
    let stats_text: Vec<String> = pinyin_stats
        .iter()
        .map(|s| format!("{}: {:.1}% accuracy ({}/{} correct)", s.pinyin, s.accuracy, s.correct, s.total_attempts))
        .collect();

    let prompt = format!(
        "You are a Chinese language tutor helping a student practice pinyin pronunciation. \
        Here is the student's performance history for pinyin they have attempted at least 10 times:\n\n{}\n\n\
        Based on this information, suggest ONE new pinyin for them to practice next. \
        Consider their weak areas (low accuracy) and provide something at an appropriate difficulty level. \
        If they struggle with certain sounds, suggest similar sounds to practice. \
        Respond with ONLY a JSON object in this format: \
        {{\"pinyin\": \"the pinyin with tone marks\", \"character\": \"the Chinese character\"}}",
        stats_text.join("\n")
    );

    let request_body = OpenAIRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![OpenAIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        max_tokens: 200,
    };

    let client = reqwest::Client::new();
    let mut last_error: Option<String> = None;

    // Retry up to 4 times to get a valid response
    for _retry in 0..4 {
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", openai_api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await;

        match response {
            Ok(res) => {
                if !res.status().is_success() {
                    let status = res.status();
                    let error_text = res.text().await.unwrap_or_default();
                    last_error = Some(format!("OpenAI API error: {} - {}", status, error_text));
                    continue;
                }

                match res.json::<OpenAIResponse>().await {
                    Ok(openai_response) => {
                        if let Some(choice) = openai_response.choices.first() {
                            let content = &choice.message.content;

                            // Validate the response has required fields
                            if let Some(parsed) = validate_pinyin_response(content) {
                                return (
                                    StatusCode::OK,
                                    axum::Json(serde_json::json!({
                                        "result": "success",
                                        "user_stats": pinyin_stats,
                                        "recommendation": parsed
                                    })),
                                ).into_response();
                            } else {
                                last_error = Some(format!("Invalid response format: {}", content));
                                continue;
                            }
                        } else {
                            last_error = Some("No response from OpenAI".to_string());
                            continue;
                        }
                    }
                    Err(e) => {
                        last_error = Some(format!("Failed to parse OpenAI response: {}", e));
                        continue;
                    }
                }
            }
            Err(e) => {
                last_error = Some(format!("Failed to call OpenAI API: {}", e));
                continue;
            }
        }
    }

    // All retries failed
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        axum::Json(serde_json::json!({
            "result": "error",
            "message": last_error.unwrap_or_else(|| "Unknown error after retries".to_string())
        })),
    ).into_response()
}
