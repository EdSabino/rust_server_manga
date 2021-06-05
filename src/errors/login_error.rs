use std::fmt;

#[derive(Debug)]
pub enum LoginError {
  WrongPassword
}

impl std::error::Error for LoginError {}

impl fmt::Display for LoginError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      LoginError::WrongPassword => write!(f, "Wrong password")
    }
  }
}