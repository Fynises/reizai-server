use axum::response::IntoResponse;
use axum::http::StatusCode as SC;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("unhandled anyhow error encountered")]
    Anyhow(#[from] anyhow::Error), //500
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServiceError::Anyhow(_) => SC::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
