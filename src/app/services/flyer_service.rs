use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::flyers as Flyer;

pub struct FlyerService;

#[async_trait]
impl CrudService<Flyer::Entity> for FlyerService {}

impl FlyerService {}

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
