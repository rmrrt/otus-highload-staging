// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationRequest {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginRequest {
    pub user_email: String,
    pub password_hash: String
}

pub struct User {
    pub first_name: String,
    pub last_name: String
}
