use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData, get_random_email};
use auth_service::routes::SignupResponse;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email 

    //add more malformed input test cases
    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),

        serde_json::json!({
            "logan": random_email,
            "requires2FA": false
        }),

        serde_json::json!({
            "login": "login",
            "requires2FA": false
        }),

        serde_json::json!({
            "email": "password123",
            "possword": "123",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email 

    let test_case = 
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        });
    
    let response = app.post_signup(&test_case).await; // call `post_signup`
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input: {:?}",
        test_case
    );

    let expected_response = SignupResponse {
        message: "User created successfully!".to_string(),
    };

    let actual_response = response
        .json::<SignupResponse>()
        .await
        .expect("Could not deserialize response body to `SignupResponse`");
        
    assert_eq!(
        actual_response,
        expected_response,
        "A resposta JSON não bate com o esperado."
    );


    
}