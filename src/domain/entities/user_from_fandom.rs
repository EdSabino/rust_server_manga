use std::fmt;
use serde::{ Deserialize, Serialize };
use diesel::PgConnection;
use diesel::result::Error as DieselError;
use crate::diesel::RunQueryDsl;

use super::User;
use crate::schema::users_from_fandom;

#[derive(Debug)]
#[derive(Insertable)]
#[table_name="users_from_fandom"]
pub struct NewUserFromFandom {
    pub user_id: i32,
    pub fandom_id: i32,
    pub admin: bool
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name="users_from_fandom"]
pub struct UserFromFandom {
    pub id: i32,
    pub user_id: i32,
    pub fandom_id: i32,
    pub admin: bool
}

impl UserFromFandom {
    pub fn new(user_id: i32, fandom_id: i32, admin: bool) -> Self {
        UserFromFandom {
            id: 0,
            user_id: user_id,
            fandom_id: fandom_id,
            admin: admin
        }
    }

    pub fn to_new(&self) -> NewUserFromFandom {
        NewUserFromFandom {
            user_id: self.user_id.clone(),
            fandom_id: self.fandom_id.clone(),
            admin: self.admin.clone()
        }
    }

    pub fn is_new(&self) -> bool {
        self.id == -1
    }

    pub fn save(&self, conn: &PgConnection) -> Result<Self, DieselError> {
        diesel::insert_into(users_from_fandom::table)
            .values(&self.to_new())
            .get_result::<Self>(conn)
    }
}

impl fmt::Display for UserFromFandom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.user_id, self.fandom_id, self.admin)
    }
}

