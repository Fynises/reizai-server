use common::structs::twitch::twitch_auth_flow::{TwitchTokenResponse, JsonUserData};
use sqlx::types::chrono::{DateTime, Utc};
use anyhow::Result;

use crate::get_pool;

#[derive(Debug, sqlx::FromRow)]
pub struct Auth {
    pub user_id: String,
    pub refresh_token: String,
    pub token: String,
    pub last_authenticated: Option<DateTime<Utc>>,
}

impl Auth {
    pub async fn query_single(user_id: &String) -> Result<Self> {
        let user_auth = sqlx::query_as::<_, Self>("
            SELECT user_id, refresh_token, token, last_authenticated 
            FROM twitch.auth 
            WHERE user_id = ?
        ").bind(user_id)
        .fetch_one(get_pool()?).await?;
        Ok(user_auth)
    }

    pub async fn insert_login(
        auth: TwitchTokenResponse,
        user: JsonUserData,
    ) -> Result<()> {
        //TODO: improve SQL query to only use one query
        let _ = sqlx::query("
            INSERT INTO twitch.user(user_id, user_login, cached_user_name)
            VALUES (?, ?, ?)
            ON CONFLICT(user_id) DO 
            UPDATE SET user_login = ?,
            cached_user_name = ?
        ")
        .bind(&user.id)
        .bind(&user.login)
        .bind(&user.display_name)
        .bind(&user.login)
        .bind(&user.display_name)
        .execute(get_pool()?).await?;

        let _ = sqlx::query("
            INSERT INTO twitch.auth(user_id, refresh_token, token, last_authenticated)
            VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            ON CONFLICT(user_id) DO
            UPDATE SET refresh_token = ?,
            token = ?,
            last_authenticated = CURRENT_TIMESTAMP
        ")
        .bind(&user.id)
        .bind(&auth.refresh_token)
        .bind(&auth.access_token)
        .bind(&auth.refresh_token)
        .bind(&auth.access_token)
        .execute(get_pool()?).await?;
        Ok(())
    }

    pub async fn refresh_token(&self) -> Result<()> {
        todo!()
    }
}
