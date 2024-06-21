use crate::domain_model::music_library;
use crate::domain_model::music_library::music_library::MusicLibrary;
use crate::domain_model::music_library::value_object::music::Music;
use crate::domain_model::user::value_object::user_id::UserID;
use crate::domain_service::music_library_repository_trait::MusicLibraryRepositoryTrait;
use anyhow::Result;
use async_graphql::async_trait::async_trait;
use sqlx::MySqlPool;

pub struct MusicLibraryRepository {
    pool: MySqlPool
}

impl MusicLibraryRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MusicLibraryRepositoryTrait for MusicLibraryRepository {
    async fn find_by_id(&self, user_id: String) -> Result<MusicLibrary> {
        let music = sqlx::query_as!(
            Music,
            r#"SELECT title, artist, apple_music_id FROM music_library WHERE user_id = ?"#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(MusicLibrary {
            user_id: UserID::new(),
            musics: vec![music],
        })
    }

    async fn find_by_user_id(&self, user_id: UserID) -> Result<Vec<MusicLibrary>> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<MusicLibrary>> {
        todo!()
    }

    async fn save(&self, music_library: MusicLibrary) -> Result<()> {
        todo!()
    }

    async fn delete(&self, music_library: MusicLibrary) -> Result<()> {
        todo!()
    }
}

