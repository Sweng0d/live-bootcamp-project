use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::domain::data_stores::UserStore;
use auth_service::Application;

#[tokio::main]
async fn main() {
    let store = HashmapUserStore::default();

    //compartilhar o UserStore em várias threads com segurança
    let user_store = Arc::new(RwLock::new(store)) as Arc<RwLock<dyn UserStore + Send + Sync>>;

    //Cria o `AppState` que guarda esse user_store
    let app_state = AppState::new(user_store);

    //Aqui você monta a aplicação em si, passando o AppState e a porta de rede.
    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    //Finalmente, a aplicação é iniciada e começa a aceitar requisições HTTP.
    app.run().await.expect("Failed to run app");
}
