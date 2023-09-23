use anyhow::Result;
use common::HTTP_CLIENT;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TwitchTokenResponse {
    access_token: String,
    expires_in: usize,
    refresh_token: String,
    scope: Vec<String>,
    token_type: String,
}

pub async fn login(auth_code: String) -> Result<TwitchTokenResponse> {
    let params = [
        ("client_id", std::env::var("TWITCH_CLIENT_ID")?),
        ("client_secret", std::env::var("TWITCH_CLIENT_SECRET")?),
        ("code", auth_code),
        ("grant_type", String::from("authorization_code")),
        ("redirect_uri", std::env::var("TWITCH_AUTH_REDIRECT")?)
    ];

    let res = HTTP_CLIENT.post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await?;
    
    Ok(res.json::<TwitchTokenResponse>().await?)
}