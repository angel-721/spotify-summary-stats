use axum::{
    extract::{Query, State},
    http::{HeaderName, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use wraped::types::{app_state::AppState, auth_code::AuthCode, song::Song};

use rspotify::{
    clients::OAuthClient, model::TimeRange, scopes, AuthCodeSpotify, Credentials, OAuth,
};

use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([HeaderName::from_static("content-type")]);

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

    let spotify_client_id =
        env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not found in .env");

    let spotify_client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not found in .env");

    let creds = Credentials::new(&spotify_client_id, &spotify_client_secret);
    let config = rspotify::Config::default();
    let client = Arc::new(AuthCodeSpotify::with_config(creds, oauth, config));

    let state = AppState { spotify: client };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Build the application with the greeting route
    let app = Router::new()
        .route("/callback", get(callback))
        .route("/login", get(login_handler))
        .route("/me", get(my_name))
        .layer(cors)
        .with_state(state)
        .nest_service("/", ServeDir::new("../public"));

    println!("Listening on http://localhost:3000/");
    axum::serve(listener, app).await.unwrap();
}

async fn my_name(State(state): State<AppState>) -> impl IntoResponse {
    let client = &state.spotify;

    println!("Got token above, now running current_playing");

    let playlist = client
        .current_user_top_tracks_manual(Some(TimeRange::ShortTerm), Some(10), Some(0))
        .await
        .unwrap()
        .items;

    let songs: Vec<Song> = playlist
        .iter()
        .filter_map(|track| {
            Some(Song {
                name: track.name.clone(),
                artist_name: track.artists[0].name.clone(),
                song_image_uri: track.album.images[0].url.clone(),
            })
        })
        .collect();

    println!("{:?}", songs);

    let list_items: String = songs
        .iter()
        .map(|song| format!("<li>{}</li>", song.name))
        .collect::<Vec<_>>()
        .join("");

    Html(format!(
        r#"
    <ul>
        {}
    </ul>
    "#,
        list_items
    ))
    .into_response()
}

async fn login_handler(State(state): State<AppState>) -> impl IntoResponse {
    let spotify = state.spotify;

    match spotify.get_authorize_url(false) {
        Ok(url) => Html(format!(
            "<script>window.location.href = '{}';</script>",
            url
        ))
        .into_response(),
        Err(e) => {
            eprintln!("Failed to get authorize URL: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate auth URL",
            )
                .into_response()
        }
    }
}

async fn callback(
    State(state): State<AppState>,
    Query(auth_code): Query<AuthCode>,
) -> impl IntoResponse {
    let spotify = state.spotify;

    match spotify.request_token(&auth_code.code).await {
        Ok(_) => {
            println!("Successfully authenticated with Spotify!");
            Html(
                r#"
                <div>Successfully authenticated!</div>
                <script>
                    setTimeout(() => {
                        window.location.href = '/';
                    }, 1000);
                </script>
                "#,
            )
            .into_response()
        }
        Err(e) => {
            eprintln!("Failed to get token: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Authentication failed: {:?}", e),
            )
                .into_response()
        }
    }
}
