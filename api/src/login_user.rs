use rocket::{State, http::Status, response::status};
use crate::models::{UserLoginRequest, UserLoginResponse, UserPasswordCheck};
use rocket::serde::{json::Json};
use sqlx::PgPool;
use crate::crypt_helper::verify_password;
use crate::utils::get_current_time;

pub async fn login_user(pool: &State<PgPool>, login_request: &UserLoginRequest) -> Result<Json<UserLoginResponse>, status::Custom<Json<UserLoginResponse>>> {
    let user = sqlx::query_as::<_,UserPasswordCheck>("SELECT id, email, password from users where email = $1")
        .bind(&login_request.email)
        .fetch_one(pool.inner())
        .await;

    let mut error_response = UserLoginResponse {
        id: 0,
        login_time_stamp: get_current_time(),
        verified: false,
        message: "".to_string()
    };

    match user {
        Ok(user_password_check) => {

            let verified = verify_password(&login_request.password, &user_password_check.password).unwrap();
            if verified {
                let response = UserLoginResponse {
                    id: user_password_check.id,
                    login_time_stamp: get_current_time(),
                    verified: true,
                    message: "".to_string()
                };

                Ok(Json(response))
            } else {
                error_response.message = "Not authorized".to_string();
                Err(status::Custom(Status::Forbidden, Json(error_response)))
            }
        }
        Err(_) => {
            error_response.message = "User not found".to_string();
            Err(status::Custom(Status::Forbidden, Json(error_response)))
        }
    }
}