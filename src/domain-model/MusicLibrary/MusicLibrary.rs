
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicLibrary {
  pub user_id: UserID,
  pub musics: Vec<Music>,
}