use crate::helpers::{TestApp, SignupData, LoginData, TwoFaData, TokenData, get_random_email};
use auth_service::{routes::SignupResponse, ErrorResponse};


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

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters
    let app = TestApp::new().await;

    let invalid_inputs = vec![
        serde_json::json!({ "email": "", "password": "password123", "requires2FA": true }),
        serde_json::json!({ "email": "wrongemail.com", "password": "password123", "requires2FA": true }),
        serde_json::json!({ "email": "user@domain", "password": "short", "requires2FA": true }),
    ];

    for invalid_payload in invalid_inputs.iter() {
        let response = app.post_signup(invalid_payload).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            400,
            "Expected 400 to payload: {:?}",
            invalid_payload
        );

        let error_response = response
            .json::<ErrorResponse>()
            .await
            .expect("Não foi possível desserializar o corpo em ErrorResponse");

        // Verifique se a mensagem de erro bate com a esperada
        assert_eq!(
            error_response.error,
            "Invalid credentials",
            "Mensagem de erro incorreta para payload: {:?}",
            invalid_payload
        );
    }
    
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    // Call the signup route twice. The second request should fail with a 409 HTTP status code    
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email 

    let valid_payload = 
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        });

    let _ = app.post_signup(&valid_payload).await; // first try, should success

    let response = app.post_signup(&valid_payload).await;

    assert_eq!(response.status().as_u16(), 409);

    let error_response = response
        .json::<ErrorResponse>()
        .await
        .expect("Não foi possível desserializar o corpo em ErrorResponse");
    
    assert_eq!(
        error_response.error,
        "User already exists"
    );

}