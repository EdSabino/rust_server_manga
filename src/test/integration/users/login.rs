use demonstrate::demonstrate;

demonstrate! {
    describe "LoginUser" {
        use std::env;
        use std::str::FromStr;
        use rocket::http::{ContentType, Status};
        use jsonwebtoken::{ decode, DecodingKey, Validation, Algorithm };

        use crate::domain::entities::{Token, login::LoginReturn};
        use crate::test::factories::login_factory;
        use super::super::helpers::create_test_server;

        before {
            let client = create_test_server();
        }

        context "with valid login" {
            before {
                let mut response = client
                    .post("/users/login")
                    .body(json!(login_factory::mock()).to_string())
                    .dispatch();
            }

            it "have OK status" {
                assert_eq!(response.status(), Status::Ok);
            }

            it "has json content-type" {
                assert_eq!(response.content_type(), Some(ContentType::from_str("application/json").unwrap()));
            }

            it "has same email" {
                let login: LoginReturn = serde_json::from_str(&response.body_string().unwrap()).expect("Cannot read");
                let token = decode::<Token>(&login.token, &DecodingKey::from_secret(env::var("SECRET_JWT").unwrap().as_ref()), &Validation::new(Algorithm::HS256)).unwrap();
                assert_eq!(token.claims.user.email, "eduardoaikin@gmail.com".to_string());
            }
        }

        context "with invalid login" {
            before {
                let mut response = client
                    .post("/users/login")
                    .body(json!(login_factory::mock_invalid()).to_string())
                    .dispatch();
            }
            it "have BadRequest status" {
                assert_eq!(response.status(), Status::BadRequest);
            }
            
            it "has plain content-type" {
                assert_eq!(response.content_type(), Some(ContentType::Plain));
            }
            
            it "has wrong password message" {
                assert_eq!(response.body_string(), Some("Wrong password".to_string()));
            }
        }

        context "with invalid payload" {
            before {
                let response = client
                    .post("/users/login")
                    .body("{}")
                    .dispatch();
            }
    
            it "has status UnprocessableEntity" {
                assert_eq!(response.status(), Status::UnprocessableEntity);
            }
        }
    }
}
