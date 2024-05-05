#[derive(Debug, Clone, PartialEq, Eq)]

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
  pub fn new(user_name: UserName, email: String, password: String) -> Self {
    RegisteredUser {
      id: UserID::new(),
      user_name,
      email,
      password,
      user_type: UserType::Registered,
      is_deleted: false,
    }
  }
}
