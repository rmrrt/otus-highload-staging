use super::models::{UserCreationRequest, UserCreationResponse};
use super::utils::parse_date;
use super::crypt_helper::hash_password;

use rocket::http::Status;
use rocket::State;
use rocket::response::status;
use rocket::serde::json::Json;
use sqlx::{PgPool};
use chrono::NaiveDate;

#[post("/create_user", format = "json", data = "<user_request>")]
pub async fn create_user(pool: &State<PgPool>, user_request: Json<UserCreationRequest>) -> Result<Json<UserCreationResponse>, status::Custom<Json<UserCreationResponse>>> {
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(_) => {
            let error_response = UserCreationResponse {
                status: "Error".to_string(),
                message: "Failed to get DB connection".to_string(),
            };
            return Err(status::Custom(Status::InternalServerError, Json(error_response)));
        },
    };

    let user_exists = sqlx::query("SELECT id from users WHERE email = $1")
        .bind(&user_request.email)
        .fetch_one(pool.inner())
        .await;

    match user_exists {
        Ok(user) => {
            let error_response = UserCreationResponse {
                status: "Error".to_string(),
                message: "User already exists".to_string()
            };
            return Err(status::Custom(Status::Conflict, Json(error_response)))
        }
        Err(_) => {
            println!("The user doesn't exist, we can create one.")
        }
    }


    let password_hash = match hash_password(&user_request.password) {
        Ok(hash) => hash,
        Err(_) => {
            let error_response = UserCreationResponse {
                status: "Error".to_string(),
                message: "Failed to hash password".to_string(),
            };
            return Err(status::Custom(Status::InternalServerError, Json(error_response)));
        },
    };

    let result = sqlx::query(
        "INSERT INTO users
        (first_name, last_name, email, password, birthday, sex, interests, city)
        VALUES ($1, $2, $3, $4, $5::date, $6, $7, $8)"
    )
        .bind(&user_request.first_name)
        .bind(&user_request.last_name)
        .bind(&user_request.email)
        .bind(&password_hash)
        .bind(&user_request.birthday)
        .bind(&user_request.sex)
        .bind(&user_request.interests)
        .bind(&user_request.city)
        .execute(&mut *conn)
        .await;

    match result {
        Ok(_) => {
            let response = UserCreationResponse {
                status: "OK".to_string(),
                message: "User created successfully".to_string(),
            };
            Ok(Json(response))
        },
        Err(e) => {
            eprintln!("Failed to insert user: {}", e);
            let error_response = UserCreationResponse {
                status: "Error".to_string(),
                message: format!("Failed to insert user: {}", e),
            };
            Err(status::Custom(Status::InternalServerError, Json(error_response)))
        }
    }
}
