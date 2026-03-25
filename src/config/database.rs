use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

pub async fn connect() -> DatabaseConnection {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let mut opt = ConnectOptions::new(database_url);

    opt
        .max_connections(10)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(true); //todo: Disable this in production

    Database::connect(opt)
        .await
        .expect("Failed to connect to database")
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::ConnectionTrait;

    #[tokio::test]
    async fn test_connect() {
        let db = connect().await;

        let result = db.execute_unprepared("SELECT 1").await;

        assert!(
            result.is_ok(),
            "Database query failed: {:?}",
            result.err()
        );
    }
}
