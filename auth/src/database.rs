use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;
#[derive(Debug)]

/// the database is reusable outside to module
/// especially for sharing the database connection
/// with other modules
pub struct Database {}

impl Database {
    pub async fn conn() -> Pool<Postgres> {
        let database_connection_url =
            std::env::var("DATABASE_URL").expect("error parsing DATABASE_URL");

        let database_pool_connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_connection_url)
            .await
            .expect("error creating connection");

        database_pool_connection
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserInformation {
    id: Uuid,
    email: String,
    password: String,
    is_verified: bool,
}

pub struct UserInformationBuilder<'a>(&'a str, &'a str);
impl<'a> UserInformationBuilder<'a> {
    pub fn new(email: &'a str, password: &'a str) -> Self {
        Self(email, password)
    }
}

impl UserInformation {
    pub async fn new(user: UserInformationBuilder<'_>) -> Self {
        let database_pool_connection = Database::conn().await;
        let user = sqlx::query_as::<_, UserInformation>(
            r#"
            INSERT INTO user_information (email, password)
            VALUES ($1, $2)
            RETURNING id, email, password, is_verified
            "#,
        )
        .bind(user.0)
        .bind(user.1)
        .fetch_one(&database_pool_connection)
        .await
        .expect("error creating user");

        user
    }
}
