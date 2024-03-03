#[macro_use] extern crate rocket;
mod models;
use models::{UserCreationRequest, UserLoginRequest};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
type PostgresPool = Pool<PostgresConnectionManager<NoTls>>;
use rocket::State;
use std::env;
use std::error::Error;
use chrono::NaiveDate;


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
async fn register(pool: &State<PostgresPool>, user_request: Json<UserCreationRequest>) -> Result<Json<UserCreationRequest>, status::Custom<String>> {
    let conn = pool.get()
    .await
    .map_err(|_| status::Custom(Status::InternalServerError, "Failed to get DB connection".to_string()))?;

    let formatted_birth_date = parse_date(&user_request.birth_date);

    let query_result = conn.execute(
        "INSERT INTO users (first_name, last_name, birth_date, sex, interests, city, user_email ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[
            &user_request.first_name, 
            &user_request.last_name,
            &formatted_birth_date,
            &user_request.sex,
            &user_request.interests,
            &user_request.city,
            &user_request.user_email
        ]
    ).await
    .map_err(|e| {
        // Log the detailed error message or include it in the response
        eprintln!("Failed to insert user: {}", e);
        status::Custom(Status::InternalServerError, format!("Failed to insert user: {}", e))
    })?;

    Ok(user_request)
}

async fn init_db_pool() -> Result<PostgresPool, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let manager = PostgresConnectionManager::new_from_stringlike(
        database_url,
        NoTls,
    ).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Pool::builder().build(manager).await.map_err(|e| Box::new(e) as Box<dyn Error>)
}

fn parse_date(date_string: &str) -> String {
    let parsed_date = NaiveDate::parse_from_str(date_string, "%a %b %d %Y")
        .expect("Failed to parse date");

    parsed_date.format("%Y-%m-%d").to_string()
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