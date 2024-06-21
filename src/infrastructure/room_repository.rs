use crate::domain_model::room::room::Room;
use crate::domain_service::room_repository_trait::RoomRepositoryTrait;

use async_graphql::async_trait::async_trait;
use anyhow::Result;
use sqlx::MySqlPool;


pub struct RoomRepository {
    pool: MySqlPool
}

impl RoomRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoomRepositoryTrait for RoomRepository {
    async fn find_by_id(&self, room_id: String) -> Result<Room> {
        todo!()
    }

    async fn save(&self, room: Room) -> Result<()> {
        todo!()
    }

    async fn delete(&self, room: Room) -> Result<()> {
        todo!()
    }

    async fn check_existence(&self, room_id: String) -> Result<()> {
        todo!()
    }
}