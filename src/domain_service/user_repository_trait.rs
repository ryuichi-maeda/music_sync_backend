use crate::domain_model::user::guest_user::GuestUser;
use crate::domain_model::user::value_object::user_id::UserID;

use axum::async_trait;
use anyhow::Result;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find_by_id(&self, id: UserID) -> Result<GuestUser>;
    async fn find_all(&self) -> Result<Vec<GuestUser>>;
    async fn save(&self, user: GuestUser) -> Result<()>;
    async fn update(&self, user: GuestUser) -> Result<()>;
    async fn delete(&self, user: GuestUser) -> Result<()>;
    async fn check_existence(&self, id: UserID) -> Result<bool>;
}