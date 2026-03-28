use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    AppState,
    app::services::user_service::UserService,
    app::services::auth_service::AuthService
};

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}

pub struct AuthController;

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    state: Extension<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // 1. Find user by email
    let user_opt = UserService::find_by_email(&state.db, &payload.email)
        .await
        .map_err(|e| {
            println!("DB Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let user = user_opt.ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. Verify password
    if !AuthService::verify_password(&payload.password, &user.password_hash) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 3. Generate JWT
    let token = AuthService::generate_jwt(user.id, user.role.unwrap_or_else(|| "visitor".to_string()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}
