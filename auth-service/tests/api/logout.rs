use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData};
//idk if i need anything else than TestApp here

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