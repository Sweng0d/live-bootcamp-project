use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    jar: CookieJar
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    // Se não tiver cookie, erro
    let cookie = jar
        .get(JWT_COOKIE_NAME)
        .ok_or(AuthAPIError::MissingToken)?;

    // Se token for inválido, erro
    validate_token(cookie.value())
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    // Remove o cookie pelo nome
    let updated_jar = jar.remove(Cookie::new(JWT_COOKIE_NAME, ""));

    // Retorna CookieJar atualizado + 200
    Ok((updated_jar, StatusCode::OK))
}
