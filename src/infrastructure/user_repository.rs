use crate::domain_model::user::guest_user::GuestUser;
use crate::domain_model::user::value_object::user_id::UserID;
use crate::domain_service::user_repository_trait::UserRepositoryTrait;

use anyhow::Result;
use async_graphql::async_trait::async_trait;
use sqlx::MySqlPool;

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_by_id(&self, id: UserID) -> Result<GuestUser> {
        let user = sqlx::query_as!(
            GuestUser,
            r#"SELECT id, username, user_type FROM users WHERE id = ?"#,
            id.raw_value()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<GuestUser>> {
        todo!()
    }

    async fn save(&self, user: GuestUser) -> Result<()> {
        todo!()
    }

    async fn update(&self, user: GuestUser) -> Result<()> {
        todo!()
    }

    async fn delete(&self, user: GuestUser) -> Result<()> {
        todo!()
    }

    async fn check_existence(&self, id: UserID) -> Result<bool> {
        todo!()
    }
}
