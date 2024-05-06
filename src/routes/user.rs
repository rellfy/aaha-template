use crate::error::{Error, RouteResult};
use crate::user::jwt::{generate_jwt, validate_jwt};
use crate::user::otp::Otp;
use crate::utils::now_unix;
use crate::ServerState;
use askama::Template;
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::{get, post, put};
use axum::Router;
use axum_extra::extract::CookieJar;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use log::error;

struct UserView {
    uuid: String,
    jwt: String,
}

struct OtpResponse {
    otp: String,
    email: String,
}

#[derive(Template)]
#[template(path = "pages/user.html")]
struct UserPage {
    user: Option<UserView>,
    otp_response: Option<OtpResponse>,
}

#[derive(Template)]
#[template(path = "pages/user.html", block = "page")]
struct UserPageContent {
    user: Option<UserView>,
    otp_response: Option<OtpResponse>,
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

async fn handle_get(jar: CookieJar) -> UserPage {
    let jwt_opt = jar.get("auth").map(|cookie| cookie.value().to_string());
    let Some(jwt) = jwt_opt else {
        return UserPage {
            user: None,
            otp_response: None,
        };
    };
    let result = validate_jwt(&jwt.to_string());
    let Ok(uuid) = result else {
        error!("failed to validate jwt: {result:#?}");
        return UserPage {
            user: None,
            otp_response: None,
        };
    };
    let user_view = UserView {
        uuid: uuid.to_string(),
        jwt: jwt.to_string(),
    };
    return UserPage {
        user: Some(user_view),
        otp_response: None,
    };
}

async fn handle_get_content(jar: CookieJar) -> UserPageContent {
    let jwt_opt = jar.get("auth").map(|cookie| cookie.value().to_string());
    let Some(jwt) = jwt_opt else {
        return UserPageContent {
            user: None,
            otp_response: None,
        };
    };
    let result = validate_jwt(&jwt.to_string());
    let Ok(uuid) = result else {
        error!("failed to validate jwt: {result:#?}");
        return UserPageContent {
            user: None,
            otp_response: None,
        };
    };
    let user_view = UserView {
        uuid: uuid.to_string(),
        jwt: jwt.to_string(),
    };
    return UserPageContent {
        user: Some(user_view),
        otp_response: None,
    };
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
    Ok(UserPageContent {
        user: None,
        otp_response: Some(OtpResponse {
            otp: otp.to_string(),
            email: form.email.clone(),
        }),
    })
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
    // The path is set so that the cookie can be cleared from the client side
    // when logging out the user.
    let auth_cookie = format!("auth={jwt}; Path=/");
    let headers = AppendHeaders([(SET_COOKIE, auth_cookie)]);
    let content = UserPageContent {
        user: Some(UserView {
            uuid: uuid.to_string(),
            jwt,
        }),
        otp_response: None,
    };
    Ok((headers, content))
}
