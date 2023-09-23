use error::DbError;
use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres};
use anyhow::Result;

pub mod twitch;
mod error;

pub static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn init_db() -> Result<()> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = Pool::<Postgres>::connect(&url).await?;
    POOL.set(pool).expect("error connecting to database");
    Ok(())
}

/// shortcut function to get a reference to the pool
pub(crate) fn get_pool<'a>() -> Result<&'a Pool<Postgres>, DbError> {
    match POOL.get() {
        Some(p) => Ok(p),
        None => Err(DbError::PoolSetError),
    }
}
