
use demonstrate::demonstrate;

demonstrate! {
    describe "CreateManga" {
        use rocket::Outcome;
        use std::str::FromStr;
        use rocket::http::uri::Origin;
        use rocket::http::{ContentType, Status};

        use crate::PgConn;
        use crate::domain::entities::{Fandom, Manga};
        use crate::test::factories::mangas_factory;
        use super::super::helpers::create_test_server_with_action;
        
        before {
            let client = create_test_server_with_action(|req, _| {
                let outcome = req.guard::<PgConn>();
                match outcome {
                    Outcome::Success(conn) => {
                        let url = Origin::parse_owned(format!("/mangas/{}", Fandom::first(&*conn).unwrap().id)).unwrap();
                        req.set_uri(url);
                    },
                    _ => panic!("Failed to connect to database")
                }
            });
        }

        context "with valid user" {
            before {
                let valid_manga = mangas_factory::mock();
                let mut response = client
                    .post("/mangas")
                    .body(json!(valid_manga).to_string())
                    .dispatch();
            }

            it "have OK status" {
                assert_eq!(response.status(), Status::Ok);
            }

            it "has json content-type" {
                assert_eq!(response.content_type(), Some(ContentType::from_str("application/json").unwrap()));
            }

            it "is same manga" {
                let manga: Manga = serde_json::from_str(&response.body_string().unwrap()).expect("Cannot read");
                assert_eq!(manga, valid_manga);
            }
        }

        context "with invalid payload" {
            before {
                let response = client
                    .post("/mangas/123")
                    .body("{}")
                    .dispatch();
            }
    
            it "has status UnprocessableEntity" {
                assert_eq!(response.status(), Status::UnprocessableEntity);
            }
        }
    }
}
