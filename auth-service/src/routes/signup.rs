use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::user::User;
use crate::domain::error::AuthAPIError;

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>,) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;
    // Create a new `User` instance using data in the `request`

    if email.is_empty() || !email.contains('@') || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;
    // TODO: early return AuthAPIError::UserAlreadyExists if email exists in user_store.

    // TODO: instead of using unwrap, early return AuthAPIError::UnexpectedError if add_user() fails.
    if user_store.users.contains_key(&user.email) {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    match user_store.add_user(user) {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            return Ok((StatusCode::CREATED, response))
        }
        Err(_) => {
            return Err(AuthAPIError::UnexpectedError)
        }
    }

    user_store.add_user(user).unwrap();
    
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
