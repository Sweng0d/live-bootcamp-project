use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use serde::{Serialize, Deserialize};

use crate::{
    app_state::AppState,
    utils::auth::generate_auth_cookie,
    domain::{email::Email, password::Password, error::AuthAPIError, data_stores::UserStoreError},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    message: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, Response), AuthAPIError> {
    // 1) Verifica input
    if request.email.len() < 8 || request.password.len() < 8 {
        let resp = (StatusCode::BAD_REQUEST, "Invalid input").into_response();
        return Ok((jar, resp));
    }

    // 2) Converte p/ tipos de domínio
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // 3) Valida usuário no store
    let user_store = &state.user_store.read().await;
    match user_store.validate_user(&email, &password).await {
        Ok(_) => {}, // OK
        Err(UserStoreError::InvalidCredentials | UserStoreError::UserNotFound) => {
            return Err(AuthAPIError::IncorrectCredentials);
        }
        Err(_) => {
            return Err(AuthAPIError::UnexpectedError);
        }
    }

    // 4) Gera cookie e adiciona
    let auth_cookie = generate_auth_cookie(&email)
        .map_err(|_| AuthAPIError::UnexpectedError)?;
    let updated_jar = jar.add(auth_cookie);

    // 5) Extrai o token do cookie
    let token_cookie = updated_jar
        .get(crate::utils::constants::JWT_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    // Monta JSON final
    let body = LoginResponse {
        token: token_cookie,
        message: "Logged in successfully".into(),
    };

    // 6) Converte (StatusCode::OK, Json(body)) em `Response`
    let resp = (StatusCode::OK, Json(body)).into_response();

    // 7) Retorna a tupla
    Ok((updated_jar, resp))
}
