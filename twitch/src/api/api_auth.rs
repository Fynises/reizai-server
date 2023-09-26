use anyhow::Result;
use common::http::HTTP_CLIENT;
use common::structs::twitch::twitch_auth_flow::TwitchTokenResponse;
use serde::Deserialize;
use crate::CONFIG;

pub async fn login(auth_code: String) -> Result<TwitchTokenResponse> {
    let params = [
        ("client_id", CONFIG.client_id.clone()),
        ("client_secret", CONFIG.secret.clone()),
        ("code", auth_code),
        ("grant_type", String::from("authorization_code")),
        ("redirect_uri", CONFIG.redirect.clone())
    ];

    let res = HTTP_CLIENT.post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await?;
    
    Ok(res.json::<TwitchTokenResponse>().await?)
}

#[derive(Deserialize)]
pub(super) struct TokenRefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}

pub(super) async fn refresh_token(token: &String) -> Result<TokenRefreshResponse> {
    let params = [
        ("client_id", CONFIG.client_id.clone()),
        ("client_secret", CONFIG.secret.clone()),
        ("grant_type", String::from("refresh_token")),
        ("refresh_token", token.clone())
    ];

    let res = HTTP_CLIENT.post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await?;
    
    Ok(res.json::<TokenRefreshResponse>().await?)
}
