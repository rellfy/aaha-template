use crate::ServerState;
use askama::Template;
use axum::routing::get;
use axum::Router;

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomePage;

#[derive(Template)]
#[template(path = "pages/home.html", block = "page")]
struct HomePageContent;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", get(|| async move { HomePage }))
        .route("/home/content", get(|| async move { HomePageContent }))
}
