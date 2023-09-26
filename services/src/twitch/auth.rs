use axum::{Router, routing::{post, get}, Json, TypedHeader, headers::{Authorization, authorization::Bearer}};
use serde::{Deserialize, Serialize};
use twitch::api::{self as api, api_users::UserRequest};
use database::twitch::auth as db;
use crate::error::ServiceError;
use super::jwt;

pub(crate) fn make_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/validate-login", get(validate))
}

#[derive(Debug, Deserialize)]
struct TwitchCallback {
    code: String,
    scope: String,
    state: String,
}

async fn login(
    Json(req): Json<TwitchCallback>
) -> Result<String, ServiceError> {
    log::info!("received request: {req:#?}");
    let auth_response = api::api_auth::login(req.code).await?;
    let user_details = api::api_users::fetch_by_token(&auth_response.access_token).await?.get_one()?;
    log::info!("got user details: {user_details:#?}");
    let _ = db::Auth::insert_login(&auth_response, &user_details).await?;
    Ok(jwt::encode_new(user_details.id)?)
}

#[derive(Debug, Serialize)]
struct LoginValidation {
    user_name: String,
    profile_picture: String,
}

async fn validate(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>
) -> Result<Json<LoginValidation>, ServiceError> {
    let claims = jwt::decode(auth.token())?;
    let mut twitch_auth = db::Auth::query_single(&claims.user_id).await?;
    let user_details = api::api_request::fetch(&mut twitch_auth, UserRequest::Token).await?
        .get_one()?;
    Ok(Json(LoginValidation {
        user_name: user_details.display_name,
        profile_picture: user_details.profile_image_url
    }))
}
