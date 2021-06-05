use crate::domain::entities::User;

use fake::Fake;
use fake::faker::{ name::raw::*, internet::raw::*, boolean::raw::*};
use fake::locales::*;

pub fn fake_new_user() -> User {
    User {
        id: -1,
        name: Name(EN).fake(),
        email: FreeEmail(EN).fake(),
        password: Password(EN, 8..12).fake(),
        admin: Boolean(EN, 50).fake(),
        photo: None
    }
}

pub fn make_admin_unsaved() -> User {
    User {
        id: -1,
        name: "Eduardo Sabino".to_string(),
        email: "eduardoaikin@gmail.com".to_string(),
        password: "senha123".to_string(),
        admin: true,
        photo: None
    }
}

pub fn make_normal_unsaved() -> User {
    User {
        id: -1,
        name: "Eduardo Sabino".to_string(),
        email: "eduardoaikin@gmail.com".to_string(),
        password: "senha123".to_string(),
        admin: false,
        photo: None
    }
}

pub fn make_invalid_user() -> User {
    User {
        id: -1,
        name: "Eduardo Sabino".to_string(),
        email: "eduardoaikigmail.com".to_string(),
        password: "senha123".to_string(),
        admin: false,
        photo: None
    }
}

pub fn make_normal_from_email(email: String) -> User {
    let mut user = User {
        id: 1,
        name: "Eduardo Sabino".to_string(),
        email: email.clone(),
        password: "senha123".to_string(),
        admin: false,
        photo: None
    };
    user.password = user.passwordh();
    user
}