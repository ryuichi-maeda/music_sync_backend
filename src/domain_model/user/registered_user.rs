use super::value_object::user_id::UserID;
use super::value_object::user_name::UserName;
use super::value_object::user_type::UserType;

#[derive(Debug, PartialEq, Eq)]
pub struct RegisteredUser {
    pub id: UserID,
    pub user_name: UserName,
    pub email: String,
    pub password: String,
    pub user_type: UserType,
    pub is_deleted: bool,
}

impl RegisteredUser {
    /// コンストラクタ
    ///
    /// # 引数
    /// - `id` - ユーザーID
    /// - `user_name` - ユーザー名
    /// - `password` - パスワード
    ///
    /// # 戻り値
    /// - `RegisteredUser` - ユーザー
    pub fn new(id: String, user_name: UserName, email: String, password: String) -> Self {
        RegisteredUser {
            id: UserID::new(id),
            user_name,
            email,
            password,
            user_type: UserType::Registered,
            is_deleted: false,
        }
    }
}
