use axum::{Router, http::HeaderValue};
use tower_http::{trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse}, cors::{CorsLayer, Any}};
use tracing::Level;
use anyhow::Result;

pub mod common;
pub mod twitch;
mod error;

pub fn create_app() -> Result<Router> {

    let cors_layer = CorsLayer::new()
        .allow_origin(std::env::var("CORS_ALLOW_ORIGIN")?.as_str().parse::<HeaderValue>()?)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .nest("/", common::make_common_routes())
        .nest("/twitch", twitch::make_twitch_routes())
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)));

    Ok(router)
}
