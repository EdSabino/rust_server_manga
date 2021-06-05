use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;

use crate::PgConn;
use crate::domain::entities::login;
use crate::domain::usecases::users::RecycleTokenUseCase;
use crate::domain::usecases::UseCase;
use crate::guards::AuthGuard;

#[post("/recycle")]
pub fn recycle(conn: PgConn, auth: AuthGuard) -> Result<Json<login::LoginReturn>, BadRequest<String>> {
    let mut usecase = RecycleTokenUseCase::new();

    match usecase.call(&*conn, auth.0.user) {
        Ok(token_response) => Ok(Json(login::LoginReturn::new(token_response))),
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}
