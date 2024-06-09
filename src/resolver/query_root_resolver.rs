use crate::domain_model::music_library::music_library::MusicLibrary;
use crate::domain_model::user::value_object::user_id::UserID;

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

    async fn getMusicLibraries<'ctx>(&self, ctx: &Context<'ctx>, room_id: i32) -> Result<Vec<MusicLibrary>> {
        Ok(vec![
            MusicLibrary::new(UserID::new())
        ])
    }
}

#[derive(SimpleObject)]
struct Ping {
    status: String,
    code: i32,
}