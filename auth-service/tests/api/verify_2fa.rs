use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData};
//idk if i need anything else than TestApp here

#[tokio::test]
async fn verify_2fa_returns_200() {
    let app = TestApp::new().await;

    let two_fa_data = TwoFaData {
        token: "dummy_2fa_token".to_string(),
    };

    let response = app.verify_2fa(&two_fa_data).await;
    assert_eq!(response.status().as_u16(), 200);
}