use async_graphql::{Context, Error, Object, Result};

use crate::domain_model::music_library;
use crate::domain_model::user::value_object::user_id::UserID;
use crate::Dependency;
use crate::domain_model::music_library::music_library::MusicLibrary;
// Mutation
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_music_library<'ctx>(&self, ctx: &Context<'ctx>, user_id: UserID, music_library: MusicLibrary) -> Result<MusicLibrary> {
        let ctx = ctx.data::<Dependency>().unwrap();
        let result = ctx.get_music_library_repository().save(user_id, music_library).await;
        if result.is_err() {
            return Err(Error::from(result.err().unwrap()));
        }
        let music_library = result.unwrap();
        Ok(music_library)
        
    }
}
