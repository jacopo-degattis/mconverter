use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Followers {
    pub href: Option<String>,
    pub total: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Images {
    // TODO: url could be directly a URL type ?
    url: String,
    height: Option<i16>,
    width: Option<i16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Owner {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    // pub _type: String,
    pub uri: String,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    // pub type: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Album {
    pub album_type: String,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Images>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: i16,
    // pub _type: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExternalIds {
    pub isrc: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrackInfo {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: i8,
    pub duration_ms: i32,
    pub episode: bool,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub name: String,
    pub popularity: i16,
    pub preview_url: Option<String>,
    pub track: bool,
    pub track_number: i16,
    // pub _type: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VideoThumbnail {
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OwnerX {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    // pub _type: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Track {
    pub added_at: String,
    pub added_by: OwnerX,
    pub is_local: bool,
    pub primary_color: Option<String>,
    pub track: TrackInfo,
    pub video_thumbnail: VideoThumbnail,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tracks {
    pub href: String,
    pub items: Vec<Track>,
    pub limit: i16,
    pub next: Option<String>,
    pub offset: i16,
    pub previous: Option<String>,
    pub total: i16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Playlist {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Images>,
    pub name: String,
    pub owner: Owner,
    pub public: bool,
    pub snapshot_id: String,
    pub tracks: Tracks,
    // pub _type: String,
    pub uri: String,
}
