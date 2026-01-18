use http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;

pub async fn hello_handler(
    headers: HeaderMap
) -> impl IntoResponse {
    (
        StatusCode::OK,
        "{\"result\": \"你好\"}"
    ).into_response()
}