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

impl VideoController {
    // List all videos
    pub async fn list(
        // Destructure the Extension to easily access `state`
        Extension(state): Extension<Arc<AppState>>
    ) -> Result<Json<Vec<videos::Model>>, StatusCode> {
        // Pass the db by reference using `&`
        // Map any database error to a 500 Internal Server Error
        let videos = VideoService::list_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(videos))
    }

    // Create a new video
    pub async fn create(
        Extension(state): Extension<Arc<AppState>>,
        Json(payload): Json<StoreVideoRequest>,
    ) -> Result<Json<videos::Model>, StatusCode> {
        // Convert your request payload into a SeaORM ActiveModel.
        // (Assuming you have an .into_active_model() method or From trait implemented.
        // If not, you'll map the fields manually here: videos::ActiveModel { name: Set(payload.name), ... })
        let active_model = payload.into_active_model();

        let video = VideoService::create(&state.db, active_model)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(video))
    }

    // Find by ID
    pub async fn find(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<videos::Model>, StatusCode> {
        let video = VideoService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            // If the database query succeeds but finds nothing, return a 404 Not Found
            .ok_or(StatusCode::NOT_FOUND)?;

        Ok(Json(video))
    }

    // Delete
    pub async fn delete(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<&'static str>, StatusCode> {
        // First, verify the video exists
        let video = VideoService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        // Convert the found model into an ActiveModel and delete it
        VideoService::delete(&state.db, video.into_active_model())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json("Deleted"))
    }
}