use crate::domain::entities::login;

pub fn mock() -> login::Login {
    login::Login {
        email: "eduardoaikin@gmail.com".to_string(),
        password: "senha123".to_string()
    }
}

pub fn mock_invalid() -> login::Login {
    login::Login {
        email: "eduardoaikin@gmail.com".to_string(),
        password: "senha1234".to_string()
    }
}