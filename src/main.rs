use crate::routes::router;
pub use config::CONFIG;
use log::info;

mod config;
mod routes;
pub mod user;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let address = format!("0.0.0.0:{}", CONFIG.port);
    info!("running server @ http://{address}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}
