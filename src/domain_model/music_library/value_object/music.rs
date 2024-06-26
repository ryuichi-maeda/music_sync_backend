use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject, InputObject)]
pub struct Music {
  pub title: String,
  pub artist: String,
  pub apple_music_id: Option<String>,
}