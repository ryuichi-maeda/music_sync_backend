use super::value_object::user_id::UserID;
use super::value_object::user_name::UserName;
use super::value_object::user_type::UserType;

pub trait User {
  fn id(&self) -> UserID {
    self.id()
  }

  fn user_name(&self) -> UserName {
    self.user_name()
  }

  fn user_type(&self) -> UserType {
    self.user_type()
  }

  fn is_deleted(&self) -> bool {
    self.is_deleted()
  }

}

