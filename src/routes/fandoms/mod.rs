mod create;
mod add_user;

use rocket::Route;

pub fn create_routes() -> Vec<Route> {
    routes![create::create, add_user::add_user]
}