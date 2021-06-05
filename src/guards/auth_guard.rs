use std::env;
use async_trait::async_trait;
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

use crate::errors::UnauthorizedError;
use crate::domain::entities::Token;

pub struct AuthGuard(pub Token);

#[async_trait]
impl<'r, 'a> FromRequest<'r, 'a> for AuthGuard {
    type Error = UnauthorizedError;

    fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, UnauthorizedError::MissingAuthorization)),
            Some(key) => {
                if key.is_empty() {
                    return Outcome::Failure((Status::Unauthorized, UnauthorizedError::MissingAuthorization));
                }
                match decode::<Token>(&key[7..], &DecodingKey::from_secret(env::var("SECRET_JWT").unwrap().as_ref()), &Validation::new(Algorithm::HS256)) {
                    Ok(token) => Outcome::Success(AuthGuard(token.claims)),
                    Err(_) => Outcome::Failure((Status::Unauthorized, UnauthorizedError::InvalidToken)),
                }
            }
        }
    }
}