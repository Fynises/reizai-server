use async_trait::async_trait;
use anyhow::Result;
use database::twitch::auth::Auth as TwitchAuth;
use common::http::HTTP_CLIENT;
use common::structs::twitch::twitch_auth_flow::ApiGetUser;
use crate::CONFIG;

use super::api_request::TwitchApiRequest;
use super::error::TwitchApiError;

pub enum UserRequest {
    Name(String),
}

async fn fetch_by_name(
    name: &String,
    token: &String,
) -> Result<ApiGetUser, TwitchApiError> {
    let query = [("login", name)];
    let req = HTTP_CLIENT.get("https://api.twitch.tv/helix/users")
        .query(&query)
        .bearer_auth(token)
        .header("Client-Id", &CONFIG.client_id)
        .send()
        .await?;
    Ok(req.json::<ApiGetUser>().await?)
}

pub async fn fetch_by_token (
    access_token: &String,
) -> Result<ApiGetUser, TwitchApiError> {
    let req = HTTP_CLIENT.get("https://api.twitch.tv/helix/users")
        .bearer_auth(access_token)
        .header("Client-Id", &CONFIG.client_id)
        .send()
        .await?;
    Ok(req.json::<ApiGetUser>().await?)
}

#[async_trait]
impl TwitchApiRequest<ApiGetUser> for UserRequest {
    async fn run(&self, auth: &TwitchAuth) -> Result<ApiGetUser, TwitchApiError> {
        match self {
            UserRequest::Name(name) => fetch_by_name(&name, &auth.token).await,
        }
    }
}
