#[macro_use] extern crate rocket;
mod models;
mod utils;
mod create_user;
use create_user::create_user;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
type PostgresPool = Pool<PostgresConnectionManager<NoTls>>;
use models::{UserCreationRequest, UserCreationResponse, UserLoginRequest};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use std::env;
use std::error::Error;


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

    rocket::build()
    .mount("/", routes![get_user, login, register])
    .manage(pool)
}