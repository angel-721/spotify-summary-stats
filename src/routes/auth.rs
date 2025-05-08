use crate::routes::prelude::*;
use axum::http::StatusCode;

pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    println!("GET /auth");

    let token = state.spotify.token.lock().await.unwrap();
    println!("{:?}", *token);

    let response = match token.is_none() {
        true => {
            #[derive(Debug, Template)]
            #[template(path = "login.html")]
            struct Tmpl {}

            let template = Tmpl {};
            //TODO: Find a way to get HTMX template to handle 401 correctly
            Ok((StatusCode::OK, Html(template.render()?)))
        }
        false => {
            #[derive(Debug, Template)]
            #[template(path = "home.html")]
            struct Tmpl {}

            let template = Tmpl {};
            Ok((StatusCode::OK, Html(template.render()?)))
        }
    };

    response
}
