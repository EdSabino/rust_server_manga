use rocket_contrib::json::Json;
use rocket::Request;
use rocket::request::Outcome;

use crate::guards::{AuthGuard, FandomGuard, IsAdminGuard, FandomAdminGuard};
use super::error_payload::ErrorPayload;
use crate::errors::UnauthorizedError;

#[catch(401)]
pub fn catch(req: &Request) -> Json<ErrorPayload> {
    match check_guards(req) {
        Outcome::Failure(e) => Json(ErrorPayload::new(e.1.to_string())),
        _ => Json(ErrorPayload::new("Unknown error".to_string()))
    }
}

fn check_guards(req: &Request) -> Outcome<(), UnauthorizedError> {
    req.guard::<AuthGuard>()?;
    req.guard::<FandomGuard>()?;
    req.guard::<IsAdminGuard>()?;
    req.guard::<FandomAdminGuard>()?;
    Outcome::Success(())
}
