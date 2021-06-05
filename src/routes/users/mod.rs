mod create;
mod login;
mod recycle;

use rocket::Route;

pub fn create_routes() -> Vec<Route> {
    routes![create::create, login::login, recycle::recycle]
}