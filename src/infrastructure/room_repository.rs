use crate::domain_model::room::room::Room;
use crate::domain_model::room::value_object::room_pin::RoomPin;
use crate::domain_model::user::guest_user::GuestUser;
use crate::domain_model::user::value_object::user_id::UserID;
use crate::domain_service::room_repository_trait::RoomRepositoryTrait;

use anyhow::Result;
use async_graphql::async_trait::async_trait;
use sqlx::MySqlPool;

pub struct RoomRepository {
    pool: MySqlPool,
}

impl RoomRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoomRepositoryTrait for RoomRepository {
    async fn find_by_room_pin(&self, room_pin: String) -> Result<Room> {
        let room_data = sqlx::query_as!(
            RoomData,
            r#" 
                SELECT 
                    id, host_id, is_active 
                FROM 
                    rooms 
                WHERE
                    room_pin = ?"#,
            room_pin
        )
        .fetch_one(&self.pool)
        .await?;

        let members = sqlx::query_as!(
            GuestUser,
            r#"
                SELECT 
                    user_id as id, username, user_type
                FROM 
                    user_rooms
                INNER JOIN 
                    users ON user_rooms.user_id = users.id
                WHERE room_id = ?"#,
            room_data.id
        )
        .fetch_all(&self.pool)
        .await?;

        let room = Room::new(
            RoomPin::new(room_pin)?,
            UserID::from(room_data.host_id),
            room_data.is_active == 1,
            members,
        );
        Ok(room)
    }

    async fn save(&self, room: Room) -> Result<Room> {
        sqlx::query!(
            r#"
                INSERT INTO rooms (room_pin, host_id, is_active)
                VALUES (?, ?, ?)
            "#,
            room.room_pin_value().raw_value(),
            room.host_id_value().raw_value(),
            room.is_active_value() as i8
        )
        .execute(&self.pool)
        .await?;

        for member in room.members_value() {
            sqlx::query!(
                r#"
                    INSERT INTO user_rooms (user_id, room_id)
                    VALUES (?, ?)
                "#,
                member.id.raw_value(),
                room.room_pin_value().raw_value()
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(room)
    }

    async fn delete(&self, room: Room, user: GuestUser) -> Result<()> {
        let room_id = sqlx::query_as!(
            RoomIdData,
            r#"SELECT id FROM rooms WHERE room_pin = ?"#,
            room.room_pin_value().raw_value()
        )
        .fetch_one(&self.pool)
        .await?;

        sqlx::query!(
            r#"DELETE FROM user_rooms WHERE room_id = ? AND user_id = ?"#,
            room_id.id,
            user.id.raw_value()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn is_active(&self, room_pin: String) -> Result<bool> {
        let room_data = sqlx::query_as!(
            RoomData,
            r#" 
                SELECT 
                    id, host_id, is_active 
                FROM 
                    rooms 
                WHERE
                    room_pin = ?"#,
            room_pin
        )
        .fetch_one(&self.pool)
        .await?;

        if room_data.is_active == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn check_existence(&self, room_pin: String) -> Result<bool> {
        let result = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM rooms WHERE room_pin = ?"#,
            room_pin
        )
        .fetch_one(&self.pool)
        .await?;

        if result.count == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

struct RoomData {
    id: i32,
    host_id: i32,
    is_active: i8,
}

struct RoomIdData {
    id: i32,
}
