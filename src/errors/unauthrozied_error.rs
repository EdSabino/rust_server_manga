use std::fmt;

#[derive(Debug)]
pub enum UnauthorizedError {
  InvalidToken,
  MissingAuthorization,
  RouteNotAllowed,
  UnknownError
}

impl std::error::Error for UnauthorizedError {}

impl fmt::Display for UnauthorizedError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      UnauthorizedError::InvalidToken => write!(f, "Invalid token"),
      UnauthorizedError::MissingAuthorization => write!(f, "Missing `Authorization` header"),
      UnauthorizedError::RouteNotAllowed => write!(f, "You don't have permission to access this route"),
      UnauthorizedError::UnknownError => write!(f, "An unknown error has ocurred")
    }
  }
}