use demonstrate::demonstrate;

demonstrate! {
    describe "CreateUser" {
        use std::str::FromStr;
        use rocket::http::{ContentType, Status};
        
        use super::super::helpers::create_test_server;
        use crate::test::factories::user_factory;
        use crate::domain::entities::User;
        
        before {
            let client = create_test_server();
        }

        context "with valid user" {
            before {
                let valid_user = user_factory::fake_new_user();
                let mut response = client
                    .post("/users")
                    .body(json!(valid_user).to_string())
                    .dispatch();
            }

            it "have OK status" {
                assert_eq!(response.status(), Status::Ok);
            }

            it "has json content-type" {
                assert_eq!(response.content_type(), Some(ContentType::from_str("application/json").unwrap()));
            }

            it "is same user" {
                let user: User = serde_json::from_str(&response.body_string().unwrap()).expect("Cannot read");
                assert_eq!(user, valid_user);
            }
        }

        context "with invalid user" {
            before {
                let response = client
                    .post("/users")
                    .body(json!(user_factory::make_invalid_user()).to_string())
                    .dispatch();
            }

            it "have BadRequest status" {
                assert_eq!(response.status(), Status::BadRequest);
            }

            it "has plain content-type" {
                assert_eq!(response.content_type(), Some(ContentType::Plain));
            }
        }

        context "with invalid payload" {
            before {
                let response = client
                    .post("/users")
                    .body("{}")
                    .dispatch();
            }
    
            it "has status UnprocessableEntity" {
                assert_eq!(response.status(), Status::UnprocessableEntity);
            }
        }
    }
}
