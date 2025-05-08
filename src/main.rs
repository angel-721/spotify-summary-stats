use axum::{routing::get, Router};
use shuttle_runtime::SecretStore;
use std::sync::Arc;

use wraped::{
    routes::{auth, callback, index, login, top_songs},
    spotify::helpers::spotify_client,
    types::app_state::AppState,
};

// #[tokio::main]

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // env_logger::init();
    // dotenv::dotenv().ok();
    let client = secrets.get("SPOTIFY_CLIENT_ID").unwrap();
    let secret = secrets.get("SPOTIFY_CLIENT_SECRET").unwrap();

    let client = Arc::new(spotify_client(&client, &secret));
    let state = AppState { spotify: client };

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let app = Router::new()
        .route("/", get(index::handler))
        .route("/callback", get(callback::handler))
        .route("/login", get(login::handler))
        .route("/top_songs", get(top_songs::handler))
        .route("/auth", get(auth::handler))
        .with_state(state);

    Ok(app.into())
    // Ok(app.into())
    // println!("Listening on http://localhost:3000/");
    // axum::serve(listener, app).await.unwrap();
}
