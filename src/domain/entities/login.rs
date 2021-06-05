use serde::{ Deserialize, Serialize };
use super::traits::HashPassword;

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginReturn {
    pub token: String
}

impl LoginReturn {
    pub fn new(token: String) -> Self {
        LoginReturn {
            token: token
        }
    }
}


impl Login {
    pub fn passwordh(&self) -> String {
        self.hash_password()
    }
}

impl HashPassword for Login {
    fn password(&self) -> String {
        self.password.clone()
    }
}