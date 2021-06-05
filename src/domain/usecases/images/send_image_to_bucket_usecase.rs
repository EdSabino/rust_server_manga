use crate::domain::repository::s3_repository::S3Repository;
use crate::errors::ImageUploadError;
use crate::domain::entities::events::UploadImageEvent;

pub struct SendImageToBucketUseCase {
    repository: S3Repository
}

impl SendImageToBucketUseCase {
    pub fn new() -> Self {
        SendImageToBucketUseCase {
            repository: S3Repository::new()
        }
    }

    pub fn call(&self, data: &UploadImageEvent) -> Result<(), ImageUploadError> {
        self.repository.upload(data.name.clone(), &data.image, data.mime.clone()).unwrap();
        Ok(())
    }
}
