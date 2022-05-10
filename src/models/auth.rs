use serde::{Deserialize, Serialize};

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

impl Default for SpotifyTokenResponse {
    fn default() -> SpotifyTokenResponse {
        SpotifyTokenResponse {
            access_token: String::from(""),
            refresh_token: String::from(""),
            scope: String::from(""),
            token_type: String::from(""),
            expires_in: 0,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeezerTokenResponse {
    access_token: String,
    expires: i16,
}

impl Default for DeezerTokenResponse {
    fn default() -> DeezerTokenResponse {
        DeezerTokenResponse {
            access_token: String::from(""),
            expires: 0
        }
    }
}
