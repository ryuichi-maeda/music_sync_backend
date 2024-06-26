use crate::domain_model::user::guest_user::GuestUser;
use crate::domain_service::user_repository_trait::UserRepositoryTrait;
use crate::domain_model::user::value_object::user_id::UserID;

use async_graphql::async_trait::async_trait;
use sqlx::MySqlPool;
use anyhow::Result;

pub struct UserRepository {
    pool: MySqlPool
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
  async fn find_by_id(&self, id: UserID) -> Result<GuestUser> {
      todo!()
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