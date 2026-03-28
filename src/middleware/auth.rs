use axum::{
    body::Body,
    http::{Request, StatusCode, Method},
    middleware::Next,
    response::Response,
};
use crate::app::services::auth_service::AuthService;

pub async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // 1. Allow all GET requests
    if req.method() == Method::GET {
        return Ok(next.run(req).await);
    }

    // 2. Extract Authorization header
    let auth_header = req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            &header[7..]
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // 3. Validate JWT
    let claims = AuthService::validate_jwt(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 4. Role-Based Access Control (Optional: ensure they are admin for mutating operations)
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // 5. If valid, proceed
    Ok(next.run(req).await)
}
