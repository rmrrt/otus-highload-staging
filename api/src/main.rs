#[macro_use] extern crate rocket;
mod models;
mod utils;
mod create_user;
mod crypt_helper;
mod get_user_by_id;
mod database_error;
mod login_user;

use database_error::DatabaseError;

use sqlx::postgres::PgPoolOptions;
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

use models::{UserCreationRequest, UserCreationResponse, UserLoginRequest, HealthResponse};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use std::env;

use crate::models::{GenericErrorResponse, UserGetByIdResponse, UserLoginResponse};
use crate::get_user_by_id::get_user_by_id;

use sqlx::postgres::{PgPool};
use tokio_postgres::types::ToSql;
use crate::login_user::login_user;

#[get("/health")]
fn health() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "OK".to_string()
    };
    Json(response)
}
#[get("/user/get/<id>")]
async fn get_user(pool: &State<PgPool>, id: i32) -> Result<Json<UserGetByIdResponse>, status::Custom<Json<GenericErrorResponse>>> {
    match get_user_by_id(&pool, &id).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}

#[post("/login", format = "json", data = "<login_request>")]
async fn login(pool: &State<PgPool>, login_request: Json<UserLoginRequest>) -> Result<Json<UserLoginResponse>, status::Custom<Json<UserLoginResponse>>> {
    if login_request.password.is_empty() || login_request.email.is_empty() {
        let error_response = UserLoginResponse {
            id: 0,
            verified: false,
            login_time_stamp: "".to_string(),
            message: "User email and password are required".to_string()
        };
        Err(status::Custom(Status::BadRequest, Json(error_response)))
    } else {
        match login_user(&pool, &login_request).await {
            Ok(response) => Ok(response),
            Err(e) => Err(e)
        }
    }
}

#[post("/user/register", format = "json", data = "<user_request>")]
async fn register(pool: &State<PgPool>, user_request: Json<UserCreationRequest>) -> Result<Json<UserCreationResponse>, status::Custom<Json<UserCreationResponse>>> {
    match create_user::create_user(&pool, user_request).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}

async fn ensure_table_exists(pool: &PgPool) -> Result<(), DatabaseError> {
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => return Err(DatabaseError(format!("Could not get a connection from pool: {:?}", e))),
    };

    match sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users
        (id SERIAL PRIMARY KEY,
        first_name VARCHAR NOT NULL,
        last_name VARCHAR NOT NULL,
        email VARCHAR NOT NULL,
        password VARCHAR NOT NULL,
        birthday DATE NOT NULL,
        city VARCHAR NOT NULL,
        interests VARCHAR NOT NULL,
        sex VARCHAR NOT NULL)")
        .execute(&mut *conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError(format!("Error creating a table as: {:?}", e))),
    }
}

#[launch]
async fn rocket() -> _ {

    let database_url = env::var("DATABASE_URL").unwrap().to_string();
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url).await.expect("Could not connect to DB");

    ensure_table_exists(&pool).await.expect("Could not create table");

    let cors = CorsOptions::default().allowed_origins(AllowedOrigins::all());

    rocket::build()
    .mount("/", routes![health, register, login, get_user])
    .manage(pool)
        .attach(cors.to_cors().unwrap())
}