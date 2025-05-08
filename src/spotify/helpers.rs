use crate::types::song::Song;

use chrono::Utc;
use rspotify::{
    clients::OAuthClient,
    model::{TimeLimits, TimeRange},
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use std::sync::Arc;

pub fn spotify_client(spotify_client_id: &str, spotify_client_secret: &str) -> AuthCodeSpotify {
    // let spotify_client_id = match env::var("SPOTIFY_CLIENT_ID") {
    //     Ok(id) => id,
    //     Err(_) => panic!("SPOTIFY_CLIENT_ID not found in .env"),
    // };
    //
    // let spotify_client_secret = match env::var("SPOTIFY_CLIENT_SECRET") {
    //     Ok(secret) => secret,
    //     Err(_) => panic!("SPOTIFY_CLIENT_SECRET not found in .env"),
    // };
    let oauth = OAuth {
        scopes: scopes!(
            "user-read-currently-playing",
            "user-read-private",
            "user-read-private",
            "playlist-modify-private",
            "user-top-read"
        ),
        redirect_uri: "http://127.0.0.1:8000/callback".to_string(),
        ..Default::default()
    };

    let creds = Credentials::new(&spotify_client_id, &spotify_client_secret);
    let config = rspotify::Config::default();

    AuthCodeSpotify::with_config(creds, oauth, config)
}

pub async fn get_top_songs(client: &Arc<AuthCodeSpotify>, time_range: TimeRange) -> Vec<Song> {
    let top_tracks = match client
        .current_user_top_tracks_manual(Some(time_range), Some(10), Some(0))
        .await
    {
        Ok(response) => response.items,
        Err(e) => {
            eprintln!("Error fetching top tracks: {}", e);
            Vec::new() // or handle error differently
        }
    };

    let top_tracks: Vec<Song> = top_tracks
        .iter()
        .map(|track| Song {
            name: track.name.clone(),
            artist_name: track.artists[0].name.clone(),
            song_image_uri: track.album.images[0].url.clone(),
        })
        .collect();

    top_tracks
}

pub async fn get_recent_songs(client: &Arc<AuthCodeSpotify>) -> Vec<Song> {
    let time = Utc::now();
    let time_limit = TimeLimits::Before(time);

    let recent_tracks = match client
        .current_user_recently_played(Some(50), Some(time_limit))
        .await
    {
        Ok(response) => response.items,
        Err(e) => {
            eprintln!("Error fetching recent tracks: {}", e);
            Vec::new() // or handle error differently
        }
    };

    let recent_songs: Vec<Song> = recent_tracks
        .iter()
        .map(|track| Song {
            name: track.track.name.clone(),
            artist_name: track.track.artists[0].name.clone(),
            song_image_uri: track.track.album.images[0].url.clone(),
        })
        .collect();

    recent_songs
}
