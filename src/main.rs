use api::{Deezer, Spotify};
use models::deezer::QueryResults;
use models::spotify::Playlist;
use regex::Regex;
use std::io::{stdin, stdout, Write};

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
    let playlist_id: &str = re
        .captures(&input)
        .and_then(|cap| cap.name("playlist").map(|p| p.as_str()))
        .unwrap();

    let tracks = spotify.get_tracks_from_playlist(playlist_id);
    let target_playlist = spotify.get_playlist_from_id(playlist_id).unwrap();

    let mut item_ids: Vec<String> = Vec::new();

    for track in tracks {
        if let Ok(result) = deezer.search(
            format!(
                "artist:'{}' track:'{}'",
                track.track.artists[0].name, track.track.name
            )
            .as_str(),
        ) {
            match result.data.len() > 0 {
                true => item_ids.push(result.data[0].id.to_string()),
                _ => {
                    // Track Not found
                }
            };
        };
    }

    if !deezer.playlist_exists(target_playlist.name.as_str()) {
        let id = deezer.create_playlist(target_playlist.name.as_str());
    } else {
        // let id = deezer.
    }

    // deezer.add_tracks_to_playlists(10344668222, item_ids);

    // println!("Got tracks ids => {:?}", item_ids);
}
