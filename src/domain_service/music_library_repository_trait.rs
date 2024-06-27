use crate::domain_model::music_library::music_library::{MusicLibrary, MusicLibraryInput};
use crate::domain_model::user::value_object::user_id::{UserID, UserIDInput};
use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait MusicLibraryRepositoryTrait: Send + Sync {
    async fn find_by_id(&self, id: String) -> Result<MusicLibrary>;
    async fn find_by_user_id(&self, user_id: UserID) -> Result<Vec<MusicLibrary>>;
    async fn save(&self, music_library: MusicLibraryInput) -> Result<MusicLibrary>;
    async fn delete(&self, music_library: MusicLibrary) -> Result<()>;
}
