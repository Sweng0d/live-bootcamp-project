use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    utils::{
        auth::generate_auth_cookie,
        constants::JWT_COOKIE_NAME,
    },
    domain::{
        data_stores::{UserStore, UserStoreError, LoginAttemptId, TwoFACode},
        email::Email,
        error::AuthAPIError,
        password::Password,
    },
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum LoginResponse {
    TwoFactorAuth {
        message: String,
        #[serde(rename = "loginAttemptId")]
        login_attempt_id: String,
    },
    RegularAuth {
        token: String,
        message: String,
    },
}

/// Rota pública de login.
/// Retorna (CookieJar, Result<impl IntoResponse, AuthAPIError>)
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    match login_internal(state, jar, request).await {
        Ok((cookie_jar, response)) => (cookie_jar, Ok(response)),
        Err((cookie_jar, e)) => (cookie_jar, Err(e)),
    }
}

/// Função interna de login, faz a verificação de credenciais
/// e decide se retorna 2FA (206) ou login normal (200).
async fn login_internal(
    state: AppState,
    jar: CookieJar,
    request: LoginRequest,
) -> Result<(CookieJar, Response), (CookieJar, AuthAPIError)> {
    // 1) Valida input
    if request.email.len() < 8 || request.password.len() < 8 {
        return Err((jar, AuthAPIError::InvalidCredentials));
    }

    // 2) Converte para tipos de domínio
    let email = Email::parse(&request.email)
        .map_err(|_| (jar.clone(), AuthAPIError::InvalidCredentials))?;
    let password = Password::parse(&request.password)
        .map_err(|_| (jar.clone(), AuthAPIError::InvalidCredentials))?;

    // 3) Valida usuário
    let user_store = &state.user_store.read().await;
    user_store.validate_user(&email, &password).await
        .map_err(|err| match err {
            UserStoreError::InvalidCredentials | UserStoreError::UserNotFound => {
                (jar.clone(), AuthAPIError::IncorrectCredentials)
            }
            _ => (jar.clone(), AuthAPIError::UnexpectedError),
        })?;

    // 4) Obtem dados do usuário
    let user = user_store.get_user(&email).await
        .map_err(|_| (jar.clone(), AuthAPIError::IncorrectCredentials))?;

    // 5) Se requer 2FA -> handle_2fa; senão -> handle_no_2fa
    if user.requires_2fa {
        handle_2fa(&email, &state, jar).await
    } else {
        handle_no_2fa(&email, jar).await
    }
}

/// Se 2FA é exigido, gera e armazena o loginAttemptId e o 2FA code, retornando 206
async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> Result<(CookieJar, Response), (CookieJar, AuthAPIError)> {
    // 1) Gera um login attempt ID e um 2FA code aleatório
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    // 2) Salva no two_fa_code_store
    {
        let mut two_fa_store = state.two_fa_code_store.write().await;
        two_fa_store
            .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
            .await
            .map_err(|_| (jar.clone(), AuthAPIError::UnexpectedError))?;
    }

    let subject = "Your 2FA Code";
    let body = format!("Olá! Seu código de 2FA é: {}", two_fa_code.as_ref());
    state.email_client
        .send_email(email, subject, &body)
        .await
        .map_err(|_| (jar.clone(), AuthAPIError::UnexpectedError))?;

    // 3) Resposta 206 + JSON
    let response_body = LoginResponse::TwoFactorAuth {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_string(),
    };
    let response = (StatusCode::PARTIAL_CONTENT, Json(response_body)).into_response();
    Ok((jar, response))
}

/// Se 2FA não é exigido, faz login normal (gera cookie JWT e retorna 200)
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> Result<(CookieJar, Response), (CookieJar, AuthAPIError)> {
    let auth_cookie = generate_auth_cookie(email)
        .map_err(|_| (jar.clone(), AuthAPIError::UnexpectedError))?;
    let updated_jar = jar.add(auth_cookie);

    let token_cookie = updated_jar
        .get(JWT_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    let response_body = LoginResponse::RegularAuth {
        token: token_cookie,
        message: "Logged in successfully".to_string(),
    };
    let response = (StatusCode::OK, Json(response_body)).into_response();
    Ok((updated_jar, response))
}
