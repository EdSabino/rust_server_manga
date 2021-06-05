mod receive_upload_image;
mod receive_delete_image;
mod receive_remove_image_cache;

use crate::domain::entities::events::{
    RemoveImageCacheEvent,
    DeleteImageEvent,
    UploadImageEvent
};

pub fn register_hooks() {
    create_eventbus!("receive_upload_image", UploadImageEvent);
    create_eventbus!("delete_image", DeleteImageEvent);
    create_eventbus!("remove_image_cache", RemoveImageCacheEvent);

    subscribe!("receive_upload_image", receive_upload_image::handler);
    subscribe!("delete_image", receive_delete_image::handler);
    subscribe!("remove_image_cache", receive_remove_image_cache::handler);
}