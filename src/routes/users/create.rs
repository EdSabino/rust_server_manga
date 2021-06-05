use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;

use crate::PgConn;
use crate::domain::entities::User;
use crate::domain::usecases::users::CreateUserUseCase;
use crate::domain::usecases::UseCase;

#[post("/", data = "<user>")]
pub fn create(conn: PgConn, user: Json<User>) -> Result<Json<User>, BadRequest<String>> {
    let mut usecase = CreateUserUseCase::new();
    match usecase.call(&*conn, user.into_inner()) {
        Ok(created_user) => Ok(Json(created_user)),
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}
