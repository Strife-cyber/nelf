use std::sync::Arc;
use axum::extract::Multipart;
use sea_orm::IntoActiveModel;
use axum::{Extension, Json, http::StatusCode};
use aws_sdk_s3::primitives::ByteStream;
use futures_util::StreamExt;
use std::io::Write;

use crate::{
    AppState,
    app::entities::users,
    app::services::crud::CrudService,
    app::services::user_service::UserService,
    app::requests::store_user_request::{StoreUserRequest, ParsedUserData}
};

pub struct UserController;

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List all users", body = [users::Model])
    )
)]
pub async fn list_users(
    state: Extension<Arc<AppState>>
) -> Result<Json<Vec<users::Model>>, StatusCode> {
    UserController::list(state).await
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body(content = StoreUserRequest, content_type="multipart/form-data"),
    responses(
        (status = 201, description = "User created successfully", body = users::Model)
    )
)]
pub async fn create_user(
    state: Extension<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<users::Model>, StatusCode> {
    UserController::create(state, multipart).await
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = users::Model),
        (status = 404, description = "User not found")
    )
)]
pub async fn find_user(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
) -> Result<Json<users::Model>, StatusCode> {
    UserController::find(state, id).await
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found")
    )
)]
pub async fn delete_user(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
) -> Result<Json<&'static str>, StatusCode> {
    UserController::delete(state, id).await
}

impl UserController {
    pub async fn list(
        Extension(state): Extension<Arc<AppState>>
    ) -> Result<Json<Vec<users::Model>>, StatusCode> {
        let users = UserService::list_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(users))
    }

    pub async fn create(
        Extension(state): Extension<Arc<AppState>>,
        mut multipart: Multipart
    ) -> Result<Json<users::Model>, StatusCode> {
        let mut parsed_data = ParsedUserData::default();

        while let Ok(Some(mut field)) = multipart.next_field().await {
            let field_name = field.name().unwrap_or("").to_string();

            if field_name == "avatar" {
                parsed_data.avatar_name = field.file_name().map(|s| s.to_string());
                
                let mut temp_file = tempfile::NamedTempFile::new()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                    temp_file.write_all(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                }

                let stream = ByteStream::from_path(temp_file.path())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                parsed_data.avatar_stream = Some(stream);
            } else if let Ok(text) = field.text().await {
                match field_name.as_str() {
                    "name" => parsed_data.name = Some(text),
                    "email" => parsed_data.email = Some(text),
                    "phone" => parsed_data.phone = Some(text),
                    "role" => parsed_data.role = Some(text),
                    "description" => parsed_data.description = Some(text),
                    "initials" => parsed_data.initials = Some(text),
                    "is_active" => parsed_data.is_active = text.parse().ok(),
                    "skills" => {
                        let skills: Vec<String> = text.split(',').map(|s| s.trim().to_string()).collect();
                        parsed_data.skills = Some(skills);
                    }
                    _ => {}
                }
            }
        }

        let user = UserService::create_with_file(&state.db, &state.s3_client, parsed_data)
            .await
            .map_err(|e| {
                println!("Error creating user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(Json(user))
    }

    pub async fn find(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<users::Model>, StatusCode> {
        let user = UserService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        Ok(Json(user))
    }

    pub async fn delete(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<&'static str>, StatusCode> {
        let user = UserService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        UserService::delete(&state.db, user.into_active_model())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json("Deleted"))
    }
}
