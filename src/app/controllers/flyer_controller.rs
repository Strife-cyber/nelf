use std::sync::Arc;
use axum::extract::Multipart;
use sea_orm::IntoActiveModel;
use axum::{Extension, Json, http::StatusCode};

use crate::{
    AppState,
    app::entities::flyers,
    app::services::crud::CrudService,
    app::services::flyer_service::FlyerService,
    app::requests::store_flyer_request::StoreFlyerRequest
};
use crate::app::requests::store_flyer_request::ParsedFlyerData;

pub struct FlyerController;

#[utoipa::path(
    get,
    path = "/api/flyers",
    responses(
        (status = 200, description = "List all flyers", body = [flyers::Model])
    )
)]
pub async fn list_flyers(
    state: Extension<Arc<AppState>>
) -> Result<Json<Vec<flyers::Model>>, StatusCode> {
    FlyerController::list(state).await
}

#[utoipa::path(
    post,
    path = "/api/flyers",
    request_body(content = StoreFlyerRequest, content_type="multipart/form-data"),
    responses(
        (status = 201, description = "Flyer created successfully", body = flyers::Model)
    )
)]
pub async fn create_flyer(
    state: Extension<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<flyers::Model>, StatusCode> {
    FlyerController::create(state, multipart).await
}

#[utoipa::path(
    get,
    path = "/api/flyers/{id}",
    params(
        ("id" = i32, Path, description = "Flyer ID")
    ),
    responses(
        (status = 200, description = "Flyer found", body = flyers::Model),
        (status = 404, description = "Flyer not found")
    )
)]
pub async fn find_flyer(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
) -> Result<Json<flyers::Model>, StatusCode> {
    FlyerController::find(state, id).await
}

#[utoipa::path(
    delete,
    path = "/api/flyers/{id}",
    params(
        ("id" = i32, Path, description = "Flyer ID")
    ),
    responses(
        (status = 200, description = "Flyer deleted successfully"),
        (status = 404, description = "Flyer not found")
    )
)]
pub async fn delete_flyer(
    state: Extension<Arc<AppState>>,
    id: axum::extract::Path<i32>,
) -> Result<Json<&'static str>, StatusCode> {
    FlyerController::delete(state, id).await
}

impl FlyerController {
    pub async fn list(
        Extension(state): Extension<Arc<AppState>>
    ) -> Result<Json<Vec<flyers::Model>>, StatusCode> {
        let flyers = FlyerService::list_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(flyers))
    }

    pub async fn create(
        Extension(state): Extension<Arc<AppState>>,
        mut multipart: Multipart
    ) -> Result<Json<flyers::Model>, StatusCode> {
        let mut parsed_data = ParsedFlyerData::default();

        while let Ok(Some(field)) = multipart.next_field().await {
            let field_name = field.name().unwrap_or("").to_string();

            if field_name == "file" {
                parsed_data.file_name = field.file_name().map(|s| s.to_string());
                if let Ok(bytes) = field.bytes().await {
                    parsed_data.file_bytes = Some(bytes.to_vec());
                }
            } else if let Ok(text) = field.text().await {
                match field_name.as_str() {
                    "name" => parsed_data.name = Some(text),
                    "event_title" => parsed_data.event_title = Some(text),
                    "short_info" => parsed_data.short_info = Some(text),
                    "description" => parsed_data.description = Some(text),
                    "is_active" => parsed_data.is_active = text.parse().ok(),
                    _ => {}
                }
            }
        }

        let flyer = FlyerService::create_with_file(&state.db, &state.s3_client, parsed_data)
            .await
            .map_err(|e| {
                println!("Error creating flyer: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(Json(flyer))
    }

    pub async fn find(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<flyers::Model>, StatusCode> {
        let flyer = FlyerService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        Ok(Json(flyer))
    }

    pub async fn delete(
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path(id): axum::extract::Path<i32>,
    ) -> Result<Json<&'static str>, StatusCode> {
        let flyer = FlyerService::find_by_id(&state.db, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        FlyerService::delete(&state.db, flyer.into_active_model())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json("Deleted"))
    }
}
