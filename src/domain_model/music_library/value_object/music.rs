use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject)]
pub struct Music {
    pub title: String,
    pub artist: String,
    pub apple_music_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, InputObject)]
pub struct MusicInput {
    pub title: String,
    pub artist: String,
    pub apple_music_id: Option<String>,
}
