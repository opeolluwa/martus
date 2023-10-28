use anyhow::{Ok, Result};
use bcrypt::{hash, DEFAULT_COST};
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
            std::env::var("DATABASE_URL").unwrap_or("postgres://opeolluwa:thunderstorm@localhost/martus_auth".to_string());

        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_connection_url)
            .await
            .expect("error creating connection")
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
    pub async fn new(user: UserInformationBuilder<'_>) -> Result<Self> {
        let database_pool_connection = Database::conn().await;

        //hash the password
        let hashed_password = hash(user.1, DEFAULT_COST)?;

        let new_user = sqlx::query_as::<_, UserInformation>(
            r#"
            INSERT INTO user_information (email, password)
            VALUES ($1, $2)
            RETURNING id, email, is_verified
            "#,
        )
        .bind(user.0)
        .bind(hashed_password)
        .fetch_one(&database_pool_connection)
        .await?;

        Ok(new_user)
    }
}
