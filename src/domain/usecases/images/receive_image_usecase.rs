use std::{fs, error::Error};
use uuid::Uuid;
use rocket::Data;
use multipart::server::{ Multipart, save::{SavedData, SavedField} };

use crate::errors::ImageUploadError;
use crate::domain::entities::events::UploadImageEvent;

pub struct ReceiveImageUseCase {
    uuid: String
}

impl ReceiveImageUseCase {
    pub fn new() -> Self {
        ReceiveImageUseCase {
            uuid: Uuid::new_v4().hyphenated().to_string()
        }
    }

    pub fn call(&self, data: Data, boundary: &str) -> Result<String, ImageUploadError> {
        match self.proccess(data, boundary) {
            Ok(_) => Ok(self.uuid.clone()),
            Err(_) => Err(ImageUploadError::NotFound)
        }
    }

    fn proccess(&self, data: Data, boundary: &str) -> Result<(), Box<dyn Error>> {
        let entries = Multipart::with_body(data.open(), boundary).save().temp().into_result()?;
        let field = entries.fields.get("image").ok_or("no image")?.get(0).ok_or("no field")?;
        match &field.data {
            SavedData::Text(_) => Err(Box::new(ImageUploadError::NotFound)),
            SavedData::Bytes(bytes) => {
                self.post_event(bytes.to_vec(), &field);
                Ok(())
            },
            SavedData::File(path, _) => {
                let file: Vec<u8> = fs::read(path).unwrap();
                self.post_event(file, &field);
                Ok(())
            }
        }
    }

    fn post_event(&self, file: Vec<u8>, field: &SavedField) {
        let mime = field.headers.content_type.clone();
        dispatch!("receive_upload_image", &mut UploadImageEvent::new(self.uuid.clone(), file, mime.unwrap().essence_str().to_string()));
    }
}
