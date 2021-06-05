use rocket::Catcher;

mod unauthorized;
pub mod error_payload;

pub fn create_catchers() -> Vec<Catcher> {
    catchers![
        unauthorized::catch
    ]
}