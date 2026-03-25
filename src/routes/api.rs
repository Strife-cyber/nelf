use axum::Router;
use axum::routing::{delete, get, post};
use crate::app::controllers::video_controller::VideoController;
use crate::app::controllers::flyer_controller::FlyerController;
use crate::app::controllers::user_controller::UserController;
use crate::app::controllers::website_preview_controller::WebsitePreviewController;

pub fn video_routes() -> Router {
    Router::new()
        .route("/videos", get(VideoController::list))
        .route("/videos", post(VideoController::create))
        .route("/videos/{id}", get(VideoController::find))
        .route("/videos/{id}", delete(VideoController::delete))
}

pub fn flyer_routes() -> Router {
    Router::new()
        .route("/flyers", get(FlyerController::list))
        .route("/flyers", post(FlyerController::create))
        .route("/flyers/{id}", get(FlyerController::find))
        .route("/flyers/{id}", delete(FlyerController::delete))
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(UserController::list))
        .route("/users", post(UserController::create))
        .route("/users/{id}", get(UserController::find))
        .route("/users/{id}", delete(UserController::delete))
}

pub fn website_preview_routes() -> Router {
    Router::new()
        .route("/website-previews", get(WebsitePreviewController::list))
        .route("/website-previews", post(WebsitePreviewController::create))
        .route("/website-previews/{id}", get(WebsitePreviewController::find))
        .route("/website-previews/{id}", delete(WebsitePreviewController::delete))
}

pub fn api_routes() -> Router {
    Router::new()
        .merge(video_routes())
        .merge(flyer_routes())
        .merge(user_routes())
        .merge(website_preview_routes())
}
