use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub(crate) enum TwitchApiError {
    #[error("json payload error")]
    JsonPayloadError(#[from] serde_json::Error),
    #[error("twitch api returned with 401")]
    Unauthorized,
    #[error("other error from twitch api")]
    ResponseError(reqwest::Error),
}

impl From<reqwest::Error> for TwitchApiError {
    fn from(value: reqwest::Error) -> Self {
        if let Some(code) = value.status() {
            match code {
                StatusCode::UNAUTHORIZED => return Self::Unauthorized,
                _ => ()
            };
            Self::ResponseError(value)
        } else {
            Self::ResponseError(value)
        }
    }
}

impl TwitchApiError {
    pub fn is_401(&self) -> bool {
        match self {
            Self::JsonPayloadError(_) => false,
            Self::Unauthorized => true,
            Self::ResponseError(_) => false,
        }
    }
}
