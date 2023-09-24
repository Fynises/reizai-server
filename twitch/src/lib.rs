use once_cell::sync::Lazy;
pub mod api;
mod chat;

pub(crate) static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config {
        client_id: std::env::var("TWITCH_CLIENT_ID").expect("failed to load"),
        secret: std::env::var("TWITCH_SECRET").expect("failed to load "),
        redirect: std::env::var("TWITCH_REDIRECT").expect("failed to load")
    }
});

#[derive(Debug)]
pub(crate) struct Config {
    pub client_id: String,
    pub secret: String,
    pub redirect: String,
}