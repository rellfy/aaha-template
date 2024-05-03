use axum::Router;
use memory_serve::{load_assets, MemoryServe};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod index;
mod user;

pub fn router() -> Router {
    let memory_router = MemoryServe::new(load_assets!("assets")).into_router();
    Router::new()
        .merge(index::router())
        .merge(user::router())
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets", ServeDir::new("assets"))
        .merge(memory_router)
}
