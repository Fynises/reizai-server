use anyhow::Result;
use common::http::HTTP_CLIENT;
use common::structs::twitch::twitch_auth_flow::TwitchTokenResponse;
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