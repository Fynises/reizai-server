use chrono::NaiveDateTime;
use common::structs::twitch::twitch_auth_flow::{TwitchTokenResponse, JsonUserData};
use anyhow::Result;

use crate::get_pool;

#[derive(Debug, sqlx::FromRow)]
pub struct Auth {
    pub user_id: String,
    pub refresh_token: String,
    pub token: String,
    pub last_authenticated: Option<NaiveDateTime>,
}

impl Auth {
    pub async fn query_single(user_id: &String) -> Result<Self> {
        let user_auth = sqlx::query_as!(Self, r#"
            SELECT user_id, refresh_token, token, last_authenticated 
            FROM twitch.auth
            WHERE user_id = $1"#,
            user_id
        ).fetch_one(get_pool()?).await?;
        Ok(user_auth)
    }

    pub async fn insert_login(
        auth: &TwitchTokenResponse,
        user: &JsonUserData,
    ) -> Result<()> {
        //TODO: improve SQL query to only use one query
        log::info!("refresh token: {auth:#?}");
        let _ = sqlx::query!(r#"
            INSERT INTO twitch.user(user_id, user_login, cached_user_name)
            VALUES($1, $2, $3)
            ON CONFLICT(user_id) DO 
            UPDATE SET 
            user_login = $2,
            cached_user_name = $3"#,
            &user.id,
            &user.login,
            &user.display_name
        ).execute(get_pool()?).await?;

        let _ = sqlx::query!(r#"
            INSERT INTO twitch.auth(user_id, refresh_token, token, last_authenticated) 
            VALUES ($1, $2, $3, CURRENT_TIMESTAMP) 
            ON CONFLICT(user_id) DO 
            UPDATE SET refresh_token = $2, 
            token = $3, 
            last_authenticated = CURRENT_TIMESTAMP"#,
            &user.id,
            &auth.refresh_token,
            &auth.access_token
        ).execute(get_pool()?).await?;
        Ok(())
    }

    pub async fn refresh_token(
        &self, 
        token: &String, 
        r_token: &String
    ) -> Result<()> {
        let _ = sqlx::query!(r#"
            UPDATE twitch.auth SET
            refresh_token = $1,
            token = $2
            WHERE user_id = $3"#,
            r_token,
            token,
            self.user_id
        ).execute(get_pool()?).await?;
        Ok(())
    }
}
