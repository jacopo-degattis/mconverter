use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
}

#[derive(Deserialize, Debug)]
pub struct DeezerConfig {
    pub app_id: String,
    pub app_secret: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub spotify: SpotifyConfig,
    pub deezer: DeezerConfig,
}
