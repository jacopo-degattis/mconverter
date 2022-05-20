use super::utils;
use crate::models::spotify::{Playlist, Track, TrackInfo, MyPlaylists};
use crate::models::SpotifyConfig;
use crate::models::SpotifyTokenResponse;
use open;
use std::io::{stdin, stdout, Write};
use url::Url;
use reqwest::StatusCode;
use std::collections::HashMap;

const API_URI: &str = "https://api.spotify.com/v1";
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

        let mut input: String = String::new();
        print!("Please paste here the url you've been redirected to: ");

        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        let split: Vec<&str> = input.split("?code=").collect();

        let code: String = match split.len() {
            2 => String::from(split[1]),
            _ => String::from(""),
        };

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

    pub fn get_playlist_from_id(&self, id: &str) -> Result<Playlist, reqwest::Error> {
        let res = self
            .client
            .get(format!("{}/playlists/{}", API_URI, id))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.credentials.access_token),
            )
            .send()
            .unwrap();

        // TODO: add authorization header by default in request without
        // having to specify it each time i make a request

        // TODO: right now type is not implemented because it is a reserved
        // keyword... is it necessary to add it ? Add 'Option' to all the fields
        // that in some case could be empty

        match res.json::<Playlist>() {
            Ok(json) => Ok(json),
            Err(err) => Err(err),
        }
    }

    pub fn get_playlist_by_name(&self, playlist_name: &str) -> String {
        let res = self
            .client
            .get(format!("{}/me/playlists", API_URI))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.credentials.access_token),
            )
            .send()
            .unwrap();

        let mut data = res.json::<serde_json::Value>().unwrap();

        // println!("Got => {:?}", data["items"]);

        let tmp = data["items"].as_array().unwrap();
        // let tmp = data.get("items").unwrap();
        // let a: Vec<Playlist> = serde_json::from_value(tmp).unwrap();
        
        for pl in tmp {
            if pl["name"].eq(playlist_name) {
                return String::from(pl["id"].as_str().unwrap())
            }
        }

        return String::from("")
    }

    pub fn playlist_exists(&self, playlist_name: &str) -> bool {
        self.get_playlist_by_name(playlist_name).len() > 0
    }

    pub fn get_tracks_from_playlist(&self, id: &str) -> Vec<Track> {
        match self.get_playlist_from_id(id) {
            // Ok(playlist) => playlist
            //     .tracks
            //     .items
            //     .into_iter()
            //     .map(|x| x.track.id)
            //     .collect::<Vec<String>>(),
            Ok(playlist) => playlist.tracks.items,
            Err(_) => Vec::new(),
        }
    }


    pub fn search(&self, query: &str) -> Result<Vec<TrackInfo>, serde_json::Error> {
        let url = format!("{}/{}", API_URI, "search");
        let mut search_url: Url = Url::parse(url.as_str()).unwrap();

        search_url.query_pairs_mut().extend_pairs([("q", query), ("type", "track")]);

        let res = self
            .client
            .get(search_url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization", 
                format!("Bearer {}", self.credentials.access_token)
            )
            .send()
            .unwrap();

        let mut tracks_array = res.json::<serde_json::Value>().unwrap();        
        
        // println!("Value, {:?}", res.text());

        match serde_json::from_value(tracks_array["tracks"]["items"].take()) {
            Ok(json) => Ok(json),
            Err(err) => Err(err)
        }
        // unimplemented!();
    }
}
