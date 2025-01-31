use auth_service::Application;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let app_state = AppState::new(user_store);

        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread. 
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new(); // Create a Reqwest http client instance

        // Create new `TestApp` instance and return it
        TestApp {
            address,
            http_client
        }

    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // TODO: Implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)
    pub async fn signup(&self, data: &SignupData) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(data)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body:&Body) -> reqwest::Response 
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            //.json(data)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_2fa(&self, data: &TwoFaData) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(data)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_token(&self, data: &TokenData) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(data)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFaData {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub token: String,
}