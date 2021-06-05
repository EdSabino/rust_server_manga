use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;
use serde::Serialize;

use crate::PgConn;
use crate::domain::usecases::fandoms::AddUserToFandomUseCase;
use crate::domain::usecases::UseCase;
use crate::guards::FandomAdminGuard;

#[derive(Serialize)]
pub struct Success {
    result: bool
}

#[put("/<fandom>/<user_id>")]
pub fn add_user(conn: PgConn, fandom: i32, user_id: i32, _fandom_guard: FandomAdminGuard) -> Result<Json<Success>, BadRequest<String>> {
    let mut usecase = AddUserToFandomUseCase::new(fandom);
    match usecase.call(&*conn, user_id) {
        Ok(_) => Ok(Json(Success { result: true })),
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}
