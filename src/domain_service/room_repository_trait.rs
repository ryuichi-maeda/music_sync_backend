use crate::domain_model::room::room::Room;
use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait RoomRepositoryTrait: Send + Sync {
    async fn find_by_id(&self, id: String) -> Result<Room>;
    async fn save(&self, room: Room) -> Result<()>;
    async fn delete(&self, room: Room) -> Result<()>;
    async fn check_existence(&self, id: String) -> Result<()>;
}
