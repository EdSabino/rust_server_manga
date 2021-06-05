use rocket_contrib::json::Json;
use rocket::Request;

use super::error_payload::ErrorPayload;
use crate::domain::entities::User;

#[catch(422)]
pub fn catch(req: &Request) -> Json<ErrorPayload> {
    let auth = req.guard::<User>();
    match auth {
        Outcome::Failure(e) => {
            println!("mds {}", e);
            Json(ErrorPayload::new(e.1.to_string()))
        },
        _ => Json(ErrorPayload::new("Unknown error".to_string()))
    }
}