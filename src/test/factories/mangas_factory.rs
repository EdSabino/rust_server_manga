use fake::Fake;
use fake::faker::{ name::raw::*, lorem::raw::*};
use fake::locales::*;

use crate::domain::entities::Manga;

pub fn mock() -> Manga {
    Manga {
        id: -1,
        name: Name(EN).fake(),
        description: Paragraph(EN, 10..3000).fake(),
        fandom_id: -1,
        photo: None
    }
}
