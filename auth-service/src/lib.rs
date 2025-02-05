use axum::{
    Router,          // Para o Router
    serve::Serve,    // Para o Serve (caso esteja usando axum::serve)
    http::StatusCode,
    routing::post,
    response::{IntoResponse, Response},
    Json,
};
use tower_http::{cors::CorsLayer, services::ServeDir};// Para o ServeDir
use std::error::Error;              // Para o Box<dyn Error>
use crate::routes::{signup, login, logout, verify_2fa, verify_token};
use crate::services::hashmap_user_store::HashmapUserStore; 
use crate::app_state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use crate::domain::error::AuthAPIError;
use axum::http::Method;

pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            // TODO: Replace [YOUR_DROPLET_IP] with your Droplet IP address
            "http://localhost:3000".parse()?,
        ];  

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/verify-2fa", post(verify_2fa))
        .route("/verify-token", post(verify_token))
        .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Self {server, address})
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }

}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            },
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED , "Unauthorized error")
            },
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing Token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid Token"),
            AuthAPIError::MalformedInput => (StatusCode::UNPROCESSABLE_ENTITY, "Malformed input"),
            
        };
        
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}