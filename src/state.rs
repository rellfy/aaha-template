use crate::db_client::DbClient;


#[derive(Clone)]
pub struct ServerState {
    pub db_client: DbClient,
}

impl ServerState {
    pub async fn from_env() -> eyre::Result<Self> {
        let db_client = DbClient::connect().await;
        Ok(Self { db_client })
    }
}
