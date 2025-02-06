use axum::{
    extract::{State, Json, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;


use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::auth::validate_token,
};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

/// Recebe `maybe_payload` com `Result<Json<VerifyTokenRequest>, JsonRejection>`
/// e `State<AppState>` para poder acessar o banned token store.
pub async fn verify_token(
    State(state): State<AppState>,
    maybe_payload: Result<Json<VerifyTokenRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match maybe_payload {
        Ok(Json(payload)) => {
            // 1) Pegar lock de leitura do banned_token_store
            let banned_guard = state.banned_token_store.read().await;

            // 2) Chamar validate_token(token, &*banned_guard)
            //    Se token for inválido ou banido, retornamos AuthAPIError::InvalidToken
            validate_token(&payload.token, &*banned_guard)
                .await
                .map_err(|_| AuthAPIError::InvalidToken)?;

            // Se tudo der certo, retorna 200
            Ok(StatusCode::OK)
        }
        // Se falhou parse (campo ausente, JSON inválido, etc), retornamos MalformedInput => 422
        Err(_rejection) => {
            Err(AuthAPIError::MalformedInput)
        }
    }
}
