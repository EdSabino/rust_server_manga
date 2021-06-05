#![feature(async_closure)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(result_contains_err)]

#[cfg(test)]
pub mod test;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate event_bus;

extern crate multipart;
extern crate redis;
extern crate dotenv;

#[macro_use]
pub mod eventbus;
pub mod schema;
pub mod guards;
pub mod errors;
pub mod domain;
mod routes;
mod catchers;
mod cors;
mod hooks;

use rocket_contrib::databases;
use rocket_contrib::serve::StaticFiles;

#[database("postgres_db")]
pub struct PgConn(databases::diesel::PgConnection);

pub fn rocket(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .attach(cors::Cors {})
        .attach(PgConn::fairing())
        .register(catchers::create_catchers())
        .mount("/users", routes::users::create_routes())
        .mount("/fandoms", routes::fandoms::create_routes())
        .mount("/mangas", routes::mangas::create_routes())
        .mount("/upload", routes::multipart::create_routes())
        .mount("/public", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
}

fn main() {
    dotenv::dotenv().ok();
    hooks::register_hooks();
    rocket(rocket::ignite())
        .launch();
}

