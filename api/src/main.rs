#[macro_use] extern crate rocket;
mod models;
use models::UserCraeationRequest;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year named {}!", age, name)
}

#[post("/user/register", format = "json", data = "<user_request>")]
fn register(user_request: Json<UserCraeationRequest>) -> Result<Json<UserCraeationRequest>, status::Custom<String>> {

    println!("Received request: {:?}", user_request);

    if user_request.first_name.is_empty() {  // Example validation
        Err(status::Custom(Status::BadRequest, "Username is required".to_string()))
    } else {
        Ok(user_request)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, register])
}