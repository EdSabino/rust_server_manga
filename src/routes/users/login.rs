use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;

use crate::PgConn;
use crate::domain::entities::login;
use crate::domain::usecases::users::LoginUserUseCase;
use crate::domain::usecases::UseCase;

#[post("/login", data = "<login>")]
pub fn login(conn: PgConn, login: Json<login::Login>) -> Result<Json<login::LoginReturn>, BadRequest<String>> {
    let mut usecase = LoginUserUseCase::new();

    match usecase.call(&*conn, login.into_inner()) {
        Ok(token_response) => Ok(Json(token_response)),
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}
