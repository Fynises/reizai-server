use async_trait::async_trait;
use database::twitch::auth::Auth as TwitchAuth;
use anyhow::{Result, anyhow};
use super::{error::TwitchApiError, api_auth::refresh_token};

#[async_trait]
pub trait TwitchApiRequest<T> {
    async fn run(&self, auth: &TwitchAuth) -> Result<T, TwitchApiError>;
}

pub async fn fetch<T> (
    twitch_auth: &mut TwitchAuth,
    callable: impl TwitchApiRequest<T>
) -> Result<T> {
    for _i in 0..3 { // 3 retries
        match callable.run(&twitch_auth).await {
            Ok(res) => return Ok(res),
            Err(TwitchApiError::Unauthorized) => {
                let token_res = refresh_token(&twitch_auth.refresh_token).await?;
                let _ = twitch_auth.refresh_token(&token_res.refresh_token, &token_res.access_token).await?;
                continue;
            },
            Err(e) => return Err(e.into())
        }
    }
    Err(anyhow!("max retries reached"))
}
