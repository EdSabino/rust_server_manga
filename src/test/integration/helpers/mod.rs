pub mod test_fairing;
mod valid_user_fairing;

use std::env;
use rocket::local::Client;
use rocket::fairing::AdHoc;
use rocket::{ Request, Data };
use std::collections::HashMap;
use test_fairing::TestTransactionFairing;
use valid_user_fairing::ValidUserFairing;
use rocket::config::{Config, Environment, Value};
use jsonwebtoken::{ encode, Header, EncodingKey };

use crate::rocket;
use crate::domain::entities::{User, UserFromFandom, Token};

pub fn create_test_server() -> Client {
    Client::new(create_test_rocket(|_, _| {})).expect("valid rocket instance")
}

pub fn create_test_server_with_action<F>(f: F) -> Client where
    F: Fn(&mut Request, &Data) + Send + Sync + 'static  {
    Client::new(create_test_rocket(f)).expect("valid rocket instance")
}

pub fn create_test_rocket<F>(f: F) -> rocket::Rocket where 
    F: Fn(&mut Request, &Data) + Send + Sync + 'static  {
    dotenv::dotenv().ok();
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set"));
    databases.insert("postgres_db", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap();
    rocket(rocket::custom(config))
        .attach(TestTransactionFairing {})
        .attach(ValidUserFairing {})
        .attach(AdHoc::on_request("PreAction", f))
}

pub fn valid_token(user: User, fandoms: Vec<UserFromFandom>) -> String {
    encode(&Header::default(), &Token::new(user, fandoms), &EncodingKey::from_secret(env::var("SECRET_JWT").unwrap().as_ref())).unwrap()
}

