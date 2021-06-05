use demonstrate::demonstrate;

demonstrate! {
    describe "RecycleToken" {
        use std::env;
        use std::str::FromStr;
        use rocket::http::{ContentType, Status, Header};
        use jsonwebtoken::{ decode, DecodingKey, Validation, Algorithm };

        use crate::domain::entities::{Token, login::LoginReturn};
        use super::super::helpers::{create_test_server_with_action, create_test_server};

        context "with valid token" {
            before {
                let client = create_test_server();
                let mut response = client
                    .post("/users/recycle")
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

        context "without token" {
            use crate::catchers::error_payload::ErrorPayload;

            before {
                let client = create_test_server_with_action(|req, _| {
                    req.replace_header(Header::new("Authorization", "Bearer "));
                });
                let mut response = client
                    .post("/users/recycle")
                    .dispatch();
            }
            it "have Unauthorized status" {
                assert_eq!(response.status(), Status::Unauthorized);
            }
            
            it "has json content-type" {
                assert_eq!(response.content_type(), Some(ContentType::from_str("application/json").unwrap()));
            }
            
            it "has wrong password message" {
                let error: ErrorPayload = serde_json::from_str(&response.body_string().unwrap()).expect("Cannot read");
                assert_eq!(error.message, "Invalid token".to_string());
            }
        }
    }
}
