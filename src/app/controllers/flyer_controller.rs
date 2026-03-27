use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode};
use sea_orm::IntoActiveModel;

use crate::{
    AppState,
    app::entities::flyers,
    app::services::crud::CrudService,
    app::services::flyer_service::FlyerService,
    app::requests::store_flyer_request::StoreFlyerRequest
};

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
    request_body = StoreFlyerRequest,
    responses(
        (status = 201, description = "Flyer created successfully", body = flyers::Model)
    )
)]
pub async fn create_flyer(
    state: Extension<Arc<AppState>>,
    payload: Json<StoreFlyerRequest>,
) -> Result<Json<flyers::Model>, StatusCode> {
    FlyerController::create(state, payload).await
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
        Json(payload): Json<StoreFlyerRequest>,
    ) -> Result<Json<flyers::Model>, StatusCode> {
        let active_model = payload.into_active_model();

        let flyer = FlyerService::create(&state.db, active_model)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
