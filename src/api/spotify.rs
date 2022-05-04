use super::utils;
use base64::encode;
use open;
use reqwest::blocking::Client;
use std::io::{stdin, stdout, Write};
use url::Url;
use utils::{SpotifyConfig, SpotifyTokenResponse};

const AUTH_URI: &str = "https://accounts.spotify.com";
const TOKEN_URI: &str = "https://accounts.spotify.com/api/token";

pub struct Spotify {
    config: SpotifyConfig,
    credentials: SpotifyTokenResponse,
    client: reqwest::blocking::Client,
}

// TODO: Improve .unwrap usages

impl Spotify {
    pub fn new() -> Self {
        Self {
            config: utils::parse_config("src/config.json").unwrap().spotify,
            credentials: SpotifyTokenResponse::default(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn update(&mut self, creds: SpotifyTokenResponse) {
        self.credentials = creds;
    }

    fn load_config_from_file(&mut self) {
        let cache_file = std::fs::File::open(".cache.json").unwrap();
        let json: SpotifyTokenResponse =
            serde_json::from_reader(cache_file).expect("Error while reading or parsing");
        self.update(json);
    }

    fn get_code(&self) -> String {
        let url = format!("{}/{}", AUTH_URI, "authorize?");
        let mut authorize_url: Url = Url::parse(url.as_str()).unwrap();

        // TODO: move config fields to &str (string slice)
        // instead of String, to aviod using .as_str()
        // NOTE: should the config struct own the string ?
        // IF so keep it with &str
        authorize_url.query_pairs_mut().extend_pairs([
            ("client_id", self.config.client_id.as_str()),
            ("redirect_uri", self.config.redirect_uri.as_str()),
            ("scope", self.config.scopes.join(" ").as_str()),
            ("response_type", String::from("code").as_str()),
        ]);

        // I want the program to crash if it can't execute the command
        open::that(authorize_url.to_string()).unwrap();

        let mut code: String = String::new();
        let mut input: String = String::new();
        print!("Please paste here the url you've been redirected to: ");

        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        let split: Vec<&str> = input.split("?code=").collect();

        code = match split.len() {
            2 => String::from(split[1]),
            _ => String::from(""),
        };
        // let code: String = String::from(split[1]);

        code
    }

    fn get_token(&mut self, code: String) {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", &code.strip_suffix("\n").unwrap_or(&code)),
            ("redirect_uri", self.config.redirect_uri.as_str()),
        ];

        let res = self
            .client
            .post(TOKEN_URI)
            .form(&params)
            .header(
                "Authorization",
                format!(
                    "Basic {}",
                    base64::encode(format!(
                        "{}:{}",
                        self.config.client_id.as_str(),
                        self.config.client_secret.as_str()
                    ))
                ),
            )
            .send()
            .unwrap();

        if let Ok(json) = res.json::<SpotifyTokenResponse>() {
            std::fs::write(".cache.json", serde_json::to_string_pretty(&json).unwrap()).unwrap();
            self.update(json);
        }
    }

    pub fn authenticate(&mut self) {
        // Before doing anything check if config file already exists

        match std::path::Path::new(".cache.json").exists() {
            true => self.load_config_from_file(),
            false => {
                let code = self.get_code();
                self.get_token(code);
            }
        };
    }
}
