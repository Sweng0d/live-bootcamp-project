use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use auth_service::{
    app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType, UserStoreType},
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore,
        hashset_banned_token_store::HashsetBannedTokenStore,
    },
    utils::constants::test,
    Application,
};
use auth_service::domain::data_stores::UserStore;
use tokio::sync::RwLock;
use std::sync::Arc;
use reqwest::cookie::Jar;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::app_state::EmailClientType;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_store: BannedTokenStoreType = Arc::new(RwLock::new(HashsetBannedTokenStore::new()));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_client: EmailClientType = Arc::new(MockEmailClient);

        let app_state = AppState::new(user_store.clone(), banned_store.clone(), two_fa_code_store.clone(), email_client.clone());

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread. 
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        // Create new `TestApp` instance and return it
        TestApp {
            address,
            cookie_jar,
            http_client,
            user_store,
            banned_token_store: banned_store,
            two_fa_code_store: two_fa_code_store.clone(),
        }

    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

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

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response where Body: serde::Serialize,{
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
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