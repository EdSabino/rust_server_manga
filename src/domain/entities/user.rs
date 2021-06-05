use validator::Validate;
use serde::{ Deserialize, Serialize };
use diesel::result::Error as DieselError;
use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::BelongingToDsl;

use crate::schema::users;
use super::utils::defautls::{default_id, default_bool};
use super::traits::HashPassword;
use super::UserFromFandom;

#[derive(Debug)]
#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
    pub photo: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Validate, Queryable, Identifiable)]
pub struct User {
    #[serde(default = "default_id")]
    pub id: i32,
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[serde(default = "default_bool")]
    pub admin: bool,
    pub photo: Option<String>
}

impl User {
    pub fn by_email(user_email: &String, conn: &PgConnection) -> Result<Self, DieselError> {
        users::table.filter(users::email.eq(user_email))
            .limit(1)
            .get_result::<Self>(conn)
    }

    pub fn by_id(id_user: i32, conn: &PgConnection) -> Result<Self, DieselError> {
        users::table.filter(users::id.eq(id_user))
            .limit(1)
            .get_result::<Self>(conn)
    }

    pub fn passwordh(&self) -> String {
        self.hash_password()
    }

    pub fn to_new(&self) -> NewUser {
        NewUser {
            name: self.name.clone(),
            email: self.email.clone(),
            password: self.passwordh(),
            admin: self.admin.clone(),
            photo: self.photo.clone()
        }
    }

    pub fn is_new(&self) -> bool {
        self.id == -1
    }

    pub fn save(&self, conn: &PgConnection) -> Result<Self, DieselError> {
        diesel::insert_into(users::table)
            .values(&self.to_new())
            .get_result::<Self>(conn)
    }

    pub fn fandoms_relation(&self, conn: &PgConnection) -> Result<Vec<UserFromFandom>, diesel::result::Error> {
        UserFromFandom::belonging_to(self)
            .get_results::<UserFromFandom>(conn)
        //users_from_fandom::table.filter(users_from_fandom::user_id.eq(self.id))
    }
}

impl HashPassword for User {
    fn password(&self) -> String {
        self.password.clone()
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email && self.name == other.name && self.admin == other.admin
    }
}

#[cfg(test)]
use demonstrate::demonstrate;

#[cfg(test)]
demonstrate! {
    describe "User" {
        use super::*;
        use crate::test::{create_test_db, factories::user_factory::fake_new_user, factories::fandom_factory::mock};
        use validator::Validate;

        context "When new user" {
            before {
                let user = fake_new_user();
            }

            it "is new" {
                assert!(user.is_new());
            }

            it "is valid" {
                assert!(user.validate().is_ok());
            }

            it "can be hash password" {
                let hashed = user.passwordh();
                assert_ne!(hashed, user.password);
            }

            it "can create NewUser" {
                let new_user = user.to_new();
                assert_eq!(new_user.email, user.email);
            }

            context "#save" {
                before {
                    let conn = create_test_db();
                }

                it "can save" {
                    assert!(user.save(&conn).is_ok());
                }

                it "returns old User" {
                    let created_user = user.save(&conn).unwrap();
                    assert!(!created_user.is_new());
                }
            }
        }

        context "with fandom" {
            before {
                let user = fake_new_user();
                let fandom = mock();
                let conn = create_test_db();
                let created_user = user.save(&conn).unwrap();
                let fandom_id = fandom.save(&conn, created_user.id).unwrap().id;
            }

            it "find a fandom" {
                assert!(created_user.fandoms_relation(&conn).is_ok());
            }
            
            it "is created fandom" {
                assert_eq!(created_user.fandoms_relation(&conn).unwrap()[0].fandom_id, fandom_id);
            }

            it "is admin" {
                assert!(created_user.fandoms_relation(&conn).unwrap()[0].admin);
            }
        }

        context "by_id" {
            before {
                let user = fake_new_user();
                let conn = create_test_db();
                let user_id = user.save(&conn).unwrap().id;
            }

            it "must be successful" {
                assert!(User::by_id(user_id, &conn).is_ok());
            }

            it "must be equal to user" {
                assert_eq!(User::by_id(user_id, &conn), Ok(user));
            }
        }

        context "by_email" {
            before {
                let user = fake_new_user();
                let conn = create_test_db();
                let user_email = user.save(&conn).unwrap().email;
            }

            it "must be successful" {
                assert!(User::by_email(&user_email, &conn).is_ok());
            }

            it "must be equal to user" {
                assert_eq!(User::by_email(&user_email, &conn), Ok(user));
            }
        }
    }
}