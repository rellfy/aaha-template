use log::error;
use once_cell::sync::Lazy;
use serde::de::StdError;
use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;
use thiserror::Error;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::from_env().unwrap());

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_JWT_EXPIRATION_MINUTES: u64 = 1440;

pub struct Config {
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiration_minutes: u64,
}

impl Config {
    pub fn from_env() -> eyre::Result<Self> {
        Ok(Self {
            port: get_var("PORT").unwrap_or(DEFAULT_PORT),
            jwt_secret: get_var("JWT_SECRET")?,
            jwt_expiration_minutes: get_var("JWT_EXPIRATION_MINUTES")
                .unwrap_or(DEFAULT_JWT_EXPIRATION_MINUTES),
        })
    }
}

#[derive(Error, Debug)]
enum EnvError {
    #[error("{0}: {1}")]
    Var(String, VarError),
    #[error("{0}: failed to parse into type")]
    Parse(String),
}

/// Fetches and parses an environment variable into type T.
fn get_var<T>(var: &str) -> eyre::Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + StdError + Send + Sync + 'static,
{
    Ok(env::var(var)
        .map_err(|e| EnvError::Var(var.to_owned(), e))?
        .parse::<T>()
        .map_err(|_| EnvError::Parse(var.to_owned()))?)
}
