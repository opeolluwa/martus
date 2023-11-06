use crate::database::{Database, Otp};
use anyhow::Result;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

impl Otp {
    pub async fn new(validity: i64) -> Result<Self> {
        let database_pool_connection = Database::conn().await;
        let record_id = Uuid::new_v4();

        let flags = otp_generator::Flags {
            digits: true,
            ..Default::default()
        };
        let otp = otp_generator::generate(6, &flags).unwrap();
        let now = Utc::now().timestamp();
        let exp = now + 60 * 1000 * validity;

        // let exp: NaiveDateTime = NaiveDateTime::date(now).timestamp();
        let otp = sqlx::query_as::<_, Otp>(
            r#"
                INSERT INTO otp (id, exp, otp)
                VALUES ($1, $2, $3)
            "#,
        )
        .bind(record_id)
        .bind(exp)
        .bind(otp)
        .fetch_one(&database_pool_connection)
        .await?;

        Ok(otp)
    }
}
