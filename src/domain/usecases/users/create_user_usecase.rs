

use std::error::Error;
use validator::Validate;
use validator::ValidationErrorsKind;

use super::UseCase;
use super::entities::User;
use crate::diesel::pg::PgConnection;
use crate::errors::CreateUserError;

pub struct CreateUserUseCase {}

impl UseCase<User, User> for CreateUserUseCase {
    fn call(&mut self, conn: &PgConnection, user: User) -> Result<User, Box<dyn Error>> {
        self.validate(&user)?;
    
        let get_user = user.save(conn)?;
    
        Ok(get_user)
    }
}

impl CreateUserUseCase {
    pub fn new() -> Self {
        CreateUserUseCase {}
    }

    fn validate(&self, new_user: &User) -> Result<(), Box<dyn Error>> {
        match new_user.validate() {
            Ok(_)=> Ok(()),
            Err(e) => {
                match e.errors().get("email").unwrap() {
                    ValidationErrorsKind::Field(_) => Err(Box::new(CreateUserError::InvalidEmail)),
                    _ => Err(Box::new(CreateUserError::UnkownError))
                }
            }
        }
    }
}
