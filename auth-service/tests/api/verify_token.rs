use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData, get_random_email};
//idk if i need anything else than TestApp here

use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;
use serde_json::json;
use reqwest::cookie::CookieStore;


#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let malformed_body = serde_json::json!({
        "invalid_field": "some_value"
    });

    let response = app.post_verify_token(&malformed_body).await;

    assert_eq!(
        response.status().as_u16(),
        422,
        "Expected 422 (Unprocessable Entity) for malformed input"
    );
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    // Verificar se cookie JWT está presente
    let body = response.json::<serde_json::Value>().await.unwrap();
    let valid_token = body["token"].as_str().expect("No token in body");

    // 3) Verificar o token no endpoint /verify-token
    let verify_payload = json!({ "token": valid_token });
    let verify_response = app.post_verify_token(&verify_payload).await;

    assert_eq!(
        verify_response.status().as_u16(),
        200,
        "Expected 200 for a valid token"
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    // Verificar se cookie JWT está presente
    let body = response.json::<serde_json::Value>().await.unwrap();
    let valid_token = body["token"].as_str().expect("No token in body");

    // 3) Verificar o token no endpoint /verify-token
    let verify_payload = json!({ "token": "this_token_is_invalid" });
    let verify_response = app.post_verify_token(&verify_payload).await;

    assert_eq!(
        verify_response.status().as_u16(),
        401,
        "Expected 200 for a valid token"
    );
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;

    // 1) Faça signup e login (ou gere um token de outra forma)
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let login_response = app.post_login(&login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);

    // 2) Extrair o token do cookie (ou do body, se for o caso).
    let url = Url::parse("http://127.0.0.1").unwrap();
    let cookie_header = (*app.cookie_jar)
        .cookies(&url)
        .expect("No cookies for this URL");
    let cookies_str = cookie_header.to_str().expect("Failed to to_str()");
    let full_cookie = cookies_str
        .split(';')
        .map(|s| s.trim())
        .find(|p| p.starts_with(&format!("{}=", JWT_COOKIE_NAME)))
        .expect("JWT cookie not found");
    let token = full_cookie
        .split('=')
        .nth(1)
        .expect("Failed to get token")
        .to_string();

    // 3) Banir esse token manualmente no store
    {
        let mut guard = app.banned_token_store.write().await;
        guard.store_token(&token);
    }

    // 4) Chamar /verify-token com o token banido
    let verify_payload = serde_json::json!({
        "token": token,
    });
    let verify_response = app.post_verify_token(&verify_payload).await;

    // 5) Esperamos 401
    assert_eq!(
        verify_response.status().as_u16(),
        401,
        "Should return 401 for a banned token"
    );
}