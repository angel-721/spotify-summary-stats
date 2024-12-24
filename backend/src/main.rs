use axum::{routing::get, Router};

use std::sync::Arc;

use wraped::{
    routes::{callback, index, login, top_songs},
    spotify::helpers::spotify_client,
    types::app_state::AppState,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let client = Arc::new(spotify_client());
    let state = AppState { spotify: client };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let app = Router::new()
        .route("/callback", get(callback::handler))
        .route("/login", get(login::handler))
        .route("/top_songs", get(top_songs::handler))
        .with_state(state)
        .nest_service("/", index::handler());

    println!("Listening on http://localhost:3000/");
    axum::serve(listener, app).await.unwrap();
}
