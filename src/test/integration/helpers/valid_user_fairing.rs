use rocket::{ Request, Data, Outcome };
use rocket::http::Header;
use rocket::fairing::{ Info, Fairing, Kind };

use crate::PgConn;
use crate::test::factories::{user_factory, fandom_factory};
use super::valid_token;

pub struct ValidUserFairing {}

impl Fairing for ValidUserFairing {
    fn info(&self) -> Info {
        Info {
            name: "ValidUserFairing",
            kind: Kind::Request
        }
    }

    fn on_request(&self, req: &mut Request<'_>, _: &Data) {
        let outcome = req.guard::<PgConn>();
        match outcome {
            Outcome::Success(conn) => {
                let user = user_factory::make_normal_unsaved().save(&*conn).unwrap();
                fandom_factory::mock().save(&*conn, user.id).unwrap();

                let fandom_results = user.fandoms_relation(&*conn).unwrap();

                req.add_header(Header::new("Authorization", format!("Bearer {}", valid_token(user, fandom_results))));
            },
            _ => panic!("Failed to connect to database")
        }
    }
}

