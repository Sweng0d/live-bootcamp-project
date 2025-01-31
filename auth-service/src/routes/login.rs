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
    data_stores::UserStoreError,
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

    let user_store = &state.user_store.read().await;

    match user_store.validate_user(&email, &password).await {
        Ok(_) => {
            // Credenciais OK, pode prosseguir
        },
        Err(UserStoreError::InvalidCredentials) | Err(UserStoreError::UserNotFound) => {
            // Falha de credenciais => retorna 401
            return Err(AuthAPIError::IncorrectCredentials);
        }
        Err(_) => {
            // Qualquer outro erro do store => UnexpectedError
            return Err(AuthAPIError::UnexpectedError);
        }
    }

    Ok((StatusCode::OK, "Logged in successfully").into_response())
}