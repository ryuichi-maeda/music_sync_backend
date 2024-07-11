use crate::domain_model::{room::room::Room, user::guest_user::GuestUser};
use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait RoomRepositoryTrait: Send + Sync {
    async fn find_by_room_pin(&self, room_pin: String) -> Result<Room>;
    async fn save(&self, room: Room) -> Result<Room>;
    async fn delete(&self, room: Room, user: GuestUser) -> Result<()>;
    async fn check_existence(&self, room_pin: String) -> Result<bool>;
}
