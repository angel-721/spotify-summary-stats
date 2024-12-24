use crate::routes::prelude::*;
pub async fn handler(State(state): State<AppState>) -> impl IntoResponse {
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
