use crate::domain::repository::redis_repository::RedisRepository;
use crate::domain::entities::events::RemoveImageCacheEvent;

pub struct RemoveImageFromCacheUseCase {
    redis: RedisRepository,
}

impl RemoveImageFromCacheUseCase {
    pub fn new() -> Self {
        RemoveImageFromCacheUseCase {
            redis: RedisRepository::new().unwrap()
        }
    }

    pub fn call(&mut self, data: &RemoveImageCacheEvent) -> Result<(), Box<dyn std::error::Error>> {
        let mut pending_images: Vec<String> = self.redis.get_pending_images()?;
        if let Some(pos) = pending_images.iter().position(|image| image == &data.name) {
            pending_images.remove(pos);
            self.redis.set_pending_images(pending_images)?;
        }
        Ok(())
    }
}
