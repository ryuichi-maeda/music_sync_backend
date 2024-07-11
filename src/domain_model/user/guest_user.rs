use async_graphql::SimpleObject;

use super::user::User;
use super::value_object::user_id::UserID;
use super::value_object::user_name::UserName;
use super::value_object::user_type::UserType;

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject)]
pub struct GuestUser {
    pub id: UserID,
    pub username: UserName,
    pub user_type: UserType,
}

impl GuestUser {
    /// コンストラクタ
    ///
    /// # 引数
    /// - `id` - ユーザーID
    /// - `user_name` - ユーザー名
    ///
    /// # 戻り値
    /// - `GuestUser` - ユーザー
    pub fn new(id: String, username: UserName) -> Self {
        GuestUser {
            id: UserID::new(id),
            username,
            user_type: UserType::Guest,
        }
    }
}

impl User for GuestUser {}
