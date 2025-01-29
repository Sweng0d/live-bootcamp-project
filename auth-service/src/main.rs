use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::domain::data_stores::UserStore;
use auth_service::Application;

#[tokio::main]
async fn main() {
    let store = HashmapUserStore::default();

    // 2) Converte para Arc<RwLock<dyn UserStore + ...>>
    let user_store = Arc::new(RwLock::new(store)) as Arc<RwLock<dyn UserStore + Send + Sync>>;

    // 3) Cria o `AppState` passando o trait object
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
