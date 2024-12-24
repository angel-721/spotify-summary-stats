use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use std::env;

pub fn spotify_client() -> AuthCodeSpotify {
    let spotify_client_id =
        env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not found in .env");

    let spotify_client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not found in .env");
    let oauth = OAuth {
        scopes: scopes!(
            "user-read-currently-playing",
            "user-read-private",
            "user-read-private",
            "playlist-modify-private",
            "user-top-read"
        ),
        redirect_uri: "http://localhost:3000/callback".to_string(),
        ..Default::default()
    };

    let creds = Credentials::new(&spotify_client_id, &spotify_client_secret);
    let config = rspotify::Config::default();
    let client = AuthCodeSpotify::with_config(creds, oauth, config);
    client
}
