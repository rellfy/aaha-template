use crate::db_client::DbClient;
use crate::CONFIG;

#[derive(Clone)]
pub struct ServerState {
    pub db_client: DbClient,
}

impl ServerState {
    pub async fn from_env() -> eyre::Result<Self> {
        let db_client = DbClient::connect(&CONFIG.database_url).await?;
        Ok(Self { db_client })
    }
}
