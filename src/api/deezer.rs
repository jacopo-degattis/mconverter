use super::utils;
use crate::models::DeezerConfig;
use std::io::{stdin, stdout, Write};
use url::Url;

const AUTH_URI: &str = "https://connect.deezer.com/oauth/";

pub struct Deezer {
    config: DeezerConfig,
    // credentials: DeezerTokenResponse,
    client: reqwest::blocking::Client,
}

impl Deezer {
    pub fn new() -> Self {
        Self {
            config: utils::parse_config("src/config.json").unwrap().deezer,
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get_code(&self) -> String {
        let url = format!("{}/{}", AUTH_URI, "auth.php");
        let mut authorize_url: Url = Url::parse(url.as_str()).unwrap();

        authorize_url.query_pairs_mut().extend_pairs([
            ("app_id", self.config.app_id.as_str()),
            ("redirect_uri", self.config.redirect_uri.as_str()),
            ("perms", self.config.scopes.join(" ").as_str()),
        ]);

        open::that(authorize_url.to_string()).unwrap();

        let mut input: String = String::new();
        print!("Plase paste here the url you've been redirected to: ");

        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        let split: Vec<&str> = input.split("?code=").collect();

        // TODO: I can directly return the match without assigning the result to a variable
        let code: String = match split.len() {
            2 => String::from(split[1]),
            _ => String::from(""),
        };

        code
    }

    fn get_token(&mut self, code: String) {
        let params = [
            ("app_id", self.config.app_id.as_str()),
            ("secret", self.config.app_secret.as_str()),
            ("code", code.strip_suffix("\n").unwrap_or(&code)),
            ("output", "json"),
        ];

        let url = format!("{}/{}", AUTH_URI, "access_token.php");
        let res = self.client.post(url).form(&params).send().unwrap();
        // if let Ok(json) = res.json::<DeezerToken>
    }

    pub fn authenticate(&mut self) {
        let code: String = self.get_code();
        self.get_token(code);
    }
}
