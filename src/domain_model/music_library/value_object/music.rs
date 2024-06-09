use async_graphql::SimpleObject;

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject)]
pub struct Music {
  pub title: String,
  pub artist: String,
  pub apple_music_id: Option<String>,
}