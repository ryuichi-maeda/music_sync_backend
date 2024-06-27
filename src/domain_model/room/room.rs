use super::value_object::room_pin::RoomPin;
use crate::domain_model::user::guest_user::GuestUser;

#[derive(Debug, PartialEq, Eq)]
pub struct Room {
    room_pin: RoomPin,
    host: GuestUser,
    is_active: bool,
    is_confirm: bool,
    members: Vec<GuestUser>,
}

impl Room {
    pub fn new(room_pin: RoomPin, host: GuestUser) -> Self {
        Room {
            room_pin,
            host: host.clone(),
            is_active: true,
            is_confirm: false,
            members: vec![host],
        }
    }

    pub fn enter_room(&mut self, user: &GuestUser) {
        self.members.push(user.clone());
    }

    pub fn confirm_room(&mut self) {
        self.is_confirm = true
    }

    pub fn exit_room(&mut self, user: &GuestUser) {
        if let Some(index) = self.members.iter().position(|x| *x == *user) {
            self.members.remove(index);
        }
    }

    pub fn close_room(&mut self) {
        self.is_active = false
    }
}
