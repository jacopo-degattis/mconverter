use api::Spotify;
use models::spotify::Playlist;

mod api;
pub mod models;

fn main() {
    let mut a = Spotify::new();
    a.authenticate();
    let tracks: Vec<String> = a.get_tracks_from_playlist("6yNRwWCVVNLEsif1esJ1Cc");

    println!("Tracks ids => {:?}", tracks);
}
