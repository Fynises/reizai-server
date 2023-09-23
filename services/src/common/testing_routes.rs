use axum::{Router, routing::get};

use crate::error::ServiceError;

pub fn make_testing_routes() -> Router {
    Router::new()
        .route("/test", get(hello_world))
}

async fn hello_world() -> Result<String, ServiceError> {
    Ok("hello world".to_string())
}