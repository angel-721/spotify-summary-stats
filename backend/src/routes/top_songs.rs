use crate::routes::prelude::*;
use crate::spotify::helpers::get_top_songs;
// use crate::types::song::Song;

pub async fn handler(State(state): State<AppState>) -> impl IntoResponse {
    let client = &state.spotify;

    println!("Got token above, now running current_playing");

    let top_songs = get_top_songs(client).await;

    println!("{:?}", top_songs);

    let list_items: String = top_songs
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
