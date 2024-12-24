use crate::types::song::Song;

use rspotify::{
    clients::OAuthClient, model::TimeRange, scopes, AuthCodeSpotify, Credentials, OAuth,
};
use std::env;
use std::sync::Arc;

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

pub async fn get_top_songs(client: &Arc<AuthCodeSpotify>) -> Vec<Song> {
    let top_tracks = client
        .current_user_top_tracks_manual(Some(TimeRange::ShortTerm), Some(10), Some(0))
        .await
        .unwrap()
        .items;

    let top_tracks: Vec<Song> = top_tracks
        .iter()
        .filter_map(|track| {
            Some(Song {
                name: track.name.clone(),
                artist_name: track.artists[0].name.clone(),
                song_image_uri: track.album.images[0].url.clone(),
            })
        })
        .collect();

    top_tracks
}
