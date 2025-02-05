use axum::{
    extract::{
        Json,
        rejection::JsonRejection
    },
    http::StatusCode,
    response::{IntoResponse},
};
use serde::Deserialize;

use crate::{
    domain::AuthAPIError,
    utils::auth::validate_token,
};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

// Em vez de receber `Json<VerifyTokenRequest>` diretamente,
// usamos `Result<Json<VerifyTokenRequest>, JsonRejection>`
// e fazemos `match` para devolver 422 se der erro de parse.

pub async fn verify_token(
    maybe_payload: Result<Json<VerifyTokenRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match maybe_payload {
        Ok(Json(payload)) => {
            // Se der certo o parse, validamos o token
            validate_token(&payload.token)
                .await
                .map_err(|_| AuthAPIError::InvalidToken)?;
            // Se tudo ok, retorna 200
            Ok(StatusCode::OK)
        }
        Err(_rejection) => {
            // Se falhou parse (campo ausente, JSON inválido, etc),
            // retornamos ERR do tipo AuthAPIError. Mas se seu
            // AuthAPIError não tiver "MalformedInput", você pode
            // criar um e mapear p/ 422 no `impl IntoResponse`.

            // Para simplificar, podemos "inventar" um erro ou
            // mapeá-lo para 'InvalidCredentials' c/ status 422,
            // mas o ideal é criar algo como `AuthAPIError::Unprocessable`.

            // Exemplo rápido:
            Err(AuthAPIError::MalformedInput)
        }
    }
}
