use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;

use crate::PgConn;
use crate::domain::entities::Fandom;
use crate::domain::usecases::fandoms::CreateFandomUseCase;
use crate::domain::usecases::UseCase;
use crate::guards::AuthGuard;

#[post("/", data = "<fandom>")]
pub fn create(conn: PgConn, fandom: Json<Fandom>, auth: AuthGuard) -> Result<Json<Fandom>, BadRequest<String>> {
    let mut usecase = CreateFandomUseCase::new();
    match usecase.call(&*conn, (fandom.into_inner(), auth.0.user)) {
        Ok(created_fandom) => Ok(Json(created_fandom)),
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}
