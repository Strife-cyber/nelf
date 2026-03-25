use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode};
use sea_orm::IntoActiveModel;

use crate::{
    AppState,
    app::entities::website_previews,
    app::services::crud::CrudService,
    app::services::website_preview_service::WebsitePreviewService,
    app::requests::store_website_preview_request::StoreWebsitePreviewRequest
};

pub struct WebsitePreviewController;

impl WebsitePreviewController {
    pub async fn list(
        Extension(state): Extension<Arc<AppState>>
    ) -> Result<Json<Vec<website_previews::Model>>, StatusCode> {
        let website_previews = WebsitePreviewService::list_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(website_previews))
    }

    pub async fn create(
        Extension(state): Extension<Arc<AppState>>,
        Json(payload): Json<StoreWebsitePreviewRequest>,
    ) -> Result<Json<website_previews::Model>, StatusCode> {
        let active_model = payload.into_active_model();

        let website_preview = WebsitePreviewService::create(&state.db, active_model)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(website_preview))
    }

    pub async fn find(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<website_previews::Model>, StatusCode> {
        let website_preview = WebsitePreviewService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        Ok(Json(website_preview))
    }

    pub async fn delete(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<&'static str>, StatusCode> {
        let website_preview = WebsitePreviewService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        WebsitePreviewService::delete(&state.db, website_preview.into_active_model())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json("Deleted"))
    }
}
