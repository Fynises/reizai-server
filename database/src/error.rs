#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("database pool was not set")]
    PoolSetError,
}
