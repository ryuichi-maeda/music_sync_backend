
pub trait User {
  fn Id(&self) -> UserID {
    self.id
  }

  fn user_name(&self) -> UserName {
    self.user_name.clone()
  }

  fn user_type(&self) -> UserType {
    self.user_type
  }

  fn is_deleted(&self) -> bool {
    self.is_deleted
  }


}

