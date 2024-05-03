use askama::Template;
use axum::routing::get;
use axum::Router;

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomePage;

pub fn router() -> Router {
    Router::new().route("/", get(|| async move { HomePage }))
}
