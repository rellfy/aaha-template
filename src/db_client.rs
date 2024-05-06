use crate::utils::now_unix;
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

    /// Applies an OTP, returning the matching user UUID if the OTP was valid.
    /// This creates a new user if it is their first time signing in.
    pub async fn apply_otp(&self, otp: u32, email: &str) -> eyre::Result<Option<Uuid>> {
        let row_opt = sqlx::query!(
            "
                WITH
                    new_user AS (
                        INSERT INTO public.user (created_timestamp_unix, email)
                        VALUES ($3, $1)
                        ON CONFLICT (email) DO NOTHING
                        RETURNING id, email
                    ),
                    existing_user AS (
                        SELECT id, email
                        FROM public.user
                        WHERE email = $1
                    ),
                    combined_user AS (
                        SELECT * FROM new_user
                        UNION
                        SELECT * FROM existing_user
                    ),
                    updated AS (
                        UPDATE public.otp
                        SET used_timestamp_unix = $3
                        WHERE (
                            value = $2 AND
                            user_email IN (SELECT email FROM combined_user) AND
                            used_timestamp_unix IS NULL AND
                            $3 <= exp_timestamp_unix
                        )
                        RETURNING user_email
                    )
                SELECT id
                FROM combined_user
                WHERE EXISTS (
                    SELECT 1 FROM updated
                    WHERE user_email = combined_user.email
                );
            ",
            email,
            otp_as_i32(otp),
            now_unix() as i64,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row_opt.and_then(|row| row.id))
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
            otp_as_i32(otp),
            email.to_string(),
            exp_timestamp_unix as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

fn otp_as_i32(otp: u32) -> i32 {
    i32::from_be_bytes(otp.to_be_bytes())
}
