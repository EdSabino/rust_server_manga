use serde::{Deserialize, Serialize};

use super::User;
use super::UserFromFandom;

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub user: User,
    pub fandoms: Vec<UserFromFandom>,
    exp: usize
}

impl Token {
    pub fn new(user: User, fandoms: Vec<UserFromFandom>) -> Self {
        Token {
            user: user,
            fandoms: fandoms,
            exp: 10000000000
        }
    }
}