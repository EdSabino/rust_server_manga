use crate::domain::entities::events::RemoveImageCacheEvent;
use crate::domain::usecases::images::RemoveImageFromCacheUseCase;

pub fn handler(message: &mut RemoveImageCacheEvent) {
    RemoveImageFromCacheUseCase::new().call(message).unwrap();
}