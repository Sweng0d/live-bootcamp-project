use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{
    app_state::{AppState, BannedTokenStoreType, UserStoreType},
    domain::data_stores::UserStore,
    services::{
        hashmap_user_store::HashmapUserStore,
        hashset_banned_token_store::HashsetBannedTokenStore,
    },
    utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {
    //salvar nome de usuarios e compartilhar na memória
    let store = HashmapUserStore::default();

    //compartilhar o UserStore em várias threads com segurança
    let user_store = Arc::new(RwLock::new(store)) as Arc<RwLock<dyn UserStore + Send + Sync>>;

    //Parecido com user_store, mas é usado para guardar tokens banidos, prevenindo reuso de JWT, por exemplo.
    let banned_store: BannedTokenStoreType = Arc::new(RwLock::new(HashsetBannedTokenStore::new()));

    //Cria o `AppState` que guarda esse user_store
    let app_state = AppState::new(user_store, banned_store);

    //Aqui você monta a aplicação em si, passando o AppState e a porta de rede.
    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    //Finalmente, a aplicação é iniciada e começa a aceitar requisições HTTP.
    app.run().await.expect("Failed to run app");
}
