use crate::domain_model::room::room::Room;
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct RoomUpdated {
    room: Room,
    is_confirmed: bool,
}

impl RoomUpdated {
    pub fn new(room: Room) -> Self {
        RoomUpdated {
            room,
            is_confirmed: false,
        }
    }

    pub fn confirm(&mut self) {
        self.is_confirmed = true;
    }
}
