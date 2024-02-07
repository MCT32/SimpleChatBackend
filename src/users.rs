use axum::{body::Body, extract::Path, http::{Response, StatusCode}};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Row};
use utoipa::ToSchema;


#[derive(Deserialize, Serialize, ToSchema)]
pub struct User {
    id: i32,
    name: String,
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = StatusCode::OK, description = "User found succesfully", body = User),
        (status = StatusCode::NOT_FOUND, description = "User could not be found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Id of user being requested")
    )
)]
pub async fn get_user_handler(Path(id): Path<i32>) -> Response<Body> {
    match get_user(id).await {
        Ok(user) => {
            match user {
                Some(user) => Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&user).unwrap())).unwrap(),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("User not found")).unwrap(),
            }
        },
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(())).unwrap()
        }
    }
}

async fn get_user(id: i32) -> Result<Option<User>, sqlx::Error> {
    let mut postgres_client = super::make_connection().await;

    match postgres_client.fetch_one(format!("SELECT * FROM users WHERE id = {}", id).as_str()).await {
        Ok(row) => {
            Ok(Some(User {
                id: row.get("id"),
                name: row.get("name")
            }))
        },
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(err) => Err(err)
    }
}
