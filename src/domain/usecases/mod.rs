use std::error::Error;
use crate::diesel::pg::PgConnection;

pub mod users;
pub mod fandoms;
pub mod mangas;
pub mod images;

use super::*;

pub trait UseCase<T, R> {
    fn call(&mut self, conn: &PgConnection, params: T) -> Result<R, Box<dyn Error>>;
}