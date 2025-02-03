use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    utils::auth::generate_auth_cookie,
};
use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::error::AuthAPIError;
use crate::domain::data_stores::UserStoreError;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Ajustamos para retornar `Result<(CookieJar, impl IntoResponse), AuthAPIError>`
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    // Se dados insuficientes, retorne BAD_REQUEST
    if request.email.len() < 8 || request.password.len() < 8 {
        return Ok((jar, (StatusCode::BAD_REQUEST, "Invalid input")));
        // ou se preferir, poderia retornar um Err(AuthAPIError::InvalidCredentials)
        // return Err(AuthAPIError::InvalidCredentials);
    }

    // Tenta parsear o e-mail
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Tenta parsear a senha
    let password = Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Faz a leitura do store e valida as credenciais
    let user_store = &state.user_store.read().await;
    match user_store.validate_user(&email, &password).await {
        Ok(_) => { /* credenciais OK */ },
        Err(UserStoreError::InvalidCredentials) 
         | Err(UserStoreError::UserNotFound) => {
            // Falha de credenciais => 401
            return Err(AuthAPIError::IncorrectCredentials);
        }
        Err(_) => {
            // Erro inesperado do store
            return Err(AuthAPIError::UnexpectedError);
        }
    }

    // Gerar cookie de autenticação
    let auth_cookie = generate_auth_cookie(&email)
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    // Adicionar ao jar
    let updated_jar = jar.add(auth_cookie);

    // Resposta de sucesso
    Ok((updated_jar, (StatusCode::OK, "Logged in successfully")))
}
