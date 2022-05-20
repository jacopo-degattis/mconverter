use super::utils;
use crate::models::deezer::{MyPlaylists, Playlist, QueryResults, Track};
use crate::models::DeezerConfig;
use crate::models::DeezerTokenResponse;
use reqwest::StatusCode;
use std::io::{stdin, stdout, Write};
use url::Url;

const API_URI: &str = "https://api.deezer.com/";
const AUTH_URI: &str = "https://connect.deezer.com/oauth/";

pub struct Deezer {
    config: DeezerConfig,
    credentials: DeezerTokenResponse,
    client: reqwest::blocking::Client,
}

impl Deezer {
    pub fn new() -> Self {
        Self {
            config: utils::parse_config("src/config.json").unwrap().deezer,
            credentials: DeezerTokenResponse::default(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn update(&mut self, creds: DeezerTokenResponse) {
        self.credentials = creds;
    }

    fn load_config_from_file(&mut self) {
        let cache_file = std::fs::File::open(".deez-cache.json").unwrap();
        let json: DeezerTokenResponse =
            serde_json::from_reader(cache_file).expect("Error while reading or parsing");
        self.update(json);
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
        ];

        // TODO: how to remove ?output=json from here ?
        // NOTE: in params it doesn't work... figure it out
        let url = format!("{}/{}?output=json", AUTH_URI, "access_token.php");
        let res = self.client.post(url).form(&params).send().unwrap();

        if let Ok(json) = res.json::<DeezerTokenResponse>() {
            std::fs::write(
                ".deez-cache.json",
                serde_json::to_string_pretty(&json).unwrap(),
            )
            .unwrap();
            self.update(json);
        }
    }

    pub fn authenticate(&mut self) {
        match std::path::Path::new(".deez-cache.json").exists() {
            true => self.load_config_from_file(),
            false => {
                let code: String = self.get_code();
                self.get_token(code);
            }
        }
    }

    pub fn search(&self, query: &str) -> Result<QueryResults, reqwest::Error> {
        let url = format!("{}/{}", API_URI, "search");
        let mut search_url: Url = Url::parse(url.as_str()).unwrap();

        search_url.query_pairs_mut().extend_pairs([("q", query)]);

        let res = self.client.get(search_url).send().unwrap();

        match res.json::<QueryResults>() {
            Ok(json) => Ok(json),
            Err(err) => Err(err),
        }
    }

    pub fn get_playlist_from_id(&self, id: &str) -> Result<Playlist, reqwest::Error> {
        let params = [("access_token", self.credentials.access_token.as_str())];
        let res = self
            .client
            .get(format!("{}/playlist/{}", API_URI, id))
            .form(&params)
            .send()
            .unwrap();
        
        let status_code = &res.status();
        
        println!("Status => {:?}", status_code);

        match res.json::<Playlist>() {
            Ok(json) => Ok(json),
            Err(err) => {
                Err(err)
            },
        }

    }

    pub fn get_playlist_by_name(&self, name: &str) -> usize {
        let mut playlist_id: usize = 0;
        let mut url: Url = Url::parse(format!("{}/user/me/playlists", API_URI).as_str()).unwrap();
        url.query_pairs_mut()
            .extend_pairs([("access_token", self.credentials.access_token.as_str())]);

        let res = self.client.get(url).send().unwrap();

        /* TODO: improve this condition using the below code
            let playlists: Vec<MyPlaylist> =
            serde_json::from_value(json["data"].take()).unwrap();
            Remove the MyPlaylists struct and just keep the MyPlaylist one
        */

        if let Ok(json) = res.json::<MyPlaylists>() {
            json.data.into_iter().for_each(|playlist| {
                if playlist.title == name {
                    playlist_id = playlist.id;
                }
            });
        }
        playlist_id
    }

    // TODO: recycle upper funtion to avoid redundant code
    // TODO: this function could also have been a request to playlist/<id>
    // and if the response was 404 then the playlists doesn't exists, DO THIS ?? OR NOT ??
    pub fn playlist_exists(&self, playlist_name: &str) -> bool {
        let mut url: Url = Url::parse(format!("{}/user/me/playlists", API_URI).as_str()).unwrap();
        url.query_pairs_mut()
            .extend_pairs([("access_token", self.credentials.access_token.as_str())]);

        let res = self.client.get(url).send().unwrap();

        match res.json::<MyPlaylists>() {
            Ok(json) => {
                let playlists = json
                    .data
                    .into_iter()
                    .map(|x| x.title)
                    .collect::<Vec<String>>();
                playlists.contains(&String::from(playlist_name))
            }
            Err(_) => false,
        }
    }

    pub fn create_playlist(&self, playlist_name: &str) -> i64 {
        if self.playlist_exists(playlist_name) {
            return 0;
        }

        let params = [("title", playlist_name)];
        let mut url: Url = Url::parse(format!("{}/user/me/playlists", API_URI).as_str()).unwrap();
        url.query_pairs_mut()
            .extend_pairs([("access_token", self.credentials.access_token.as_str())]);

        let res = self.client.post(url).form(&params).send().unwrap();

        match res.status() {
            StatusCode::OK => {
                let a = res.json::<serde_json::Value>().unwrap();
                a["id"].as_i64().unwrap()
            }
            _ => 0,
        }
    }

    pub fn get_tracks_from_playlist(&self, playlist_id: &str) -> Vec<Track> {
        match self.get_playlist_from_id(playlist_id) {
            Ok(playlist) => playlist.tracks.data,
            Err(_) => Vec::new(),
        }
    }

    pub fn add_tracks_to_playlists(&self, playlist_id: usize, tracks: Vec<String>) {
        // TODO: handle create playlist if it doesn't exists somewhere else

        let params = [("songs", tracks.join(","))];

        let mut url: Url =
            Url::parse(format!("{}/playlist/{}/tracks", API_URI, playlist_id).as_str()).unwrap();
        url.query_pairs_mut()
            .extend_pairs([("access_token", self.credentials.access_token.as_str())]);

        let res = self.client.post(url).form(&params).send().unwrap();

        println!("Status => {:?}", res.status());
    }
}
