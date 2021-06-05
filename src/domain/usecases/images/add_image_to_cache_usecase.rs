use std::time::Duration;
use tokio::time::sleep;

use crate::errors::ImageUploadError;
use crate::domain::entities::events::{UploadImageEvent, DeleteImageEvent};
use crate::domain::repository::redis_repository::RedisRepository;

pub struct AddImageToCache {
    redis: RedisRepository
}

impl AddImageToCache {
    pub fn new() -> Self {
        AddImageToCache {
            redis: RedisRepository::new().unwrap()
        }
    }

    pub fn call(&mut self, data: &UploadImageEvent) -> Result<(), ImageUploadError> {
        println!("callllibng");
        match self.set_redis(data.name.clone()) {
            Ok(_) => {
                self.await_for_delete(data.name.clone());
                Ok(())
            },
            Err(e) => {
                println!("{}", e.to_string());
                Err(ImageUploadError::UnkownError)
            }
        }
    }

    fn set_redis(&mut self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut pending_images: Vec<String> = self.redis.get_pending_images()?;
        pending_images.push(name.clone());
        self.redis.set_pending_images(pending_images)?;

        Ok(())
    }

    fn await_for_delete(&self, name: String) {
        std::thread::spawn(async move || {
            sleep(Duration::from_secs(600)).await;
            dispatch!("delete_image", &mut DeleteImageEvent::new(name));
        });
    }
}

