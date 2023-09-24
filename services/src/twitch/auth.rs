use axum::{Router, routing::post, Json};
use serde::Deserialize;
use twitch::api as api;
use database::twitch::auth as db;
use crate::error::ServiceError;

pub(crate) fn make_router() -> Router {
    Router::new()
        .route("/login", post(login))
}

#[derive(Deserialize)]
struct TwitchCallback {
    code: String,
    scope: String,
    state: String,
}

async fn login(
    Json(req): Json<TwitchCallback>
) -> Result<String, ServiceError> {
    let auth_response = api::api_auth::login(req.code).await?;
    let user_details = api::api_users::fetch_by_token(&auth_response.access_token).await?.get_one()?;
    let _ = db::Auth::insert_login(auth_response, user_details).await?;

    todo!()
}
