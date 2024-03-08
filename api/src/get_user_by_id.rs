use rocket::{State, http::Status, response::status};
use rocket::serde::{json::Json};
use sqlx::PgPool;
use crate::models::{GenericErrorResponse, UserGetByIdResponse};

pub async fn get_user_by_id(
    pool: &State<PgPool>,
    id: &i32
) -> Result<Json<UserGetByIdResponse>, status::Custom<Json<GenericErrorResponse>>> {
    let result = sqlx::query_as::<_, UserGetByIdResponse>(
        "SELECT id, first_name, last_name, email, birthday::text, sex, interests, city FROM users WHERE id = $1"
    )
        .bind(id)
        .fetch_one(pool.inner())
        .await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(sqlx::Error::RowNotFound) => Err(status::Custom(
            Status::NotFound,
            Json(GenericErrorResponse {
                error: "User not found".to_string(),
            }),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            Json(GenericErrorResponse {
                error: "Failed to execute query".to_string(),
            }),
        )),
    }
}
