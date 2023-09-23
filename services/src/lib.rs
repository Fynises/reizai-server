use axum::Router;
use tower_http::trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse};
use tracing::Level;


pub mod common;
pub mod twitch;
mod error;

pub fn create_app() -> Router {
    Router::new()
        .nest("/", common::make_common_routes())
        .layer(TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)))
}
