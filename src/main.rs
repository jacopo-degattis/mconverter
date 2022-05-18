use api::{Deezer, Spotify};
use models::deezer::QueryResults;
use models::spotify::Playlist;
use regex::Regex;
use std::io::{stdin, stdout, Write};

mod api;
pub mod models;

fn spot_to_deezer(spotify: &Spotify, deezer: &Deezer, playlist_id: &str) {
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
    
    match deezer.playlist_exists(target_playlist.name.as_str()) {
        false => {
            let playlist_id = deezer.create_playlist(target_playlist.name.as_str()) as usize;
            deezer.add_tracks_to_playlists(playlist_id, item_ids);
        }
        true => {
            print!("Warning, a playlist with this name already exists ! Do you want to merge the songs ? (y/n): ");
            let mut choice: String = String::new();
            let _ = stdout().flush();
            stdin()
                .read_line(&mut choice)
                .expect("Please enter a valid response");

            if choice.trim().eq("y") {
                let playlist_id = deezer.get_playlist_by_name(target_playlist.name.as_str());
                deezer.add_tracks_to_playlists(playlist_id, item_ids);
            } else {
                println!("Not merging the playlists...");
            }
        }
    } 
}

fn deezer_to_spot(spotify: &Spotify, deezer: &Deezer, playlist_id: &str) {
    let tracks = deezer.get_tracks_from_playlist(playlist_id);
    let target_playlist = deezer.get_playlist_from_id(playlist_id);

    let mut item_ids: Vec<String> = Vec::new();

    println!("HERE 1");

    for track in tracks {

        // FIXME: is broken
        match spotify.search(
            format!(
                "artist:{}+track:{}",
                track.artist.name,
                track.title
            )
            .as_str()
        ) {
            Ok(result) => println!("Got => {:?}", result),
            Err(err) => println!("Error => {:?}", err)
        };
    }
}

fn main() {
    let mut deezer = Deezer::new();
    let mut spotify = Spotify::new();
    deezer.authenticate();
    spotify.authenticate();
    // // TODO: add regex URL check
    let mut input: String = String::new();

    // TODO: this way I only support spotify playlist, but it should
    // also support deezer's one
    let spotify_re = Regex::new(r"\bhttps?://[^/]*\bspotify\.com/playlist/(?P<playlist>[^\s?]+)").unwrap();
    let deezer_re = Regex::new(r"\bhttps?://[^/]*\bdeezer\.com/[a-z]{2}/playlist/(?P<playlist>[^\s?]+)").unwrap();

    print!("Playlist uri: ");

    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Please enter a valid URL");

    // TODO: imrove match condition here
    if let Some(playlist_id) = spotify_re
        .captures(&input)
        .and_then(|cap| cap.name("playlist").map(|p| p.as_str())) {
            println!("From spotify to deezer...");
            spot_to_deezer(&spotify, &deezer, playlist_id);
        }

    if let Some(playlist_id) = deezer_re
        .captures(&input)
        .and_then(|cap| cap.name("playlist").map(|p| p.as_str())) {
            println!("From deezer to spotify...");
            deezer_to_spot(&spotify, &deezer, playlist_id);
        }
}
