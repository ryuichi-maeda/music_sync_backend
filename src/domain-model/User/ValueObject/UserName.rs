use anyhow::Result;

#[derive(Error, Debug, Clone)]
pub struct UserName(String);

#[derive(Error, Debug, Clone)]
pub enum UserNameError {
  #[error("the group chat name is empty")]
  Empty,
  #[error("the group chat name is too long")]
  TooLong,
}

impl UserName {
  pub fn new(userName: &str) -> Result<Self> {
    if userName.is_empty() {
      Err(UserNameError::Empty.into())
    }
    
  }
}