use sqlx::types::chrono::{DateTime, Utc};
use anyhow::Result;
use crate::get_pool;

#[derive(Debug, sqlx::FromRow)]
pub struct Auth {
    user_id: String,
    refresh_token: Option<String>,
    token: Option<String>,
    last_authenticated: Option<DateTime<Utc>>,
}

impl Auth {
    pub async fn query_single(user_id: &String) -> Result<Self> {
        let user_auth = sqlx::query_as::<_, Self>(
            "
            SELECT user_id, refresh_token, token, last_authenticated 
            FROM twitch.auth 
            WHERE user_id = ?
            "
        ).bind(user_id)
        .fetch_one(get_pool()?).await?;
        Ok(user_auth)
    }

    pub async fn refresh_token(&self) -> Result<()> {
        todo!()
    }
}
