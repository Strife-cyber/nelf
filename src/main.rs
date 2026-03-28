use std::sync::Arc;
use axum::Extension;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::state::AppState;

pub mod config;
pub mod routes;
pub mod app;
mod state;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::app::controllers::video_controller::list_videos,
        crate::app::controllers::video_controller::create_video,
        crate::app::controllers::video_controller::find_video,
        crate::app::controllers::video_controller::delete_video,
        crate::app::controllers::flyer_controller::list_flyers,
        crate::app::controllers::flyer_controller::create_flyer,
        crate::app::controllers::flyer_controller::find_flyer,
        crate::app::controllers::flyer_controller::delete_flyer,
        crate::app::controllers::user_controller::list_users,
        crate::app::controllers::user_controller::create_user,
        crate::app::controllers::user_controller::find_user,
        crate::app::controllers::user_controller::delete_user,
        crate::app::controllers::website_preview_controller::list_website_previews,
        crate::app::controllers::website_preview_controller::create_website_preview,
        crate::app::controllers::website_preview_controller::find_website_preview,
        crate::app::controllers::website_preview_controller::delete_website_preview,
    ),
    components(
        schemas(
            crate::app::entities::videos::Model,
            crate::app::requests::store_video_request::StoreVideoRequest,
            crate::app::entities::flyers::Model,
            crate::app::requests::store_flyer_request::StoreFlyerRequest,
            crate::app::entities::users::Model,
            crate::app::requests::store_user_request::StoreUserRequest,
            crate::app::entities::website_previews::Model,
            crate::app::requests::store_website_preview_request::StoreWebsitePreviewRequest,
        )
    ),
    tags(
        (name = "nelf", description = "Nelf API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let db = config::database::connect().await;
    let client = config::filesystems::connect().await?;

    let state = Arc::new(AppState {
        db: Arc::new(db),
        s3_client: Arc::new(client)
    });

    // Swagger UI at /swagger-ui/
    let swagger_router = SwaggerUi::new("/swagger-ui")
        .url("/swagger-ui/openapi.json", ApiDoc::openapi());

    let app = axum::Router::new()
        .nest("/api", routes::api::api_routes())
        .merge(swagger_router)
        .layer(axum::extract::DefaultBodyLimit::disable())
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on {}", listener.local_addr()?);
    println!("Swagger UI available at http://127.0.0.1:3000/swagger-ui/");

    axum::serve(listener, app).await?;

    Ok(())
}
