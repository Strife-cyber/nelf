use std::time::SystemTime;
use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::flyers as Flyer;
use super::super::requests::store_flyer_request::ParsedFlyerData;

pub struct FlyerService;

#[async_trait]
impl CrudService<Flyer::Entity> for FlyerService {}

impl FlyerService {
    /// Creates a new flyer record in the database after uploading its file to S3.
    /// 
    /// # Arguments
    /// * `db` - Database connection reference
    /// * `s3_client` - AWS S3 client for file storage
    /// * `parsed_data` - Data extracted from the multipart form request
    pub async fn create_with_file(
        db: &sea_orm::DatabaseConnection,
        s3_client: &aws_sdk_s3::Client,
        mut parsed_data: ParsedFlyerData
    ) -> anyhow::Result<Flyer::Model> {
        let name = parsed_data.name.clone().ok_or_else(|| anyhow::anyhow!("Name is required"))?;
        
        // Use .take() to move the stream out of parsed_data without moving the whole struct
        let file_stream = parsed_data.file_stream.take().ok_or_else(|| anyhow::anyhow!("File is required"))?;

        let original_file_name = parsed_data.file_name.clone().unwrap_or_else(|| "unnamed_upload".to_string());

        // Generate a unique filename using timestamp to avoid collisions in S3
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let file_name = format!("{}_{}", timestamp, original_file_name.replace(" ", "_"));

        // Upload to S3 under the 'flyers/' prefix using streaming ByteStream
        let s3_key = format!("flyers/{}", file_name);
        crate::config::filesystems::upload(s3_client, &s3_key, file_stream).await?;

        // Construct the full public URL for the uploaded file
        let url = std::env::var("AWS_URL").unwrap_or_else(|_| "".to_string());
        let bucket = std::env::var("AWS_BUCKET").unwrap_or_else(|_| "".to_string());

        let s3_url = format!("{}/{}/{}", url, bucket, s3_key);

        // Convert the remaining parsed data and the new S3 URL into a SeaORM ActiveModel
        let active_model = parsed_data.into_active_model(name, s3_url);

        // Persist to the database
        let flyer = Self::create(db, active_model).await?;

        Ok(flyer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Set, IntoActiveModel};
    use crate::config::database::connect;

    #[tokio::test]
    async fn test_flyer_crud_operations() {
        // 1. Establish Database Connection
        let db = connect().await;

        // 2. CREATE
        let new_flyer = Flyer::ActiveModel {
            name: Set("Test Flyer".to_owned()),
            url: Set("https://example.com/test.pdf".to_owned()),
            description: Set(Some("A flyer created for testing".to_owned())),
            event_title: Set(Some("Test Event".to_owned())),
            ..Default::default()
        };

        let inserted_flyer = FlyerService::create(&db, new_flyer)
            .await
            .expect("Failed to create flyer");

        println!("Created Flyer ID: {}", inserted_flyer.id);
        assert_eq!(inserted_flyer.name, "Test Flyer");

        // 3. READ
        let found_flyer = FlyerService::find_by_id(&db, inserted_flyer.id)
            .await
            .expect("Database error during find")
            .expect("Flyer not found in database");

        assert_eq!(found_flyer.url, "https://example.com/test.pdf");

        // 4. UPDATE
        // Convert the Model back into an ActiveModel so we can modify it
        let mut active_flyer = found_flyer.into_active_model();
        active_flyer.name = Set("Updated Test Flyer".to_owned());

        let updated_flyer = FlyerService::update(&db, active_flyer)
            .await
            .expect("Failed to update flyer");

        assert_eq!(updated_flyer.name, "Updated Test Flyer");

        // 5. DELETE
        // Convert back to ActiveModel for deletion
        let active_flyer_to_delete = updated_flyer.into_active_model();
        let delete_result = FlyerService::delete(&db, active_flyer_to_delete)
            .await
            .expect("Failed to delete flyer");

        assert_eq!(delete_result.rows_affected, 1);

        // Verify it's actually gone
        let check_deleted = FlyerService::find_by_id(&db, inserted_flyer.id)
            .await
            .unwrap();
        assert!(check_deleted.is_none(), "Flyer should be deleted!");
    }
}
