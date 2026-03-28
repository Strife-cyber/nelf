use std::time::SystemTime;
use async_trait::async_trait;
use sea_orm::IntoActiveModel;
use super::crud::CrudService;

use super::super::entities::website_previews as WebsitePreview;
use super::super::requests::store_website_preview_request::ParsedWebsitePreviewData;

pub struct WebsitePreviewService;

#[async_trait]
impl CrudService<WebsitePreview::Entity> for WebsitePreviewService {}

impl WebsitePreviewService {
    /// Creates a new website preview record in the database after optionally uploading an image to S3.
    pub async fn create_with_file(
        db: &sea_orm::DatabaseConnection,
        s3_client: &aws_sdk_s3::Client,
        mut parsed_data: ParsedWebsitePreviewData
    ) -> anyhow::Result<WebsitePreview::Model> {
        let url = parsed_data.url.clone().ok_or_else(|| anyhow::anyhow!("URL is required"))?;
        
        let mut uploaded_image_url = None;

        if let Some(image_stream) = parsed_data.image_stream.take() {
            let original_file_name = parsed_data.image_name.clone().unwrap_or_else(|| "unnamed_preview".to_string());

            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            let file_name = format!("{}_{}", timestamp, original_file_name.replace(" ", "_"));

            let s3_key = format!("previews/{}", file_name);
            crate::config::filesystems::upload(s3_client, &s3_key, image_stream).await?;

            let s3_url = std::env::var("AWS_URL").unwrap_or_else(|_| "".to_string());
            let bucket = std::env::var("AWS_BUCKET").unwrap_or_else(|_| "".to_string());

            uploaded_image_url = Some(format!("{}/{}/{}", s3_url, bucket, s3_key));
        }

        let active_model = parsed_data.into_active_model(url, uploaded_image_url);

        let preview = Self::create(db, active_model).await?;

        Ok(preview)
    }

    pub async fn update_with_file(
        db: &sea_orm::DatabaseConnection,
        s3_client: &aws_sdk_s3::Client,
        id: i32,
        mut parsed_data: ParsedWebsitePreviewData
    ) -> anyhow::Result<WebsitePreview::Model> {
        let preview = Self::find_by_id(db, id).await?.ok_or_else(|| anyhow::anyhow!("Website preview not found"))?;
        
        let mut uploaded_image_url = None;

        if let Some(image_stream) = parsed_data.image_stream.take() {
            let original_file_name = parsed_data.image_name.clone().unwrap_or_else(|| "unnamed_preview".to_string());
            let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
            let file_name = format!("{}_{}", timestamp, original_file_name.replace(" ", "_"));
            let s3_key = format!("previews/{}", file_name);
            
            crate::config::filesystems::upload(s3_client, &s3_key, image_stream).await?;

            let s3_url = std::env::var("AWS_URL").unwrap_or_else(|_| "".to_string());
            let bucket = std::env::var("AWS_BUCKET").unwrap_or_else(|_| "".to_string());
            uploaded_image_url = Some(format!("{}/{}/{}", s3_url, bucket, s3_key));
        }

        let active_model = parsed_data.update_active_model(preview.into_active_model(), uploaded_image_url);
        let updated_preview = Self::update(db, active_model).await?;

        Ok(updated_preview)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Set, IntoActiveModel};
    use crate::config::database::connect;

    #[tokio::test]
    async fn test_website_preview_crud_operations() {
        // 1. Establish Database Connection
        let db = connect().await;

        // 2. CREATE
        let new_preview = WebsitePreview::ActiveModel {
            url: Set("https://rust-lang.org/test".to_owned()),
            title: Set(Some("Rust Test Page".to_owned())),
            description: Set(Some("A temporary page for integration testing.".to_owned())),
            content_type: Set("website".to_owned()),
            image_url: Set(Some("https://rust-lang.org/logo.png".to_owned())),
            favicon_url: Set(Some("https://rust-lang.org/favicon.ico".to_owned())),
            ..Default::default()
        };

        let inserted_preview = WebsitePreviewService::create(&db, new_preview)
            .await
            .expect("Failed to create website preview");

        println!("Created Preview ID: {}", inserted_preview.id);
        assert_eq!(inserted_preview.url, "https://rust-lang.org/test");
        assert_eq!(inserted_preview.content_type, "website");

        // 3. READ
        let found_preview = WebsitePreviewService::find_by_id(&db, inserted_preview.id)
            .await
            .expect("Database error during find")
            .expect("Preview not found in database");

        assert_eq!(found_preview.clone().title.unwrap(), "Rust Test Page");

        // 4. UPDATE
        // Convert the Model back into an ActiveModel so we can modify it
        let mut active_preview = found_preview.into_active_model();
        active_preview.title = Set(Some("Updated Rust Test Page".to_owned()));
        active_preview.content_type = Set("documentation".to_owned());

        let updated_preview = WebsitePreviewService::update(&db, active_preview)
            .await
            .expect("Failed to update website preview");

        assert_eq!(updated_preview.clone().title.unwrap(), "Updated Rust Test Page");
        assert_eq!(updated_preview.content_type, "documentation");

        // 5. DELETE
        // Convert back to ActiveModel for deletion
        let active_preview_to_delete = updated_preview.into_active_model();
        let delete_result = WebsitePreviewService::delete(&db, active_preview_to_delete)
            .await
            .expect("Failed to delete website preview");

        assert_eq!(delete_result.rows_affected, 1);

        // Verify it is actually gone
        let check_deleted = WebsitePreviewService::find_by_id(&db, inserted_preview.id)
            .await
            .unwrap();
        assert!(check_deleted.is_none(), "Website preview should be deleted!");
    }
}
