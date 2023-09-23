use once_cell::sync::OnceCell;
use anyhow::{Result, anyhow};
use redis::aio::ConnectionManager;

pub(crate) static REDIS_CM: OnceCell<ConnectionManager> = OnceCell::new();

pub async fn init_redis() -> Result<()> {
    let client = redis::Client::open(std::env::var("REDIS_URL")?)?;
    let conn_manager = ConnectionManager::new(client).await?;
    match REDIS_CM.set(conn_manager) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(anyhow!("error setting redis connection manager to OnceCell")),
    }
}
