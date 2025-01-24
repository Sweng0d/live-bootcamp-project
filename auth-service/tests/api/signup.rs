use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData};
//idk if i need anything else than TestApp here

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