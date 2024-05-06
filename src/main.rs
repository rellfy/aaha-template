use crate::routes::router;
pub use config::CONFIG;
use log::info;
pub use state::ServerState;

mod config;
pub mod db_client;
pub mod email;
pub mod error;
mod routes;
mod state;
pub mod user;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let state = ServerState::from_env()
        .await
        .expect("failed to fetch server state");
    let address = format!("0.0.0.0:{}", CONFIG.port);
    info!("running server @ http://{address}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    let service = router(state);
    axum::serve(listener, service).await.unwrap();
}
