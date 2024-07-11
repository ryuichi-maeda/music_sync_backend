use async_graphql::{InputObject, SimpleObject};

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
        UserID { value: user_id }
    }

    pub fn raw_value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, InputObject)]
pub struct UserIDInput {
    pub value: String,
}

impl From<i32> for UserID {
    fn from(user_id: i32) -> Self {
        UserID {
            value: user_id.to_string(),
        }
    }
}
