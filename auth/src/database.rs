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
        let database_connection_url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://opeolluwa:thunderstorm@localhost/martus_auth".to_string());

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

        let id = Uuid::new_v4();
        let email = user.0;
        let password = hash(user.1, DEFAULT_COST)?;

        let new_user = sqlx::query_as::<_, UserInformation>(
            r#"
            INSERT INTO user_information (id, email, password)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(email)
        .bind(password)
        .fetch_one(&database_pool_connection)
        .await?;

        Ok(new_user)
    }

    // get a user record
    pub async fn fetch<'a>(email: &'a str) -> Result<Self> {
        let database_pool_connection = Database::conn().await;
        let user = sqlx::query_as::<_, UserInformation>(
            r#"
        SELECT * FROM user_information WHERE email=$1
        "#,
        )
        .bind(email)
        .fetch_one(&database_pool_connection)
        .await?;

        Ok(user)
    }

    pub async fn validate_password<'a>(&self, password: &'a str) -> Result<bool> {
        let database_pool_connection = Database::conn().await;
        let user = sqlx::query_as::<_, UserInformation>(
            r#"
        SELECT * FROM user_information WHERE email=$1
        "#,
        )
        .bind(&self.email)
        .fetch_one(&database_pool_connection)
        .await?;

        Ok(bcrypt::verify(password, &user.password)?)
    }
    // return the user without the password
    pub fn serialize() {}

    // update password
    pub fn change_password() {}

    // set verified
    pub fn verify() {}
}
