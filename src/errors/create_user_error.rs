use std::fmt;

#[derive(Debug)]
pub enum CreateUserError {
  InvalidEmail,
  UnkownError
}

impl std::error::Error for CreateUserError {}

impl fmt::Display for CreateUserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CreateUserError::InvalidEmail => write!(f, "Invalid email"),
      CreateUserError::UnkownError => write!(f, "Unknown error creating user")
    }
  }
}