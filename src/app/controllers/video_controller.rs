use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode};
use sea_orm::IntoActiveModel;

use crate::{
    AppState,
    app::entities::videos,
    app::services::crud::CrudService,
    app::services::video_service::VideoService,
    app::requests::store_video_request::StoreVideoRequest
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
    request_body = StoreVideoRequest,
    responses(
        (status = 201, description = "Video created successfully", body = videos::Model)
    )
)]
pub async fn create_video(
    state: Extension<Arc<AppState>>,
    payload: Json<StoreVideoRequest>,
) -> Result<Json<videos::Model>, StatusCode> {
    VideoController::create(state, payload).await
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
        Json(payload): Json<StoreVideoRequest>,
    ) -> Result<Json<videos::Model>, StatusCode> {
        let active_model = payload.into_active_model();

        let video = VideoService::create(&state.db, active_model)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(video))
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
