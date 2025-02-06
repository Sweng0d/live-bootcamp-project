use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::{
    app_state::AppState, // Precisamos disso
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,   // recebemos AppState
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    // 1) Se não tiver cookie, erro
    let cookie = jar
        .get(JWT_COOKIE_NAME)
        .ok_or(AuthAPIError::MissingToken)?;

    // 2) Verifica se o token é válido
    // Obtemos lock de leitura do banned_token_store
    let read_guard = state.banned_token_store.read().await;
    validate_token(cookie.value(), &*read_guard)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    drop(read_guard); // libera o lock de leitura (opcional, mas bom para clareza)

    // 3) Banir o token - agora precisamos do lock de escrita
    let mut write_guard = state.banned_token_store.write().await;
    write_guard.store_token(cookie.value());
    drop(write_guard); // libera o lock de escrita

    // 4) Remove o cookie do CookieJar
    let updated_jar = jar.remove(Cookie::new(JWT_COOKIE_NAME, ""));

    // 5) Retorna jar atualizado + 200
    Ok((updated_jar, StatusCode::OK))
}
