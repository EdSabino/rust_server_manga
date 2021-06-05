use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;

use crate::PgConn;
use crate::domain::entities::Manga;
use crate::domain::usecases::mangas::CreateMangaUseCase;
use crate::domain::usecases::UseCase;
use crate::guards::FandomGuard;

#[post("/<fandom_id>", data = "<manga>")]
pub fn create(conn: PgConn, manga: Json<Manga>, fandom_id: i32, _fandom_guard: FandomGuard) -> Result<Json<Manga>, BadRequest<String>> {
    let mut usecase = CreateMangaUseCase::new();
    match usecase.call(&*conn, (manga.into_inner(), fandom_id)) {
        Ok(created_manga) => Ok(Json(created_manga)),
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}
