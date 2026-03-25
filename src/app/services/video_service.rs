use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::videos as Video;

pub struct VideoService;

#[async_trait]
impl CrudService<Video::Entity> for VideoService {}

impl VideoService {}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Set, IntoActiveModel};
    use crate::config::database::connect;

    #[tokio::test]
    async fn test_video_crud_operations() {
        // 1. Establish Database Connection
        let db = connect().await;

        // 2. CREATE
        let new_video = Video::ActiveModel {
            name: Set("Integration Test Video".to_owned()),
            url: Set("https://youtube.com/watch?v=integration_test".to_owned()),
            description: Set(Some("Automated test video entry".to_owned())),
            event_title: Set(Some("Test Driven Development".to_owned())),
            thumbnail_url: Set(Some("https://example.com/thumb.jpg".to_owned())),
            short_info: Set(Some("Just testing".to_owned())),
            ..Default::default()
        };

        let inserted_video = VideoService::create(&db, new_video)
            .await
            .expect("Failed to create video");

        println!("Created Video ID: {}", inserted_video.id);
        assert_eq!(inserted_video.name, "Integration Test Video");
        assert_eq!(inserted_video.url, "https://youtube.com/watch?v=integration_test");

        // 3. READ
        let found_video = VideoService::find_by_id(&db, inserted_video.id)
            .await
            .expect("Database error during find")
            .expect("Video not found in database");

        assert_eq!(found_video.clone().event_title.unwrap(), "Test Driven Development");

        // 4. UPDATE
        // Convert the Model back into an ActiveModel so we can modify it
        let mut active_video = found_video.into_active_model();
        active_video.name = Set("Updated Integration Test Video".to_owned());
        active_video.is_active = Set(false); // Testing boolean update

        let updated_video = VideoService::update(&db, active_video)
            .await
            .expect("Failed to update video");

        assert_eq!(updated_video.name, "Updated Integration Test Video");
        assert_eq!(updated_video.is_active, false);

        // 5. DELETE
        // Convert back to ActiveModel for deletion
        let active_video_to_delete = updated_video.into_active_model();
        let delete_result = VideoService::delete(&db, active_video_to_delete)
            .await
            .expect("Failed to delete video");

        assert_eq!(delete_result.rows_affected, 1);

        // Verify it is actually gone
        let check_deleted = VideoService::find_by_id(&db, inserted_video.id)
            .await
            .unwrap();
        assert!(check_deleted.is_none(), "Video should be deleted!");
    }
}

