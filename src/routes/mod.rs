pub mod auth;
pub mod callback;
pub mod index;
pub mod login;
pub mod recent_songs;
pub mod top_songs;

pub mod prelude {
    pub use axum::{
        extract::{Query, State},
        http::{HeaderName, Method, StatusCode},
        response::{Html, IntoResponse},
        routing::get,
        Router,
    };

    pub use crate::types::{app_error::AppError, app_state::AppState};
    pub use askama::Template;
}
