use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject)]
pub struct UserID {
    value: String,
}

impl UserID {
    /// コンストラクタ
    ///
    /// # 戻り値
    /// - `UserID` - ユーザーID
    pub fn new(user_id: String) -> Self {
        UserID {
            value: user_id
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, InputObject)]
pub struct UserIDInput {
    pub value: String,
}
