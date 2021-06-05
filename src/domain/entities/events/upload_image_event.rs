use crate::eventbus::Event;

pub struct UploadImageEvent {
    pub name: String,
    pub image: Vec<u8>,
    pub mime: String
}

impl UploadImageEvent {
    pub fn new(name: String, image: Vec<u8>, mime: String) -> Self {
        UploadImageEvent {
            image: image,
            name: name,
            mime: mime
        }
    }
}

impl Event for UploadImageEvent {}