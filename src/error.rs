use askama_axum::axum_core::response::Response;
use axum::body::Body;
use axum::http;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

/// An error type that can be converted into an Axum response.
#[derive(Debug, Error)]
pub enum Error {
    #[error("internal server error")]
    ServerError(eyre::Report),
    #[error("invalid OTP code")]
    InvalidOtp,
    #[error("invalid email")]
    InvalidEmail,
}

pub type RouteResult<T> = Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let body = Body::from(self.to_string().into_bytes());
        http::Response::builder().status(status).body(body).unwrap()
    }
}

impl From<eyre::Report> for Error {
    fn from(value: eyre::Report) -> Self {
        Self::ServerError(value)
    }
}

impl<T> From<Error> for RouteResult<T> {
    fn from(value: Error) -> Self {
        Err(value)
    }
}
