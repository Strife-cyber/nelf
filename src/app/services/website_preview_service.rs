use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::website_previews as WebsitePreview;

pub struct WebsitePreviewService;

#[async_trait]
impl CrudService<WebsitePreview::Entity> for WebsitePreviewService {}

impl WebsitePreviewService {}

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
