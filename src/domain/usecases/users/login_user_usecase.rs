use std::env;
use std::error::Error;
use jsonwebtoken::{ encode, Header, EncodingKey };

use super::entities::{ login, Token, User };
use crate::errors::LoginError;
use crate::diesel::pg::PgConnection;
use super::UseCase;

pub struct LoginUserUseCase {}

impl UseCase<login::Login, login::LoginReturn> for LoginUserUseCase {
    fn call(&mut self, conn: &PgConnection, user: login::Login) -> Result<login::LoginReturn, Box<dyn Error>> {
        let get_user = User::by_email(&user.email, conn)?;
        let fandoms = get_user.fandoms_relation(conn)?;

        if get_user.password == user.passwordh() {
            let token = encode(&Header::default(), &Token::new(get_user, fandoms), &EncodingKey::from_secret(env::var("SECRET_JWT")?.as_ref()))?;
            Ok(login::LoginReturn::new(token))
        } else {
            Err(Box::new(LoginError::WrongPassword))
        }
    }
}

impl LoginUserUseCase {
    pub fn new() -> Self {
        LoginUserUseCase {}
    }
}
