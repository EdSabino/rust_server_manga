use async_trait::async_trait;
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};

use crate::errors::UnauthorizedError;
use crate::domain::entities::User;
use super::auth_guard::AuthGuard;

pub struct IsAdminGuard(User);

#[async_trait]
impl<'r, 'a> FromRequest<'r, 'a> for IsAdminGuard {
    type Error = UnauthorizedError;

    fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let guard = req.guard::<AuthGuard>()?;

        if guard.0.user.admin {
            Outcome::Success(IsAdminGuard(guard.0.user))
        } else {
            Outcome::Failure((Status::BadRequest, UnauthorizedError::RouteNotAllowed))
        }
    }
}