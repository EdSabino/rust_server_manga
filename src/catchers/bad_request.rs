use rocket_contrib::json::Json;
use rocket::Request;
use rocket::request::Outcome;

use crate::guards::{AuthGuard, FandomGuard};
use super::error_payload::ErrorPayload;

#[catch(400)]
pub fn catch(req: &Request) -> Json<ErrorPayload> {
    let auth = req.guard::<AuthGuard>();
    
    match auth {
        Outcome::Failure(e) => Json(ErrorPayload::new(e.1.to_string())),
        _ => {
            match req.guard::<FandomGuard>() {
                Outcome::Failure(e) => Json(ErrorPayload::new(e.1.to_string())),
                _ => Json(ErrorPayload::new("Unknown errorasdasd".to_string()))
            }
        }
    }
}
