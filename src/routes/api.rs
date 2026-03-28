use axum::Router;
use axum::routing::{delete, get, post};
use crate::app::controllers::video_controller;
use crate::app::controllers::flyer_controller;
use crate::app::controllers::user_controller;
use crate::app::controllers::website_preview_controller;
use crate::app::controllers::auth_controller;
use crate::middleware::auth::auth_middleware;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/auth/login", post(auth_controller::login))
}

pub fn video_routes() -> Router {
    Router::new()
        .route("/videos", get(video_controller::list_videos))
        .route("/videos", post(video_controller::create_video))
        .route("/videos/{id}", get(video_controller::find_video))
        .route("/videos/{id}", delete(video_controller::delete_video))
}

pub fn flyer_routes() -> Router {
    Router::new()
        .route("/flyers", get(flyer_controller::list_flyers))
        .route("/flyers", post(flyer_controller::create_flyer))
        .route("/flyers/{id}", get(flyer_controller::find_flyer))
        .route("/flyers/{id}", delete(flyer_controller::delete_flyer))
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(user_controller::list_users))
        .route("/users", post(user_controller::create_user))
        .route("/users/{id}", get(user_controller::find_user))
        .route("/users/{id}", delete(user_controller::delete_user))
}

pub fn website_preview_routes() -> Router {
    Router::new()
        .route("/website-previews", get(website_preview_controller::list_website_previews))
        .route("/website-previews", post(website_preview_controller::create_website_preview))
        .route("/website-previews/{id}", get(website_preview_controller::find_website_preview))
        .route("/website-previews/{id}", delete(website_preview_controller::delete_website_preview))
}

pub fn api_routes() -> Router {
    Router::new()
        .merge(auth_routes())
        .merge(
            Router::new()
                .merge(video_routes())
                .merge(flyer_routes())
                .merge(user_routes())
                .merge(website_preview_routes())
                .layer(axum::middleware::from_fn(auth_middleware))
        )
}
