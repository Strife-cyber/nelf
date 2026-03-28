use std::sync::Arc;
use axum::extract::Multipart;
use sea_orm::IntoActiveModel;
use axum::{Extension, Json, http::StatusCode};
use aws_sdk_s3::primitives::ByteStream;
use futures_util::StreamExt;
use std::io::Write;

use crate::{
    AppState,
    app::entities::website_previews,
    app::services::crud::CrudService,
    app::services::website_preview_service::WebsitePreviewService,
    app::requests::store_website_preview_request::{StoreWebsitePreviewRequest, ParsedWebsitePreviewData}
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
    request_body(content = StoreWebsitePreviewRequest, content_type="multipart/form-data"),
    responses(
        (status = 201, description = "Website preview created successfully", body = website_previews::Model)
    )
)]
pub async fn create_website_preview(
    state: Extension<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<website_previews::Model>, StatusCode> {
    WebsitePreviewController::create(state, multipart).await
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
        mut multipart: Multipart
    ) -> Result<Json<website_previews::Model>, StatusCode> {
        let mut parsed_data = ParsedWebsitePreviewData::default();

        while let Ok(Some(mut field)) = multipart.next_field().await {
            let field_name = field.name().unwrap_or("").to_string();

            if field_name == "image" {
                parsed_data.image_name = field.file_name().map(|s| s.to_string());
                
                let mut temp_file = tempfile::NamedTempFile::new()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                    temp_file.write_all(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                }

                let stream = ByteStream::from_path(temp_file.path())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                parsed_data.image_stream = Some(stream);
            } else if let Ok(text) = field.text().await {
                match field_name.as_str() {
                    "url" => parsed_data.url = Some(text),
                    "title" => parsed_data.title = Some(text),
                    "description" => parsed_data.description = Some(text),
                    "short_info" => parsed_data.short_info = Some(text),
                    "favicon_url" => parsed_data.favicon_url = Some(text),
                    "content_type" => parsed_data.content_type = Some(text),
                    "is_active" => parsed_data.is_active = text.parse().ok(),
                    _ => {}
                }
            }
        }

        let website_preview = WebsitePreviewService::create_with_file(&state.db, &state.s3_client, parsed_data)
            .await
            .map_err(|e| {
                println!("Error creating website preview: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

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
