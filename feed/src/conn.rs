use std::collections::HashMap;
use std::time::Duration;

use snafu::ResultExt;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySqlPool, PgPool, Row};
use tracing::info;

use crate::error::{DatabaseConnectSnafu, DatabaseRequestSnafu, FeedResult};

#[derive(Debug, Clone)]
pub struct DbConn {
    pool: MySqlPool,
    db_addr: String,
}

impl DbConn {
    /// Create the database connection.
    ///
    /// Environment variables `DATABASE_URL` must be set. The format is
    /// ```text
    /// postgres://postgres:password@localhost/test
    /// ```
    pub async fn new() -> FeedResult<DbConn> {
        let db_addr = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_addr)
            .await
            .context(DatabaseConnectSnafu)?;
        info!("Connected to database");

        Ok(Self { pool, db_addr })
    }

    pub async fn execute(&self, req: &str) -> FeedResult<()> {
        // sqlx::query(req)
        //     .fetch_one(&self.pool)
        //     .await
        //     .context(DatabaseRequestSnafu)?;
        let mut params = HashMap::new();
        let client = reqwest::Client::new();
        params.insert("sql", req);
        let result = client
            .post("http://localhost:4000/v1/sql?db=public")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await;
        info!("request result: {:?}", result);
        info!("{}", result.unwrap().text().await.unwrap());
        Ok(())
    }
}
