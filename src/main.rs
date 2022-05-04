use api::Spotify;

mod api;
pub mod models;

fn main() {
    let mut a = Spotify::new();
    a.authenticate();
    a.get_playlist_from_id("6yNRwWCVVNLEsif1esJ1Cc");
}
