use reqwest::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum TwitchApiError {
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

impl IntoResponse for TwitchApiError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
