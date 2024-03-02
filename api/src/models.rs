// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCraeationRequest {
    pub first_name: String,
    pub last_name: String,
}

pub struct User {
    pub first_name: String,
    pub last_name: String
}
