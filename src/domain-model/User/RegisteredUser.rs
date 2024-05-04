#[derive(Debug, Clone, PartialEq, Eq)]

pub struct RegisteredUser {
  pub id: UserID,
  pub username: UserName,
  pub email: String,
  pub password: String,
  pub deleted: bool,
}

impl RegisteredUser {
  /// コンストラクタ
  ///
  /// # 引数
  /// - `id` - ユーザーID
  /// - `userName` - ユーザー名
  /// - `password` - パスワード
  ///
  /// # 戻り値
  /// - `RegisteredUser` - ユーザー
  pub fn new(userName: UserName, email: String, password: String) -> Self {
    User {
      id: UserID::new(),
      username,
      email,
      password,
      deleted: false,
    }
  }
}
