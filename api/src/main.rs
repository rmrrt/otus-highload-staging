#[macro_use] extern crate rocket;
mod models;
use models::{UserCreationRequest, UserLoginRequest};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;
use diesel::RunQueryDsl;

use rocket_sync_db_pools::{database,diesel};

#[database("otus_highload")]
struct Db(diesel::PgConnection);


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
fn register(user_request: Json<UserCreationRequest>) -> Result<Json<UserCreationRequest>, status::Custom<String>> {

    println!("Received request: {:?}", user_request);

    if user_request.first_name.is_empty() { 
        Err(status::Custom(Status::BadRequest, "Username is required".to_string()))
    } else {
        Ok(user_request)
    }
}

fn ensure_table_exists(conn: &mut PgConnection) {
    sql_query("CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        first_name VARCHAR NOT NULL,
        last_name VARCHAR NOT NULL,
        birth_date DATE NOT NULL,
        sex VARCHAR NOT NULL,
        interests TEXT,
        city VARCHAR NOT NULL
    )").execute(conn).expect("Error creating users table");
}

#[launch]
fn rocket() -> _ {

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url).expect("Error connecting to database");
    ensure_table_exists(&mut conn);

    rocket::build()
    .attach(Db::fairing())
    .mount("/", routes![get_user, login, register])
}