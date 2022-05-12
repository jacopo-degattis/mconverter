use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Owner {
    pub id: usize,
    pub name: String,
    pub tracklist: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    pub id: usize,
    pub name: String,
    pub link: String,
    pub tracklist: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Album {
    pub id: usize,
    pub title: String,
    pub cover: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
    pub cover_xl: String,
    pub md5_image: String,
    pub tracklist: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Track {
    pub id: usize,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub link: String,
    pub duration: i32,
    pub rank: i32,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: i8,
    pub explicit_content_cover: i8,
    pub preview: String,
    pub md5_image: String,
    pub time_add: usize,
    pub artist: Artist,
    pub album: Album,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub data: Vec<Track>,
    pub checksum: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Playlist {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub duration: i32,
    pub public: bool,
    pub is_loved_track: bool,
    pub collaborative: bool,
    pub nb_tracks: i32,
    pub fans: i32,
    pub link: String,
    pub share: String,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub checksum: String,
    pub tracklist: String,
    pub creation_date: String,
    pub md5_image: String,
    pub picture_type: String,
    pub creator: Owner,
    pub tracks: Tracks,
}