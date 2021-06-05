pub mod integration;
pub mod factories;

use std::env;
use diesel::PgConnection;
use crate::diesel::Connection;

pub fn create_test_db() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    conn.begin_test_transaction().unwrap();
    conn
}