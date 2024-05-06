use sqlx::postgres::PgPoolOptions;
use sqlx::{Error as SqlxError, Pool, Postgres};
use uuid::Uuid;

const MAX_POOL_CONNECTIONS: u32 = 10;

#[derive(Debug, Clone)]
pub struct DbClient {
    pool: Pool<Postgres>,
}

// This is defined to allow the consumer crate to have exposure to the sqlx error type.
pub type DbError = SqlxError;

impl DbClient {
    pub async fn connect(connection_string: &str) -> Result<Self, DbError> {
        let pool = PgPoolOptions::new()
            .max_connections(MAX_POOL_CONNECTIONS)
            .connect(connection_string)
            .await?;
        Ok(Self { pool })
    }

    pub async fn fetch_user_uuid_by_email(&self, _email: &str) -> eyre::Result<Option<Uuid>> {
        Ok(None)
    }

    pub async fn apply_otp(&self, _otp: u32, _email: &str) -> eyre::Result<Option<Uuid>> {
        Ok(Some(Uuid::new_v4()))
    }

    pub async fn store_otp(
        &self,
        otp: u32,
        email: &str,
        exp_timestamp_unix: u64,
    ) -> eyre::Result<()> {
        sqlx::query!(
            "
                INSERT INTO public.otp (
                    value,
                    user_email,
                    exp_timestamp_unix
                )
                VALUES (
                    $1, $2, $3
                )
            ",
            i32::from_be_bytes(otp.to_be_bytes()),
            email.to_string(),
            exp_timestamp_unix as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
