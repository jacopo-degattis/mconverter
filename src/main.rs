use api::{Deezer, Spotify};
use models::spotify::Playlist;
use std::io::{stdout, stdin, Write};
use regex::Regex;

mod api;
pub mod models;

fn main() {

    // TODO: add regex URL check

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

    // match re.is_match(&input) {
    //     true => {

    //     },
    //     false => {
    //         println!("Invalid url, please enter a valid one");
    //     }
    // }
    // let arr = &input.split("/playlist/").collect::<Vec<&str>>();

    // let mut a = Spotify::new();
    // let mut d = Deezer::new();

    // d.authenticate();
    // match d.get_playlist_from_id("9046456342") {
    //     Ok(data) => println!("Got => {:?}", data),
    //     Err(err) => println!("Err => {:?}", err)
    // };
    // a.authenticate();
    // let tracks: Vec<String> = a.get_tracks_from_playlist("6yNRwWCVVNLEsif1esJ1Cc");
    // println!("Tracks ids => {:?}", tracks);
}
