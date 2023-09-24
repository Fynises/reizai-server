use axum::Router;

pub mod auth;

pub (crate) fn make_twitch_routes() -> Router {
    Router::new()
        .nest("/auth", auth::make_router())
}