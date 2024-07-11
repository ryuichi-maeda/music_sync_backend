use async_graphql::SimpleObject;

use super::value_object::room_pin::RoomPin;
use crate::domain_model::user::{guest_user::GuestUser, value_object::user_id::UserID};

#[derive(SimpleObject, Debug, Clone, PartialEq, Eq)]
pub struct Room {
    room_pin: RoomPin,
    host_id: UserID,
    is_active: bool,
    members: Vec<GuestUser>,
    is_confirmed: bool,
}

impl Room {
    pub fn new(
        room_pin: RoomPin,
        host_id: UserID,
        is_active: bool,
        members: Vec<GuestUser>,
    ) -> Self {
        Room {
            room_pin,
            host_id,
            is_active,
            members,
            is_confirmed: false,
        }
    }

    pub fn enter(&mut self, user: &GuestUser) {
        self.members.push(user.clone());
    }

    pub fn exit(&mut self, user: &GuestUser) {
        if let Some(index) = self.members.iter().position(|x| *x == *user) {
            self.members.remove(index);
        }
    }

    pub fn close(&mut self) {
        self.is_active = false
    }

    pub fn confirm(&mut self) {
        self.is_confirmed = true;
    }

    pub fn room_pin_value(&self) -> &RoomPin {
        &self.room_pin
    }

    pub fn host_id_value(&self) -> &UserID {
        &self.host_id
    }

    pub fn is_active_value(&self) -> bool {
        self.is_active
    }

    pub fn members_value(&self) -> &Vec<GuestUser> {
        &self.members
    }
}
