use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData};
//idk if i need anything else than TestApp here

#[tokio::test]
async fn verify_token_returns_200() {
    let app = TestApp::new().await;

    let token_data = TokenData {
        token: "dummy_jwt_token".to_string(),
    };

    let response = app.verify_token(&token_data).await;
    assert_eq!(response.status().as_u16(), 200);
}