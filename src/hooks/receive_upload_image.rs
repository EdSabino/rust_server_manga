use crate::domain::entities::events::UploadImageEvent;
use crate::domain::usecases::images::{SendImageToBucketUseCase, AddImageToCache};

pub fn handler(message: &mut UploadImageEvent) {
    SendImageToBucketUseCase::new().call(message).unwrap();
    AddImageToCache::new().call(message).unwrap();
}