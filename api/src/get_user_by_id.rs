use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use rocket::{State, http::Status, response::status};
use rocket::serde::{json::Json};
use crate::models::{GenericErrorResponse, UserGetByIdResponse};

type PostgresPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn get_user_by_id(
    pool: &State<PostgresPool>,
    id: &i32
) -> Result<UserGetByIdResponse, status::Custom<Json<GenericErrorResponse>>> {
    let conn = pool.get().await.map_err(|_| status::Custom(
        Status::InternalServerError,
        Json(GenericErrorResponse {
            error: "Failed to fetch connection from pool".to_string(),
        }),
    ))?;

    let stmt = conn.prepare("SELECT * FROM users WHERE id = $1").await.map_err(|_| status::Custom(
        Status::InternalServerError,
        Json(GenericErrorResponse {
            error: "Failed to prepare query".to_string(),
        }),
    ))?;

    match conn.query_one(&stmt, &[&id]).await {
        Ok(row) => {
            let user = UserGetByIdResponse {
                id: row.get("id"),
                // Populate other fields of User from the row
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
                birthday: row.get("birthday"),
                sex: row.get("sex"),
                interests: row.get("interests"),
                city: row.get("city"),
            };
            Ok(user)
        },
        Err(e) => {
            if e.code() == Some(&tokio_postgres::error::SqlState::NO_DATA_FOUND) {
                Err(status::Custom(
                    Status::NotFound,
                    Json(GenericErrorResponse {
                        error: "User not found".to_string(),
                    }),
                ))
            } else {
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(GenericErrorResponse {
                        error: "Query execution failed".to_string(),
                    }),
                ))
            }
        },
    }
}
