use std::{env, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use axum_extra::extract::CookieJar;
use serde::Serialize;
use tower_http::services::ServeDir;

// Ajuste: agora precisamos importar `EmailClientType` e um cliente de e-mail concreto:
use auth_service::{
    app_state::{AppState, UserStoreType, BannedTokenStoreType, TwoFACodeStoreType, EmailClientType},
    services::{
        hashmap_user_store::HashmapUserStore,
        hashset_banned_token_store::HashsetBannedTokenStore,
        hashmap_two_fa_code_store::HashmapTwoFACodeStore,
        mock_email_client::MockEmailClient,  // <--- se você tiver um mock de email
    },
    domain::data_stores::{UserStore, BannedTokenStore, TwoFACodeStore},
};

#[tokio::main]
async fn main() {
    // 1) Cria as instâncias concretas
    let user_store: UserStoreType = Arc::new(RwLock::new(HashmapUserStore::default()));
    // Use `HashsetBannedTokenStore::new()` em vez de `default()`
    let banned_token_store: BannedTokenStoreType = Arc::new(RwLock::new(HashsetBannedTokenStore::new()));
    let two_fa_code_store: TwoFACodeStoreType = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

    // 2) Crie o email_client (Mock, por exemplo)
    let email_client: EmailClientType = Arc::new(MockEmailClient);

    // 3) Monte o AppState com 4 parâmetros
    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,  // <-- quarto parâmetro
    );

    // 4) Crie o Router com estado
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(root))
        .route("/protected", get(protected))
        .with_state(app_state);

    // 5) Inicie o servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    login_link: String,
    logout_link: String,
}

async fn root() -> impl IntoResponse {
    let mut address = env::var("AUTH_SERVICE_IP").unwrap_or("localhost".to_owned());
    if address.is_empty() {
        address = "localhost".to_owned();
    }
    let login_link = format!("http://{}:3000", address);
    let logout_link = format!("http://{}:3000/logout", address);

    let template = IndexTemplate {
        login_link,
        logout_link,
    };
    Html(template.render().unwrap())
}

async fn protected(jar: CookieJar) -> impl IntoResponse {
    let jwt_cookie = match jar.get("jwt") {
        Some(cookie) => cookie,
        None => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let api_client = reqwest::Client::builder().build().unwrap();

    let verify_token_body = serde_json::json!({
        "token": &jwt_cookie.value(),
    });

    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or("0.0.0.0".to_owned());
    let url = format!("http://localhost:3000/verify-token");
    let response = match api_client.post(&url).json(&verify_token_body).send().await {
        Ok(response) => response,
        Err(e) => {
            println!("{}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match response.status() {
        reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::BAD_REQUEST => {
            StatusCode::UNAUTHORIZED.into_response()
        }
        reqwest::StatusCode::OK => Json(ProtectedRouteResponse {
            img_url: "https://i.ibb.co/YP90j68/Light-Live-Bootcamp-Certificate.png".to_owned(),
        })
        .into_response(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Serialize)]
pub struct ProtectedRouteResponse {
    pub img_url: String,
}
