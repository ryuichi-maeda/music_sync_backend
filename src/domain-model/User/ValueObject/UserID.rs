
pub struct UserID {
  value: Uuid,
}

impl UserID {
  /// コンストラクタ
  ///
  /// # 戻り値
  /// - `UserID` - ユーザーID
  pub fn new() -> Self {
    UserID {
      value: Uuid::new_v4(),
    }
  }
}