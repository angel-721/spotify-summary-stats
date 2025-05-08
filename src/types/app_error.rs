use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    /// not found
    #[error("Not found")]
    NotFound,

    /// could not render template
    #[error("Failed to render template: {0}")]
    Render(#[from] askama::Error),
}

/// This is your error handler
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // It uses an askama template to display its content.
        // The member `lang` is used by "_layout.html" which "error.html" extends. Even though it
        // is always the fallback language English in here, "_layout.html" expects to be able to
        // access this field, so you have to provide it.
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {
            err: AppError,
        }

        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = Tmpl { err: self };
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
