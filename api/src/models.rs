// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserCraeationRequest {
    first_name: String,
    last_name: String,
}

pub struct User {
    first_name: String,
    last_name: String
}
