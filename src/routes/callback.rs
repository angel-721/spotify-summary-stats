use crate::routes::prelude::*;
use crate::types::query_parameters::AuthCode;
use rspotify::clients::OAuthClient;

pub async fn handler(
    State(state): State<AppState>,
    Query(auth_code): Query<AuthCode>,
) -> impl IntoResponse {
    let spotify = state.spotify;
    println!("GET /callback");

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
