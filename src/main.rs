use api::Spotify;
use models::spotify::Playlist;

mod api;
pub mod models;

fn main() {
    let mut a = Spotify::new();
    a.authenticate();
    if let Ok(playlist) = a.get_playlist_from_id("6yNRwWCVVNLEsif1esJ1Cc") {
        println!("Got => {:?}", playlist);
    }
}
