use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode};
use sea_orm::IntoActiveModel;

use crate::{
    AppState,
    app::entities::users,
    app::services::crud::CrudService,
    app::services::user_service::UserService,
    app::requests::store_user_request::StoreUserRequest
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
    request_body = StoreUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = users::Model)
    )
)]
pub async fn create_user(
    state: Extension<Arc<AppState>>,
    payload: Json<StoreUserRequest>,
) -> Result<Json<users::Model>, StatusCode> {
    UserController::create(state, payload).await
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
        Json(payload): Json<StoreUserRequest>,
    ) -> Result<Json<users::Model>, StatusCode> {
        let active_model = payload.into_active_model();

        let user = UserService::create(&state.db, active_model)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
