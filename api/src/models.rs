// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationRequest {
    pub first_name: String,
    pub last_name: String,
    pub user_email: String,
    pub birth_date: String,
    pub sex: String,
    pub interests: String,
    pub city: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginRequest {
    pub user_email: String,
    pub password_hash: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationResponse {
    pub status: String,
    pub message: String
}

