mod users;

use axum::{routing::get, Router};
use sqlx::{Executor, ConnectOptions};
use sqlx::postgres::{PgConnectOptions, PgConnection, PgSslMode};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::users::__path_get_user_handler;


#[derive(OpenApi)]
#[openapi(paths(get_user_handler), components(schemas(users::User)))]
struct ApiDoc;

async fn make_connection() -> PgConnection {
    PgConnectOptions::new()
        .host("127.0.0.1")
        .port(5432)
        .username("simplechat")
        .password("simplechat")
        .ssl_mode(PgSslMode::Disable)
        .connect()
        .await.expect("Failed to connect to database.")
}

#[tokio::main]
async fn main() {
    let mut postgres_client = make_connection().await;

    postgres_client.execute("
    CREATE TABLE IF NOT EXISTS users (
        id      SERIAL PRIMARY KEY,
        name    varchar(20) NOT NULL
    )    
    ").await.unwrap();

    let app = Router::new()
        .route("/users/:id", get(users::get_user_handler))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
