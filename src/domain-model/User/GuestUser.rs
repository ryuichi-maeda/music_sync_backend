#[derive(Debug, Clone, PartialEq, Eq)]

pub struct GuestUser {
  pub id: UserID,
  pub user_name: UserName,
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
    User {
      id: UserID::new(),
      user_name,
      is_deleted: false,
    }
  }
}

impl User for GuestUser {
  fn Id(&self) -> UserID {
    self.id
  }

  fn name(&self) -> UserName {
    self.user_name.clone()
  }

  fn is_deleted(&self) -> bool {
    self.deleted
  }
}