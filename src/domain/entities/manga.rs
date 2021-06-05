use validator::Validate;
use serde::{ Deserialize, Serialize };
use diesel::result::Error as DieselError;
use diesel::{PgConnection, RunQueryDsl};

use crate::schema::mangas;
use super::utils::defautls::default_id;
use crate::schema::mangas::dsl::*;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="mangas"]
struct NewManga {
    name: String,
    description: String,
    fandom_id: i32,
    photo: Option<String>
}

#[derive(Serialize, Deserialize, Queryable, Validate, Debug )]
pub struct Manga {
    #[serde(default = "default_id")]
    pub id: i32,
    #[validate(length(min = 7))]
    pub name: String,
    pub description: String,
    #[serde(default = "default_id")]
    pub fandom_id: i32,
    pub photo: Option<String>
}

impl Manga {
    pub fn save(&self, conn: &PgConnection) -> Result<Self, DieselError> {
        if !self.is_new() {
            return Err(DieselError::RollbackTransaction);
        }
        diesel::insert_into(mangas)
            .values(&self.to_new())
            .get_result::<Self>(conn)
    }

    fn to_new(&self) -> NewManga {
        NewManga {
            name: self.name.clone(),
            description: self.description.clone(),
            fandom_id: self.fandom_id.clone(),
            photo: self.photo.clone(),
        }
    }

    fn is_new(&self) -> bool {
        self.id == -1
    }
}

impl PartialEq for Manga {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.description == other.description
    }
}
