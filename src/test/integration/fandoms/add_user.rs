use demonstrate::demonstrate;

demonstrate! {
    describe "AddUserToFandom" {
        use rocket::Outcome;
        use std::str::FromStr;
        use rocket::http::{ContentType, Status, Header};
        use rocket::http::uri::Origin;

        use crate::PgConn;
        use crate::domain::entities::Fandom;
        use crate::test::factories::{user_factory, fandom_factory};
        use super::super::helpers::{create_test_server_with_action, valid_token};

        context "with valid user logged" {
            before {
                let client = create_test_server_with_action(|req, _| {
                    let outcome = req.guard::<PgConn>();
                    match outcome {
                        Outcome::Success(conn) => {
                            let user2 = user_factory::make_normal_from_email("eduardo.aikin2@gmail.com".to_string()).save(&*conn).unwrap();

                            let url = Origin::parse_owned(format!("/fandoms/{}/{}", Fandom::first(&*conn).unwrap().id, user2.id)).unwrap();
                            req.set_uri(url);
                        },
                        _ => panic!("Failed to connect to database")
                    }
                });
            }

            context "with valid fandom and user" {
                before {
                    let mut response = client
                        .put("/fandoms")
                        .body(json!(fandom_factory::mock()).to_string())
                        .dispatch();
                }

                it "have OK status" {
                    assert_eq!(response.status(), Status::Ok);
                }

                it "has json content-type" {
                    assert_eq!(response.content_type(), Some(ContentType::from_str("application/json").unwrap()));
                }

                it "returns result true" {
                    assert_eq!(response.body_string().unwrap(), "{\"result\":true}");
                }
            }
        }

        context "without user" {
            before {
                let client = create_test_server_with_action(|req, _| {
                    let outcome = req.guard::<PgConn>();
                    match outcome {
                        Outcome::Success(conn) => {
                            let url = Origin::parse_owned(format!("/fandoms/{}/232", Fandom::first(&*conn).unwrap().id)).unwrap();
                            req.set_uri(url);
                        },
                        _ => panic!("Failed to connect to database")
                    }
                });
            }

            context "with valid fandom and invalid user" {
                before {
                    let mut response = client
                        .put("/fandoms")
                        .body(json!(fandom_factory::mock()).to_string())
                        .dispatch();
                }

                it "have BadRequest status" {
                    assert_eq!(response.status(), Status::BadRequest);
                }

                it "has plain content-type" {
                    assert_eq!(response.content_type(), Some(ContentType::Plain));
                }
            }
        }
        context "with user not from fandom" {
            before {
                let client = create_test_server_with_action(|req, _| {
                    let outcome = req.guard::<PgConn>();
                    match outcome {
                        Outcome::Success(conn) => {
                            let user = user_factory::make_normal_from_email("eduardo.aikin@gmail.com".to_string()).save(&*conn).unwrap();
                            let user2 = user_factory::make_normal_from_email("eduardo.aikin1@gmail.com".to_string()).save(&*conn).unwrap();
                            let fandom = fandom_factory::mock().save(&*conn, user.id).unwrap();

                            let url = Origin::parse_owned(format!("/fandoms/{}/{}", fandom.id, user2.id)).unwrap();
                            req.add_header(Header::new("Authorization", format!("Bearer {}", valid_token(user2, vec![]))));
                            
                            req.set_uri(url);
                        },
                        _ => panic!("Failed to connect to database")
                    }
                });
            }

            context "with valid fandom and invalid user" {
                before {
                    let mut response = client
                        .put("/fandoms")
                        .body(json!(fandom_factory::mock()).to_string())
                        .dispatch();
                }

                it "have BadRequest status" {
                    assert_eq!(response.status(), Status::Unauthorized);
                }

                it "has plain content-type" {
                    assert_eq!(response.content_type(), Some(ContentType::from_str("application/json").unwrap()));
                }
            }
        }
    }
}
