
pub trait User {
  fn Id(&self) -> UserID;
  fn name(&self) -> UserName;
  fn isDeleted(&self) -> bool;
}

