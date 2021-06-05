use std::fmt;

#[derive(Debug)]
pub enum CreateMangaError {
  InvalidName,
  UnkownError
}

impl std::error::Error for CreateMangaError {}

impl fmt::Display for CreateMangaError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CreateMangaError::InvalidName => write!(f, "Invalid name"),
      CreateMangaError::UnkownError => write!(f, "Unknown error")
    }
  }
}