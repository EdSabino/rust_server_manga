use std::env;
use std::error::Error;
use jsonwebtoken::{ encode, Header, EncodingKey };

use super::entities::{ Token, User };
use crate::diesel::pg::PgConnection;
use super::UseCase;

pub struct RecycleTokenUseCase {}

impl UseCase<User, String> for RecycleTokenUseCase {
    fn call(&mut self, conn: &PgConnection, user: User) -> Result<String, Box<dyn Error>> {
        let get_user = User::by_id(user.id, conn)?;
        let fandoms = get_user.fandoms_relation(conn)?;
        let token = encode(&Header::default(), &Token::new(get_user, fandoms), &EncodingKey::from_secret(env::var("SECRET_JWT")?.as_ref()))?;
        Ok(token)
    }
}

impl RecycleTokenUseCase {
    pub fn new() -> Self {
        RecycleTokenUseCase {}
    }
}
