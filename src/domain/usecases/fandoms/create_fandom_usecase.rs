

use std::error::Error;
use validator::Validate;

use super::entities::{ Fandom, User };
use super::UseCase;
use crate::diesel::pg::PgConnection;

pub struct CreateFandomUseCase {}

impl UseCase<(Fandom, User), Fandom> for CreateFandomUseCase {
    fn call(&mut self, conn: &PgConnection, params: (Fandom, User)) -> Result<Fandom, Box<dyn Error>> {
        params.0.validate()?;
    
        let get_fandom = params.0.save(conn, params.1.id)?;
    
        Ok(get_fandom)
    }
}

impl CreateFandomUseCase {
    pub fn new() -> Self {
        CreateFandomUseCase {}
    }
}
