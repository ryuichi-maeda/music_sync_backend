use super::user::User;
use super::value_object::user_id::UserID;
use super::value_object::user_name::UserName;
use super::value_object::user_type::UserType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuestUser {
    pub id: UserID,
    pub user_name: UserName,
    pub user_type: UserType,
    pub is_deleted: bool,
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
    pub fn new(user_name: UserName) -> Self {
        GuestUser {
            id: UserID::new(),
            user_name,
            user_type: UserType::Guest,
            is_deleted: false,
        }
    }
}

impl User for GuestUser {}
