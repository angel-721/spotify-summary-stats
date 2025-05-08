use serde::Deserialize;
#[derive(Deserialize)]
pub struct AuthCode {
    pub code: String,
}

#[derive(Deserialize)]
pub struct TimeRangeParam {
    pub time_range: String,
}
