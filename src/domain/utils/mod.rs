use crate::domain::repository::redis_repository::RedisRepository;
use crate::domain::entities::events::RemoveImageCacheEvent;
use crate::errors::ImageUploadError;

pub fn check_and_propagate_image(photo: &Option<String>, redis: &mut RedisRepository) -> Result<(), Box<dyn std::error::Error>> {
    match photo {
        Some(photo_uuid) => {
            let mut pending_images: Vec<String> = redis.get_pending_images()?;
            if let Some(pos) = pending_images.iter().position(|image| image == photo_uuid) {
                dispatch!("remove_image_cache", &mut RemoveImageCacheEvent::new(photo_uuid.clone()));
        
                pending_images.remove(pos);
                redis.set_pending_images(pending_images)?;
                return Ok(());
            }
            Err(Box::new(ImageUploadError::NotFound))
        },
        None => Ok(())
    }
}