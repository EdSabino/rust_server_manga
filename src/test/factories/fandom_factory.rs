use crate::domain::entities::Fandom;

use fake::Fake;
use fake::faker::{ name::raw::*, lorem::raw::*};
use fake::locales::*;

pub fn fake_new_fandom() -> Fandom {
    Fandom {
        id: -1,
        name: Name(EN).fake(),
        description: Paragraph(EN, 10..3000).fake(),
        photo: None
    }
}

pub fn mock() -> Fandom {
    Fandom {
        id: -1,
        name: "Fandom".to_string(),
        description: "Meu fandom".to_string(),
        photo: None
    }
}
