use async_graphql::{Context, Error, Object, Result};

use crate::domain_model::music_library::music_library::{MusicLibrary, MusicLibraryInput};
use crate::domain_model::room::room::Room;
use crate::domain_model::room::value_object::room_pin::RoomPin;
use crate::domain_model::user::value_object::user_id::{UserID, UserIDInput};
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
        let result = ctx.get_music_library_repository().save(music_library).await;
        if result.is_err() {
            return Err(Error::from(result.err().unwrap()));
        }
        let music_library = result.unwrap();
        Ok(music_library)
    }

    pub async fn create_room<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: UserIDInput,
    ) -> Result<Room> {
        let ctx = ctx.data::<Dependency>().unwrap();

        // get host user
        let result = ctx
            .get_user_repository()
            .find_by_id(UserID::new(user_id.value))
            .await;
        if result.is_err() {
            return Err(Error::from(result.err().unwrap()));
        }
        let host = result.unwrap();

        // issue new room pin
        let mut room_pin;
        loop {
            room_pin = RoomPin::random_new();
            let result = ctx
                .get_room_repository()
                .check_existence(room_pin.raw_value())
                .await;
            if result.is_err() {
                return Err(Error::from(result.err().unwrap()));
            }
            let exists = result.unwrap();
            if !exists {
                break;
            }
        }

        // create new room
        let room = Room::new(room_pin, host.id, true, vec![]);
        let result = ctx.get_room_repository().save(room).await;
        if result.is_err() {
            return Err(Error::from(result.err().unwrap()));
        }
        let room = result.unwrap();
        Ok(room)
    }
}
