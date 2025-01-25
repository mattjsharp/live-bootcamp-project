use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashmapUserStore, utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::new();
    let app_state = AppState::new(Arc::new(RwLock::new(user_store)));

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}