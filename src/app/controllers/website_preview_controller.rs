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

#[utoipa::path(
    get,
    path = "/api/website-previews",
    responses(
        (status = 200, description = "List all website previews", body = [website_previews::Model])
    )
)]
pub async fn list_website_previews(
    state: Extension<Arc<AppState>>
) -> Result<Json<Vec<website_previews::Model>>, StatusCode> {
    WebsitePreviewController::list(state).await
}

#[utoipa::path(
    post,
    path = "/api/website-previews",
    request_body = StoreWebsitePreviewRequest,
    responses(
        (status = 201, description = "Website preview created successfully", body = website_previews::Model)
    )
)]
pub async fn create_website_preview(
    state: Extension<Arc<AppState>>,
    payload: Json<StoreWebsitePreviewRequest>,
) -> Result<Json<website_previews::Model>, StatusCode> {
    WebsitePreviewController::create(state, payload).await
}

#[utoipa::path(
    get,
    path = "/api/website-previews/{id}",
    params(
        ("id" = i32, Path, description = "Website Preview ID")
    ),
    responses(
        (status = 200, description = "Website preview found", body = website_previews::Model),
        (status = 404, description = "Website preview not found")
    )
)]
pub async fn find_website_preview(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
) -> Result<Json<website_previews::Model>, StatusCode> {
    WebsitePreviewController::find(state, id).await
}

#[utoipa::path(
    delete,
    path = "/api/website-previews/{id}",
    params(
        ("id" = i32, Path, description = "Website Preview ID")
    ),
    responses(
        (status = 200, description = "Website preview deleted successfully"),
        (status = 404, description = "Website preview not found")
    )
)]
pub async fn delete_website_preview(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
) -> Result<Json<&'static str>, StatusCode> {
    WebsitePreviewController::delete(state, id).await
}

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
