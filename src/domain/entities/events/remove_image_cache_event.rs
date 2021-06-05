use crate::eventbus::Event;

pub struct RemoveImageCacheEvent {
    pub name: String
}

impl RemoveImageCacheEvent {
    pub fn new(name: String) -> Self {
        RemoveImageCacheEvent {
            name: name
        }
    }
}

impl Event for RemoveImageCacheEvent {}