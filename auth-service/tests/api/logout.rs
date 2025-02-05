use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData, get_random_email};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;
use reqwest::cookie::CookieStore;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 400, "Should return 400 if JWT cookie is missing");
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // Adiciona cookie inválido
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;
    // Ajustei a mensagem para refletir a verificação de token inválido
    assert_eq!(response.status().as_u16(), 401, "Should return 401 if token is invalid");
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
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
    let url = Url::parse("http://127.0.0.1").unwrap();
    let cookie_header = (*app.cookie_jar)
        .cookies(&url)
        .expect("No cookies at all for this URL");

    let cookies_str = cookie_header
        .to_str()
        .expect("Failed to convert header to string");
    // cookies_str algo como "jwt=valor; outro=algo"

    let cookie_found = cookies_str
        .split(';')
        .map(|s| s.trim())
        .any(|cookie_pair| cookie_pair.starts_with(&format!("{}=", JWT_COOKIE_NAME)));

    assert!(
        cookie_found,
        "JWT cookie should be present after login"
    );

    let logout_response = app.logout().await;
    assert_eq!(logout_response.status().as_u16(), 200, "Should return 200 with valid JWT");
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
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

    // Verificar cookie JWT
    let url = Url::parse("http://127.0.0.1").unwrap();
    let cookie_header = (*app.cookie_jar)
        .cookies(&url)
        .expect("No cookies at all for this URL");

    let cookies_str = cookie_header
        .to_str()
        .expect("Failed to convert header to string");
    let cookie_found = cookies_str
        .split(';')
        .map(|s| s.trim())
        .any(|cookie_pair| cookie_pair.starts_with(&format!("{}=", JWT_COOKIE_NAME)));

    assert!(cookie_found, "JWT cookie should be present after login");

    // Primeiro logout: deve ser 200
    let first_logout = app.logout().await;
    assert_eq!(first_logout.status().as_u16(), 200, "Should return 200 with valid JWT");

    // Segundo logout: sem cookie (ou token inválido), deve ser 400
    let second_logout = app.logout().await;
    assert_eq!(second_logout.status().as_u16(), 400, "Should return 400 for logout twice in a row");
}
