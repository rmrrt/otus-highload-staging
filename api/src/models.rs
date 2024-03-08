use chrono::{DateTime, NaiveDate};
// src/models.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub birthday: String,
    pub sex: String,
    pub interests: String,
    pub city: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGetByIdRequest {
    pub id: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GenericErrorResponse {
    pub error: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserGetByIdResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birthday: String,
    pub sex: String,
    pub interests: String,
    pub city: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub id: i32,
    pub verified: bool,
    pub login_time_stamp: String,
    pub message: String
}
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserPasswordCheck {
    pub id: i32,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationResponse {
    pub status: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String
}
