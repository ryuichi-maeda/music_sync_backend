use async_graphql::{Context, Error, Object, Result};

use crate::domain_model::music_library::music_library::{MusicLibrary, MusicLibraryInput};
use crate::domain_model::user::value_object::user_id::{UserIDInput};
use crate::Dependency;

// Mutation
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_music_library<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        music_library: MusicLibraryInput,
    ) -> Result<MusicLibrary> {
        let ctx = ctx.data::<Dependency>().unwrap();
        let result = ctx
            .get_music_library_repository()
            .save(music_library)
            .await;
        if result.is_err() {
            return Err(Error::from(result.err().unwrap()));
        }
        let music_library = result.unwrap();
        Ok(music_library)
    }
}
