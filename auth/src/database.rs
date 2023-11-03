use anyhow::{Ok, Result};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

use crate::jwt::Jwt;
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
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistedJwt {
    pub id: Uuid,
    pub email: String,
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

    // update password
    pub async fn change_password<'a>(email: &'a str, password: &'a str) -> Result<UserInformation> {
        let database_pool_connection = Database::conn().await;
        let user = sqlx::query_as::<_, UserInformation>(
            r#"
            UPDATE user_information SET password=$1 WHERE email=$2
            RETURNING *
        "#,
        )
        .bind(email)
        .bind(password)
        .fetch_one(&database_pool_connection)
        .await
        .ok()
        .unwrap();

        Ok(user)
    }

    // set verified
    pub async fn set_verified<'a>(email: &'a str) -> Result<bool> {
        let database_pool_connection = Database::conn().await;
        let user = sqlx::query_as::<_, UserInformation>(
            r#"
        ALTER user_information SET is_verified=true WHERE email=$1
        "#,
        )
        .bind(email)
        .fetch_one(&database_pool_connection)
        .await
        .ok();

        Ok(user.is_some())
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
    // creds_exists
    pub async fn creds_exists<'a>(email: &'a str) -> Result<bool> {
        let database_pool_connection = Database::conn().await;
        let user = sqlx::query_as::<_, UserInformation>(
            r#"
        SELECT * FROM user_information WHERE email=$1
        "#,
        )
        .bind(email)
        .fetch_one(&database_pool_connection)
        .await
        .ok();

        Ok(user.is_some())
    }

    pub async fn logout<'a>(token: &'a str) {
        Jwt::blacklist(token).await;
    }
}
