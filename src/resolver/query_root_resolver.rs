use std::sync::Arc;
use std::vec;

use crate::domain_model::music_library::music_library::MusicLibrary;
use crate::domain_service::music_library_repository_trait::MusicLibraryRepositoryTrait;
use crate::domain_service::room_repository_trait::RoomRepositoryTrait;
use crate::domain_service::user_repository_trait::UserRepositoryTrait;
use crate::Dependency;

use async_graphql::*;

// Query
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> Ping {
        Ping {
            status: "ok".to_string(),
            code: 200,
        }
    }

    async fn get_music_libraries<'ctx>(&self, ctx: &Context<'ctx>, room_id: String) -> Result<Vec<MusicLibrary>> {
        let ctx = ctx.data::<Dependency>().unwrap();
        let music = ctx.get_music_library_repository().find_by_id(room_id).await?;
        Ok(vec![music])
    }
}

#[derive(SimpleObject)]
struct Ping {
    status: String,
    code: i32,
}