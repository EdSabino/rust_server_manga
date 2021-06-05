

use std::error::Error;
use diesel::pg::PgConnection;
use validator::{Validate, ValidationErrorsKind};

use super::UseCase;
use super::entities::Manga;
use super::utils::check_and_propagate_image;
use crate::errors::CreateMangaError;
use crate::domain::repository::redis_repository::RedisRepository;

pub struct CreateMangaUseCase {
    redis: RedisRepository
}

impl UseCase<(Manga, i32), Manga> for CreateMangaUseCase {
    fn call(&mut self, conn: &PgConnection, mut params: (Manga, i32)) -> Result<Manga, Box<dyn Error>> {
        params.0.fandom_id = params.1;
        self.validate(&params.0)?;
        check_and_propagate_image(&params.0.photo, &mut self.redis)?;
    
        let get_manga = params.0.save(conn)?;
    
        Ok(get_manga)
    }
}

impl CreateMangaUseCase {
    pub fn new() -> Self {
        CreateMangaUseCase {
            redis: RedisRepository::new().unwrap()
        }
    }

    fn validate(&self, new_user: &Manga) -> Result<(), Box<dyn Error>> {
        match new_user.validate() {
            Ok(_)=> Ok(()),
            Err(e) => {
                match e.errors().get("name").unwrap() {
                    ValidationErrorsKind::Field(_) => Err(Box::new(CreateMangaError::InvalidName)),
                    _ => Err(Box::new(CreateMangaError::UnkownError))
                }
            }
        }
    }
}
