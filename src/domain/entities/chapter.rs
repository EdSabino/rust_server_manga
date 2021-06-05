use validator::Validate;
use serde::{ Deserialize, Serialize };
use diesel::result::Error as DieselError;
use diesel::{PgConnection, RunQueryDsl};

use crate::schema::chapters;
use super::utils::defautls::default_id;
use crate::schema::chapters::dsl::*;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="chapters"]
struct NewChapter {
    name: String,
    description: String,
    pages: i32,
    manga_id: i32
}

#[derive(Serialize, Deserialize, Queryable, Validate, Debug )]
pub struct Chapter {
    #[serde(default = "default_id")]
    pub id: i32,
    #[validate(length(min = 7))]
    pub name: String,
    pub description: String,
    pub pages: i32,
    #[serde(default = "default_id")]
    pub manga_id: i32 
}

impl Chapter {
    pub fn save(&self, conn: &PgConnection) -> Result<Self, DieselError> {
        if !self.is_new() {
            return Err(DieselError::RollbackTransaction);
        }
        diesel::insert_into(chapters)
            .values(&self.to_new())
            .get_result::<Self>(conn)
    }

    fn to_new(&self) -> NewChapter {
        NewChapter {
            name: self.name.clone(),
            description: self.description.clone(),
            manga_id: self.manga_id.clone(),
            pages: self.pages.clone(),
        }
    }

    fn is_new(&self) -> bool {
        self.id == -1
    }
}

impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.description == other.description
    }
}
