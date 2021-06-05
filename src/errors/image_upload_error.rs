use std::fmt;

#[derive(Debug)]
pub enum ImageUploadError {
  NotFound,
  UnkownError
}

impl std::error::Error for ImageUploadError {}

impl fmt::Display for ImageUploadError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ImageUploadError::NotFound => write!(f, "Not found"),
      ImageUploadError::UnkownError => write!(f, "Unknown error")
    }
  }
}