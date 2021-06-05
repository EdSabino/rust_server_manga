use rocket::{ Request, Data, Outcome };
use crate::rocket::fairing::{ Info, Fairing, Kind };
use diesel::connection::Connection;

use crate::PgConn;

pub struct TestTransactionFairing {}

impl Fairing for TestTransactionFairing {
    fn info(&self) -> Info {
        Info {
            name: "TestTransactionFairing",
            kind: Kind::Request
        }
    }

    fn on_request(&self, req: &mut Request<'_>, _: &Data) {
        let outcome = req.guard::<PgConn>();
        match outcome {
            Outcome::Success(conn) => {
                (*conn).begin_test_transaction().unwrap();
            },
            _ => panic!("Failed to connect to database")
        }
    }
}
