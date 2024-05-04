use axum::Router;
use memory_serve::{load_assets, MemoryServe};
use tower_http::trace::TraceLayer;

mod index;
mod user;

pub fn router() -> Router {
    let memory_router = MemoryServe::new(load_assets!("assets")).into_router();
    Router::new()
        .merge(index::router())
        .merge(user::router())
        .layer(TraceLayer::new_for_http())
        .merge(memory_router)
}
