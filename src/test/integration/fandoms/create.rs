use demonstrate::demonstrate;

demonstrate! {
    describe "CreateFandom" {
        use std::str::FromStr;
        use rocket::http::{ContentType, Status};

        use crate::test::factories::fandom_factory;
        use super::super::helpers::{create_test_server, create_test_server_with_action };
        use crate::domain::entities::Fandom;

        context "with valid user logged" {
            before {
                let client = create_test_server();
            }

            context "with valid fandom" {
                before {
                    let valid_fandom = fandom_factory::fake_new_fandom();
                    let mut response = client
                        .post("/fandoms")
                        .body(json!(valid_fandom).to_string())
                        .dispatch();
                }

                it "have OK status" {
                    assert_eq!(response.status(), Status::Ok);
                }

                it "has json content-type" {
                    assert_eq!(response.content_type(), Some(ContentType::from_str("application/json").unwrap()));
                }

                it "return created fandom" {
                    let fandom: Fandom = serde_json::from_str(&response.body_string().unwrap()).expect("Cannot read");
                    assert_eq!(fandom, valid_fandom);
                }
            }
        }

        context "without valid user logged" {
            use rocket::http::Header;

            before {
                let client = create_test_server_with_action(|req, _| {
                    req.replace_header(Header::new("Authorization", ""))
                });
            }

            context "with valid fandom" {
                use crate::catchers::error_payload::ErrorPayload;
                before {
                    let mut response = client
                        .post("/fandoms")
                        .body(json!(fandom_factory::mock()).to_string())
                        .dispatch();
                }

                it "have UNAUTHORIZED status" {
                    assert_eq!(response.status(), Status::Unauthorized);
                }

                it "has json content-type" {
                    assert_eq!(response.content_type(), Some(ContentType::from_str("application/json").unwrap()));
                }

                it "return created fandom" {
                    let fandom: ErrorPayload = serde_json::from_str(&response.body_string().unwrap()).expect("Cannot read");
                    assert_eq!(fandom.message, "Missing `Authorization` header".to_string());
                }
            }
        }

        context "with invalid payload" {
            before {
                let client = create_test_server();
                let response = client
                    .post("/fandoms")
                    .body("{}")
                    .dispatch();
            }
    
            it "has status UnprocessableEntity" {
                assert_eq!(response.status(), Status::UnprocessableEntity);
            }
        }
    }
}
