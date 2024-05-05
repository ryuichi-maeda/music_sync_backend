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
  pub fn new(user_name: &str) -> Result<Self> {
    if user_name.is_empty() {
      Err(UserNameError::Empty.into())
    } else if user_name.len() > 14 {
      Err(UserNameError::TooLong.into())
    } else {
      Ok(UserName(user_name.to_string()))
    }
    
  }
}