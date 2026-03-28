use std::time::SystemTime;
use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::users as User;
use super::super::requests::store_user_request::ParsedUserData;

pub struct UserService;

#[async_trait]
impl CrudService<User::Entity> for UserService {}

impl UserService {
    /// Creates a new user record in the database after optionally uploading an avatar to S3.
    pub async fn create_with_file(
        db: &sea_orm::DatabaseConnection,
        s3_client: &aws_sdk_s3::Client,
        mut parsed_data: ParsedUserData
    ) -> anyhow::Result<User::Model> {
        let name = parsed_data.name.clone().ok_or_else(|| anyhow::anyhow!("Name is required"))?;
        let email = parsed_data.email.clone().ok_or_else(|| anyhow::anyhow!("Email is required"))?;
        
        let mut uploaded_avatar_url = None;

        if let Some(avatar_stream) = parsed_data.avatar_stream.take() {
            let original_file_name = parsed_data.avatar_name.clone().unwrap_or_else(|| "unnamed_avatar".to_string());

            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            let file_name = format!("{}_{}", timestamp, original_file_name.replace(" ", "_"));

            let s3_key = format!("users/avatars/{}", file_name);
            crate::config::filesystems::upload(s3_client, &s3_key, avatar_stream).await?;

            let s3_url = std::env::var("AWS_URL").unwrap_or_else(|_| "".to_string());
            let bucket = std::env::var("AWS_BUCKET").unwrap_or_else(|_| "".to_string());

            uploaded_avatar_url = Some(format!("{}/{}/{}", s3_url, bucket, s3_key));
        }

        let active_model = parsed_data.into_active_model(name, email, uploaded_avatar_url);

        let user = Self::create(db, active_model).await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Set, IntoActiveModel};
    use crate::config::database::connect;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn test_user_crud_operations() {
        // 1. Establish Database Connection
        let db = connect().await;

        // Generate a unique email to prevent unique constraint violations on repeated test runs
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let test_email = format!("test_user_{}@nelf.com", timestamp);

        // 2. CREATE
        let new_user = User::ActiveModel {
            name: Set("Test Driven User".to_owned()),
            email: Set(test_email.clone()),
            role: Set(Some("Admin".to_owned())),
            initials: Set(Some("TD".to_owned())),
            // Testing the PostgreSQL Array type
            skills: Set(Some(vec!["Rust".to_owned(), "Testing".to_owned(), "SeaORM".to_owned()])),
            ..Default::default()
        };

        let inserted_user = UserService::create(&db, new_user)
            .await
            .expect("Failed to create user");

        println!("Created User ID: {}", inserted_user.id);
        assert_eq!(inserted_user.name, "Test Driven User");
        assert_eq!(inserted_user.email, test_email);

        // 3. READ
        let found_user = UserService::find_by_id(&db, inserted_user.id)
            .await
            .expect("Database error during find")
            .expect("User not found in database");

        // Verify the array data came back correctly
        let skills = found_user.skills.clone().unwrap();
        assert!(skills.contains(&"SeaORM".to_string()));

        // 4. UPDATE
        // Convert the Model back into an ActiveModel so we can modify it
        let mut active_user = found_user.into_active_model();
        active_user.name = Set("Updated Test User".to_owned());
        active_user.role = Set(Some("Super Admin".to_owned()));

        let updated_user = UserService::update(&db, active_user)
            .await
            .expect("Failed to update user");

        assert_eq!(updated_user.name, "Updated Test User");
        assert_eq!(updated_user.clone().role.unwrap(), "Super Admin");

        // 5. DELETE
        // Convert back to ActiveModel for deletion
        let active_user_to_delete = updated_user.into_active_model();
        let delete_result = UserService::delete(&db, active_user_to_delete)
            .await
            .expect("Failed to delete user");

        assert_eq!(delete_result.rows_affected, 1);

        // Verify it is actually gone
        let check_deleted = UserService::find_by_id(&db, inserted_user.id)
            .await
            .unwrap();
        assert!(check_deleted.is_none(), "User should be deleted!");
    }
}
