use crate::routes::prelude::*;
use crate::spotify::helpers::{get_recent_songs, get_top_songs};
use crate::types::query_parameters::TimeRangeParam;
use rspotify::model::TimeRange;

pub async fn handler(
    State(state): State<AppState>,
    Query(query_param): Query<TimeRangeParam>,
) -> impl IntoResponse {
    println!("GET /top_songs");
    let client = &state.spotify;

    let range = match query_param.time_range.as_str() {
        "long_term" => TimeRange::LongTerm,
        "medium_term" => TimeRange::MediumTerm,
        "short_term" => TimeRange::ShortTerm,
        _ => TimeRange::ShortTerm,
    };

    let top_songs = get_top_songs(client, range).await;

    // get_recent_songs(client).await;

    let list_items: String = top_songs
        .iter()
        .map(|song| {
            format!(
                "<li class='top-song'><div class='top-song-content'>{} <img src={} /></div></li>",
                song.name, song.song_image_uri
            )
        })
        .collect::<Vec<_>>()
        .join("");

    Html(format!(
        r#"
    <ul id="top-songs-ul">
        {}
    </ul>
    "#,
        list_items
    ))
    .into_response()
}
