use std::sync::Arc;
use aws_sdk_s3::Client;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub s3_client: Arc<Client>
}
