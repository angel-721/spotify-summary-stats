use std::fmt;

#[derive(Debug)]
pub struct Song {
    pub name: String,
    pub song_image_uri: String,
    pub artist_name: String,
}

// manually for the type.
impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
