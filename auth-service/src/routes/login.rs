use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::domain::{
    email::Email,
    error::AuthAPIError,
    password::Password,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>,) -> Result<impl IntoResponse, AuthAPIError> {
    if request.email.len() < 8 || request.password.len() < 8 {
        return Ok((StatusCode::BAD_REQUEST, "Invalid input").into_response());
    }

    let email = match Email::parse(&request.email) {
        Ok(mail) => mail,
        Err(_) => {
            return Err(AuthAPIError::InvalidCredentials);
        }
    };

    let password = match Password::parse(&request.password) {
        Ok(pass) => pass,
        Err(_) => {
            return Err(AuthAPIError::InvalidCredentials);
        }
    };

    Ok((StatusCode::OK, "Logged in successfully").into_response())
}