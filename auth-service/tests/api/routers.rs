use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData};
//idk if i need anything else than TestApp here

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

// TODO: Implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token)
// For now, simply assert that each route returns a 200 HTTP status code.

#[tokio::test]
async fn signup_returns_200() {
    let app = TestApp::new().await;

    let signup_data = SignupData {
        username: "test_user".to_string(),
        password: "test_password".to_string(),
    };

    let response = app.signup(&signup_data).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_returns_200() {
    let app = TestApp::new().await;

    let signup_data = SignupData {
        username: "test_login_user".to_string(),
        password: "test_login_password".to_string(),
    };

    app.signup(&signup_data).await;

    let login_data = LoginData {
        username: "test_login_user".to_string(),
        password: "test_login_password".to_string(),
    };

    let response = app.login(&login_data).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_returns_200() {
    let app = TestApp::new().await;

    let signup_data = SignupData {
        username: "test_logout_user".to_string(),
        password: "test_logout_password".to_string(),
    };

    app.signup(&signup_data).await;

    let login_data = LoginData {
        username: "test_logout_user".to_string(),
        password: "test_logout_password".to_string(),
    };

    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_returns_200() {
    let app = TestApp::new().await;

    let two_fa_data = TwoFaData {
        token: "dummy_2fa_token".to_string(),
    };

    let response = app.verify_2fa(&two_fa_data).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_returns_200() {
    let app = TestApp::new().await;

    let token_data = TokenData {
        token: "dummy_jwt_token".to_string(),
    };

    let response = app.verify_token(&token_data).await;
    assert_eq!(response.status().as_u16(), 200);
}



