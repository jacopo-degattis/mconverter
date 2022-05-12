use api::{Deezer, Spotify};
use models::spotify::Playlist;
use std::io::{stdout, stdin, Write};
use regex::Regex;

mod api;
pub mod models;

fn main() {
    let mut deezer = Deezer::new();
    let mut spotify = Spotify::new();
    deezer.authenticate();
    spotify.authenticate();
    
    // // TODO: add regex URL check

    let mut input: String = String::new();

    // TODO: this way I only support spotify playlist, but it should
    // also support deezer's one
    let re = Regex::new(r"\bhttps?://[^/]*\bspotify\.com/playlist/(?P<playlist>[^\s?]+)").unwrap();

    print!("Playlist uri: ");

    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Please enter a valid URL");

    // TODO: imrove match condition here
    let playlist_id: &str = re.captures(&input).and_then(|cap| {
        cap.name("playlist").map(|p| p.as_str())
    }).unwrap();


    let tracks = spotify.get_tracks_from_playlist(playlist_id);
    
    println!("Tracks => {:?}", tracks);

}
