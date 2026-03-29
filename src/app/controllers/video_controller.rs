use std::sync::Arc;
use axum::extract::Multipart;
use sea_orm::IntoActiveModel;
use axum::{Extension, Json, http::StatusCode};
use aws_sdk_s3::primitives::ByteStream;
use futures_util::StreamExt;
use std::io::Write;

use crate::{
    AppState,
    app::entities::videos,
    app::services::crud::CrudService,
    app::services::video_service::VideoService,
    app::requests::store_video_request::{StoreVideoRequest, UpdateVideoRequest, ParsedVideoData}
};

pub struct VideoController;

// List all videos
#[utoipa::path(
    get,
    path = "/api/videos",
    responses(
        (status = 200, description = "List all videos", body = [videos::Model])
    )
)]
pub async fn list_videos(
    state: Extension<Arc<AppState>>
) -> Result<Json<Vec<videos::Model>>, StatusCode> {
    VideoController::list(state).await
}

// Create a new video
#[utoipa::path(
    post,
    path = "/api/videos",
    request_body(content = StoreVideoRequest, content_type="multipart/form-data"),
    responses(
        (status = 201, description = "Video created successfully", body = videos::Model)
    )
)]
pub async fn create_video(
    state: Extension<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<videos::Model>, StatusCode> {
    VideoController::create(state, multipart).await
}

// Find by ID
#[utoipa::path(
    get,
    path = "/api/videos/{id}",
    params(
        ("id" = i32, Path, description = "Video ID")
    ),
    responses(
        (status = 200, description = "Video found", body = videos::Model),
        (status = 404, description = "Video not found")
    )
)]
pub async fn find_video(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
) -> Result<Json<videos::Model>, StatusCode> {
    VideoController::find(state, id).await
}

#[utoipa::path(
    put,
    path = "/api/videos/{id}",
    params(
        ("id" = i32, Path, description = "Video ID")
    ),
    request_body(content = UpdateVideoRequest, content_type="multipart/form-data"),
    responses(
        (status = 200, description = "Video updated successfully", body = videos::Model),
        (status = 404, description = "Video not found")
    )
)]
pub async fn update_video(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
    multipart: Multipart,
) -> Result<Json<videos::Model>, StatusCode> {
    VideoController::update(state, id, multipart).await
}

// Delete
#[utoipa::path(
    delete,
    path = "/api/videos/{id}",
    params(
        ("id" = i32, Path, description = "Video ID")
    ),
    responses(
        (status = 200, description = "Video deleted successfully"),
        (status = 404, description = "Video not found")
    )
)]
pub async fn delete_video(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
) -> Result<Json<&'static str>, StatusCode> {
    VideoController::delete(state, id).await
}

impl VideoController {
    pub async fn list(
        Extension(state): Extension<Arc<AppState>>
    ) -> Result<Json<Vec<videos::Model>>, StatusCode> {
        let videos = VideoService::list_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(videos))
    }

    pub async fn create(
        Extension(state): Extension<Arc<AppState>>,
        mut multipart: Multipart
    ) -> Result<Json<videos::Model>, StatusCode> {
        let parsed_data = Self::parse_multipart(&mut multipart).await?;

        let video = VideoService::create_with_file(&state.db, &state.s3_client, parsed_data)
            .await
            .map_err(|e| {
                println!("Error creating video: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(Json(video))
    }

    pub async fn update(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
        mut multipart: Multipart
    ) -> Result<Json<videos::Model>, StatusCode> {
        let parsed_data = Self::parse_multipart(&mut multipart).await?;

        let video = VideoService::update_with_file(&state.db, &state.s3_client, id, parsed_data)
            .await
            .map_err(|e| {
                println!("Error updating video: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(Json(video))
    }

    async fn parse_multipart(multipart: &mut Multipart) -> Result<ParsedVideoData, StatusCode> {
        let mut parsed_data = ParsedVideoData::default();

        while let Ok(Some(mut field)) = multipart.next_field().await {
            let field_name = field.name().unwrap_or("").to_string();

            if field_name == "thumbnail" {
                parsed_data.thumbnail_name = field.file_name().map(|s| s.to_string());
                
                let mut temp_file = tempfile::NamedTempFile::new()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                    temp_file.write_all(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                }

                let stream = ByteStream::from_path(temp_file.path())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                parsed_data.thumbnail_stream = Some(stream);
                parsed_data.temp_file = Some(temp_file);
            } else if let Ok(text) = field.text().await {
                match field_name.as_str() {
                    "name" => parsed_data.name = Some(text),
                    "url" => parsed_data.url = Some(text),
                    "event_title" => parsed_data.event_title = Some(text),
                    "short_info" => parsed_data.short_info = Some(text),
                    "description" => parsed_data.description = Some(text),
                    "is_active" => parsed_data.is_active = text.parse().ok(),
                    _ => {}
                }
            }
        }
        Ok(parsed_data)
    }

    pub async fn find(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<videos::Model>, StatusCode> {
        let video = VideoService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        Ok(Json(video))
    }

    pub async fn delete(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<&'static str>, StatusCode> {
        let video = VideoService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        VideoService::delete(&state.db, video.into_active_model())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json("Deleted"))
    }
}
