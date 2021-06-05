use crate::domain::repository::s3_repository::S3Repository;
use crate::domain::entities::events::{DeleteImageEvent, RemoveImageCacheEvent};

pub struct DeleteImageUseCase {
    repository: S3Repository
}

impl DeleteImageUseCase {
    pub fn new() -> Self {
        DeleteImageUseCase {
            repository: S3Repository::new()
        }
    }

    pub fn call(&mut self, data: &DeleteImageEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.delete(data.name.clone()).unwrap();
        dispatch!("remove_image_cache", &mut RemoveImageCacheEvent::new(data.name.clone()));
        Ok(())
    }
}
