use super::models::{UserCreationRequest,UserCreationResponse};
use super::utils::parse_date;
use super::crypt_helper::hash_password;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
type PostgresPool = Pool<PostgresConnectionManager<NoTls>>;
use rocket::http::Status;
use rocket::State;
use rocket::response::status;
use rocket::serde::json::Json;
pub async fn create_user(
    pool: &State<PostgresPool>, 
    user_request: Json<UserCreationRequest>
) -> Result<Json<UserCreationResponse>, status::Custom<Json<UserCreationResponse>>> {
    let conn = pool.get().await.map_err(|_| {
        let error_response = UserCreationResponse {
            status: "Error".to_string(),
            message: "Failed to get DB connection".to_string(),
        };
        status::Custom(Status::InternalServerError, Json(error_response))
    })?;

    let formatted_birth_date = parse_date(&user_request.birthday);
    let password_hash =  hash_password(&user_request.password).expect("Error hashing password");

    match conn.execute(
        "INSERT INTO users (first_name, last_name, email, password, birthday, sex, interests, city) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[
            &user_request.first_name,
            &user_request.last_name,
            &user_request.email,
            &password_hash,
            &formatted_birth_date,
            &user_request.sex,
            &user_request.interests,
            &user_request.city,
        ],
    ).await {
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
