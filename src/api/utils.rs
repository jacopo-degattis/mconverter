use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Error;

// TODO: is there a better way to define all the fields to be public without
// defining each line to do so ?

// TODO: move all struct in a file called types.rs ?

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub expires_in: i16,
}

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

pub fn parse_config(config_file: &str) -> Result<Config, Error> {
    match File::open(config_file) {
        Ok(content) => {
            let config: Config = serde_json::from_reader(content).unwrap();
            Ok(config)
        }
        Err(err) => Err(err),
    }
}
