

use std::error::Error;

use super::UseCase;
use crate::diesel::pg::PgConnection;
use crate::domain::entities::Fandom;

pub struct AddUserToFandomUseCase {
    fandom_id: i32
}

impl UseCase<i32, ()> for AddUserToFandomUseCase {
    fn call(&mut self, conn: &PgConnection, params: i32) -> Result<(), Box<dyn Error>> {
        Fandom::add_user(conn, self.fandom_id, params)?;
    
        Ok(())
    }
}

impl AddUserToFandomUseCase {
    pub fn new(fandom_id: i32) -> Self {
        AddUserToFandomUseCase {
            fandom_id: fandom_id
        }
    }
}
