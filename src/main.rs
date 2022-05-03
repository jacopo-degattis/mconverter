use api::Spotify;

mod api;

fn main() {
    let mut a = Spotify::new();
    a.authenticate();
}
