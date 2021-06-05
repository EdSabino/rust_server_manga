use async_trait::async_trait;
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};

use super::AuthGuard;
use crate::errors::UnauthorizedError;
use crate::domain::entities::Token;

pub struct FandomGuard(pub Token);

#[async_trait]
impl<'r, 'a> FromRequest<'r, 'a> for FandomGuard {
    type Error = UnauthorizedError;

    fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_guard = req.guard::<AuthGuard>()?;
        let fandom_id: i32 = req.get_param::<i32>(0)
            .and_then(|r| r.ok())
            .unwrap_or(-1);

        let token = auth_guard.0;
        match token.fandoms.iter().find(|item| item.fandom_id == fandom_id) {
            Some(_) => Outcome::Success(FandomGuard(token)),
            None => Outcome::Failure((Status::Unauthorized, UnauthorizedError::RouteNotAllowed))
        }
    }
}
