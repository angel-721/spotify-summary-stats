use rspotify::AuthCodeSpotify;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub spotify: Arc<AuthCodeSpotify>,
}
