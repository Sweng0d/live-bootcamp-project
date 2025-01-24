use axum::response::IntoResponse;
use axum::http::StatusCode;

pub async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}