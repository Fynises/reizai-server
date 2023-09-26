use chrono::Duration;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use jsonwebtoken as jwt;
use jwt::{EncodingKey, DecodingKey, Validation};

static SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET").expect("jwt secret not set in environment")
});

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    pub user_id: String,
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        Self {
            exp: (chrono::Utc::now() + Duration::hours(2)).timestamp(),
            user_id,
        }
    }
}

pub fn encode_new(user_id: String) -> Result<String> {
    Ok(jwt::encode(
        &jwt::Header::default(), 
        &Claims::new(user_id), 
        &EncodingKey::from_secret(SECRET.as_ref())
    )?)
}

pub fn decode(token: &str) -> Result<Claims> {
    let token = jwt::decode::<Claims>(
        token, 
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default()
    )?;
    Ok(token.claims)
}
