pub mod twitch_auth_flow {
    use serde::Deserialize;
    use anyhow::{Result, anyhow};

    #[derive(Debug, Deserialize)]
    pub struct TwitchTokenResponse {
        pub access_token: String,
        pub expires_in: usize,
        pub refresh_token: String,
        pub scope: Vec<String>,
        pub token_type: String,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct JsonUserData {
        pub id: String,
        pub login: String,
        pub display_name: String,
        pub r#type: String,
        pub broadcaster_type: String,
        pub description: String,
        pub profile_image_url: String,
        pub offline_image_url: String,
        pub view_count: Option<usize>,
    }

    #[derive(Debug, Deserialize)]
    pub struct ApiGetUser {
        pub data: Vec<JsonUserData>,
    }

    impl ApiGetUser {
        ///shortcut to get a single value from the api, assuming the api is working as intended
        pub fn get_one(self) -> Result<JsonUserData> {
            match self.data.len() {
                1 => Ok(self.data[0].clone()),
                _ => Err(anyhow!("data invalid size"))
            }
        }
    }

}