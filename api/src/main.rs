#[macro_use] extern crate rocket;
mod models;
mod utils;
mod create_user;
use create_user::create_user;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
type PostgresPool = Pool<PostgresConnectionManager<NoTls>>;
use models::{UserCreationRequest, UserCreationResponse, UserLoginRequest, HealthResponse};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use std::env;
use std::error::Error;

#[get("/health")]
fn health() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "OK".to_string()
    };
    Json(response)
}
#[get("/user/get/<id>")]
fn get_user(id: &str) -> String {
    format!("Hello, user id {}", id)
}

#[post("/login", format = "json", data = "<login_request>")]
fn login(login_request: Json<UserLoginRequest>) -> Result<Json<UserLoginRequest>, status:: Custom<String>> {
    println!("Received request {:?}", login_request);
    if login_request.password_hash.is_empty() {
        Err(status::Custom(Status::BadRequest, "Password is required".to_string()))
    } else {
        Ok(login_request)
    }
}

#[post("/user/register", format = "json", data = "<user_request>")]
async fn register(pool: &State<PostgresPool>, user_request: Json<UserCreationRequest>) -> Result<Json<UserCreationResponse>, status::Custom<String>> {
    let user_creation_response = create_user(pool, user_request).await.expect("Something");
    Ok(user_creation_response)
}

async fn init_db_pool() -> Result<PostgresPool, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let manager = PostgresConnectionManager::new_from_stringlike(
        database_url,
        NoTls,
    ).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Pool::builder().build(manager).await.map_err(|e| Box::new(e) as Box<dyn Error>)
}

async fn ensure_table_exists(pool: &PostgresPool) -> Result<Json<HealthResponse>, status::Custom<Json<HealthResponse>>> {
    let conn = pool.get().await.map_err(|_| {
        let error_response = HealthResponse {
            status: "Error".to_string(),
        };
        status::Custom(Status::InternalServerError, Json(error_response))
    })?;

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, first_name VARCHAR NOT NULL, last_name VARCHAR NOT NULL, birthday VARCHAR NOT NULL, city VARCHAR NOT NULL, interests VARCHAR NOT NULL, sex VARCHAR NOT NULL, email VARCHAR NOT NULL)",
        &[]
    ).await {
        Ok(_) => {
            let response = HealthResponse {
                status: "OK".to_string(),
            };
            Ok(Json(response))
        }
        Err(e) => {
            println!("Error creating table: {}", e);
            let error_response = HealthResponse {
                status: "Error".to_string()
            };
            Err(status::Custom(Status::InternalServerError, Json(error_response)))
        }
    }
}

// fn ensure_table_exists(conn: &mut PgConnection) {
//     sql_query("CREATE TABLE IF NOT EXISTS users (
//         id SERIAL PRIMARY KEY,
//         first_name VARCHAR NOT NULL,
//         last_name VARCHAR NOT NULL,
//         birth_date DATE NOT NULL,
//         sex VARCHAR NOT NULL,
//         interests TEXT,
//         city VARCHAR NOT NULL
//     )").execute(conn).expect("Error creating users table");
// }

#[launch]
async fn rocket() -> _ {

    let pool = init_db_pool().await.expect("database pool");

    ensure_table_exists(&pool).await.expect("Could not connect to DB");

    rocket::build()
    .mount("/", routes![health, get_user, login, register])
    .manage(pool)
}