#[derive(Debug, Clone, PartialEq, Eq)]

pub struct GuestUser {
  pub id: UserID,
  pub username: UserName,
  pub deleted: bool,
}

impl GuestUser {
  /// コンストラクタ
  ///
  /// # 引数
  /// - `id` - ユーザーID
  /// - `userName` - ユーザー名
  ///
  /// # 戻り値
  /// - `GuestUser` - ユーザー
  pub fn new(userName: UserName) -> Self {
    User {
      id: UserID::new(),
      username,
      deleted: false,
    }
  }
}

impl User for GuestUser {
  fn Id(&self) -> UserID {
    self.id
  }

  fn name(&self) -> UserName {
    self.username.clone()
  }

  fn isDeleted(&self) -> bool {
    self.deleted
  }
}