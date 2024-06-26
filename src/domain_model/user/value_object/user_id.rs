use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject, InputObject)]
pub struct UserID {
  value: String,
}

impl UserID {
  /// コンストラクタ
  ///
  /// # 戻り値
  /// - `UserID` - ユーザーID
  pub fn new() -> Self {
    UserID {
      value: Uuid::new_v4().to_string(),
    }
  }
}