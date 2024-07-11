use anyhow::Result;
use async_graphql::SimpleObject;
use std::fmt::{self, Display, Formatter};
use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq, SimpleObject)]
pub struct UserName {
    value: String,
}

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
            Ok(UserName {
                value: user_name.to_string(),
            })
        }
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for UserName {
    fn from(user_name: String) -> Self {
        UserName { value: user_name }
    }
}
