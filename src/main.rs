use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use askama_axum::Template;
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use log::info;
mod api;

#[derive(Template)]
#[template(path = "page/index.html")]
struct IndexPage<'a> {
    text: &'a str,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/api/user/auth", api::user::auth::route())
        .nest_service("/assets", ServeDir::new("assets"));
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    info!("running server http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> IndexPage<'static> {
    IndexPage {
        text: "hello!!!!!!!"
    }
}
