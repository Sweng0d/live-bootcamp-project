use crate::helpers::{TestApp, get_random_email};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use serde_json::json;


#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    // Casos de corpo de login malformados: sem password, sem username, credenciais muito curtas etc.
    let test_cases = vec![
        serde_json::json!({ "username": "missing_password" }),
        serde_json::json!({ "password": "missing_username" }),
        serde_json::json!({ "username": "tes", "password": "123" }), // curtas demais etc.
    ];

    for body in test_cases {
        // POST /login com o JSON malformado
        let response = app.post_login(&body).await;

        // Esperamos 422 (Unprocessable Entity)
        assert_eq!(
            422,
            response.status().as_u16(),
            "Expected 422 for malformed credentials. Payload: {:?}",
            body
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message. 
    let app = TestApp::new().await;

    let invalid_login_payload = json!({
        "email": "invalidemail.com",   // sem '@'
        "password": ""
    });

    let response = app.post_login(&invalid_login_payload).await;

    assert_eq!(
        400,
        response.status().as_u16(),
        "Expected 400 Bad Request for invalid input. Payload: {:?}",
        invalid_login_payload
    );

}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.     
    let app = TestApp::new().await;

    let signup_data = serde_json::json!({
        "email": "correct_user@example.com",
        "password": "secret123"
    });

    app.post_signup(&signup_data).await;

    let invalid_login = serde_json::json!({
        "email": "correct_user@example.com",
        "password": "wrongpassword"
    });

    let response = app.post_login(&invalid_login).await;

    assert_eq!(
        401,
        response.status().as_u16(),
        "Expected 401 Unauthorized for incorrect credentials."
    );

}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
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

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let login_response = app.post_login(&login_body).await;

    assert_eq!(
        206,
        login_response.status().as_u16(),
        "Esperado 206 para credenciais válidas com 2FA habilitado."
    );

    let json_body: serde_json::Value = login_response
        .json()
        .await
        .expect("Falha ao converter resposta para JSON.");

    let message = json_body
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert_eq!(
        message,
        "2FA required",
        "Esperado '2FA required' no campo 'message'."
    );

    let login_attempt_id = json_body
        .get("loginAttemptId")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        !login_attempt_id.is_empty(),
        "Esperado campo 'loginAttemptId' não vazio."
    );

    // -- Verificação extra: o loginAttemptId está salvo no two_fa_code_store? --
    // 1) Transforma `random_email` em `Email`
    let user_email = auth_service::domain::email::Email::parse(&random_email)
        .expect("Falha ao parsear random_email para Email");

    // 2) Dá "read" no two_fa_code_store
    let guard = app.two_fa_code_store.read().await;

    // 3) Busca (LoginAttemptId, TwoFACode) do usuário
    let (stored_attempt_id, stored_two_fa_code) = guard
        .get_code(&user_email)
        .await
        .expect("Nenhum 2FA code encontrado no store para esse usuário!");

    // 4) Compara
    assert_eq!(
        stored_attempt_id.as_ref(),
        login_attempt_id,
        "loginAttemptId armazenado não bate com o retornado pela rota"
    );
    assert!(
        !stored_two_fa_code.as_ref().is_empty(),
        "Código 2FA salvo não deveria estar vazio."
    );



}