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

    // 4. Role-Based Access Control + Ownership Check
    let path = req.uri().path();
    let method = req.method();

    // Special case: Users can update or delete their own profile
    if path.starts_with("/api/users/") && (method == Method::PUT || method == Method::DELETE) {
        if let Some(id_str) = path.strip_prefix("/api/users/") {
            if let Ok(target_id) = id_str.parse::<i32>() {
                if claims.sub == target_id || claims.role == "admin" {
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    // For everything else mutating, must be admin
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // 5. If valid, proceed
    Ok(next.run(req).await)
}
