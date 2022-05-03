use api::Spotify;

mod api;

fn main() {
    let a = Spotify::new();
    a.authenticate();
}
