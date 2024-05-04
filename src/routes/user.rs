use crate::error::{Error, RouteResult};
use crate::user::jwt::generate_jwt;
use crate::user::otp::Otp;
use crate::utils::now_unix;
use crate::ServerState;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::Router;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};

#[derive(Template)]
#[template(path = "pages/user.html")]
struct UserPage<'a> {
    user_name: &'a str,
}

#[derive(Template)]
#[template(path = "pages/user.html", block = "page")]
struct UserPageContent<'a> {
    user_name: &'a str,
}

#[derive(TryFromMultipart)]
struct BeginAuthForm {
    email: String,
}

#[derive(TryFromMultipart)]
struct CompleteAuthForm {
    otp: String,
    email: String,
}

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/user", get(handle_get))
        .route("/user/content", get(handle_get_content))
        .route("/user/auth", put(handle_put_auth))
        .route("/user/auth", post(handle_post_auth))
}

async fn handle_get() -> UserPage<'static> {
    UserPage { user_name: "n/a" }
}

async fn handle_get_content() -> UserPageContent<'static> {
    UserPageContent { user_name: "n/a" }
}

async fn handle_put_auth(
    state: State<ServerState>,
    form: TypedMultipart<BeginAuthForm>,
) -> RouteResult<impl IntoResponse> {
    let otp = Otp::random();
    let exp_timestamp_unix = now_unix() + (5 * 60);
    state
        .db_client
        .store_otp(otp.as_u32(), &form.email, exp_timestamp_unix)
        .await?;
    Ok(format!(
        "otp created: {otp} (in a real app this would be an email)"
    ))
}

async fn handle_post_auth(
    state: State<ServerState>,
    form: TypedMultipart<CompleteAuthForm>,
) -> RouteResult<impl IntoResponse> {
    let Some(otp) = Otp::from_str(&form.otp) else {
        return Error::InvalidOtp.into();
    };
    let Some(uuid) = state.db_client.apply_otp(otp.as_u32(), &form.email).await? else {
        return Error::InvalidOtp.into();
    };
    let jwt = generate_jwt(&uuid)?;
    Ok(format!("{jwt}"))
}
