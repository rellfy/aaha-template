use askama::Template;
use axum::routing::get;
use axum::Router;

#[derive(Template)]
#[template(path = "pages/user.html")]
struct UserPage<'a> {
    user_name: &'a str,
}

pub fn router() -> Router {
    Router::new().route("/user", get(handle_get))
}

async fn handle_get() -> UserPage<'static> {
    UserPage { user_name: "n/a" }
}
