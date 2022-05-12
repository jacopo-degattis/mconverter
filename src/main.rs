use api::{Deezer, Spotify};
use models::spotify::Playlist;

mod api;
pub mod models;

fn main() {
    // let mut a = Spotify::new();
    let mut d = Deezer::new();

    d.authenticate();
    match d.get_playlist_from_id("9046456342") {
        Ok(data) => println!("Got => {:?}", data),
        Err(err) => println!("Err => {:?}", err)
    };
    // a.authenticate();
    // let tracks: Vec<String> = a.get_tracks_from_playlist("6yNRwWCVVNLEsif1esJ1Cc");
    // println!("Tracks ids => {:?}", tracks);
}
