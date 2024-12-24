use serde::Deserialize;
#[derive(Deserialize)]
pub struct AuthCode {
    pub code: String,
}
