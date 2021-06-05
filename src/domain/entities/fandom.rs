use validator::Validate;
use serde::{ Deserialize, Serialize };
use diesel::result::Error as DieselError;
use diesel::{PgConnection, RunQueryDsl, Connection, QueryDsl};

use crate::schema::fandoms::dsl::*;
use crate::schema::users_from_fandom::dsl::*;
use crate::schema::fandoms;
use super::utils::defautls::default_id;
use super::UserFromFandom;

#[derive(Insertable, Debug)]
#[table_name="fandoms"]
pub struct NewFandom {
    pub name: String,
    pub description: String,
    pub photo: Option<String>
}

#[derive(Serialize, Deserialize, Validate, Queryable, Debug)]
pub struct Fandom {
    #[serde(default = "default_id")]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub photo: Option<String>
}

impl Fandom {
    pub fn first(conn: &PgConnection) -> Result<Fandom, DieselError> {
        fandoms::table
            .limit(1)
            .get_result::<Self>(conn)
    }

    pub fn add_user(conn: &PgConnection, fandom: i32, user: i32) -> Result<UserFromFandom, DieselError> {
        let user_fandom = UserFromFandom::new(user, fandom, false);

        diesel::insert_into(users_from_fandom)
            .values(&user_fandom.to_new())
            .get_result::<UserFromFandom>(conn)
    }

    pub fn save(&self, conn: &PgConnection, user: i32) -> Result<Fandom, DieselError> {
        conn.transaction::<Fandom, diesel::result::Error, _>(|| {
            let new_fandom = diesel::insert_into(fandoms)
                .values(&self.to_new())
                .get_result::<Fandom>(conn)?;
            let user_fandom = UserFromFandom::new(user, new_fandom.id, true);
        
            diesel::insert_into(users_from_fandom)
                .values(&user_fandom.to_new())
                .get_result::<UserFromFandom>(conn)?;

            Ok(new_fandom)
        })
    }

    pub fn to_new(&self) -> NewFandom {
        NewFandom {
            name: self.name.clone(),
            description: self.description.clone(),
            photo: self.photo.clone()
        }
    }

    pub fn is_new(&self) -> bool {
        self.id == -1
    }
}

impl PartialEq for Fandom {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.description == other.description
    }
}