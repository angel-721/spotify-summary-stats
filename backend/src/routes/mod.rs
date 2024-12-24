pub mod callback;
pub mod login;
pub mod top_songs;

pub mod prelude {
    pub use axum::{
        extract::{Query, State},
        http::{HeaderName, Method, StatusCode},
        response::{Html, IntoResponse},
        routing::get,
        Router,
    };

    pub use crate::types::app_state::AppState;
}
