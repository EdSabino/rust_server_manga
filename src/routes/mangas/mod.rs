mod create;

use rocket::Route;

pub fn create_routes() -> Vec<Route> {
    routes![create::create]
}