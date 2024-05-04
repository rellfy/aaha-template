use uuid::Uuid;

// TODO: implement with SQLx.
#[derive(Debug, Clone)]
pub struct DbClient;

impl DbClient {
    pub async fn connect() -> Self {
        Self
    }

    pub async fn fetch_user_uuid_by_email(&self, _email: &str) -> eyre::Result<Option<Uuid>> {
        Ok(None)
    }

    pub async fn apply_otp(&self, _otp: u32, _email: &str) -> eyre::Result<Option<Uuid>> {
        Ok(Some(Uuid::new_v4()))
    }

    pub async fn store_otp(
        &self,
        _otp: u32,
        _email: &str,
        _exp_timestamp_unix: u64,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
