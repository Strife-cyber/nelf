use crate::state::AppState;
use axum::Extension;
use axum::http::HeaderValue;
use std::{env, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod app;
pub mod config;
pub mod middleware;
pub mod routes;
mod state;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::app::controllers::auth_controller::login,
        crate::app::controllers::video_controller::list_videos,
        crate::app::controllers::video_controller::create_video,
        crate::app::controllers::video_controller::find_video,
        crate::app::controllers::video_controller::update_video,
        crate::app::controllers::video_controller::delete_video,
        crate::app::controllers::flyer_controller::list_flyers,
        crate::app::controllers::flyer_controller::create_flyer,
        crate::app::controllers::flyer_controller::find_flyer,
        crate::app::controllers::flyer_controller::update_flyer,
        crate::app::controllers::flyer_controller::delete_flyer,
        crate::app::controllers::user_controller::list_users,
        crate::app::controllers::user_controller::create_user,
        crate::app::controllers::user_controller::find_user,
        crate::app::controllers::user_controller::update_user,
        crate::app::controllers::user_controller::delete_user,
        crate::app::controllers::website_preview_controller::list_website_previews,
        crate::app::controllers::website_preview_controller::create_website_preview,
        crate::app::controllers::website_preview_controller::find_website_preview,
        crate::app::controllers::website_preview_controller::update_website_preview,
        crate::app::controllers::website_preview_controller::delete_website_preview,
    ),
    components(
        schemas(
            crate::app::controllers::auth_controller::LoginRequest,
            crate::app::controllers::auth_controller::AuthResponse,
            crate::app::entities::videos::Model,
            crate::app::requests::store_video_request::StoreVideoRequest,
            crate::app::requests::store_video_request::UpdateVideoRequest,
            crate::app::entities::flyers::Model,
            crate::app::requests::store_flyer_request::StoreFlyerRequest,
            crate::app::requests::store_flyer_request::UpdateFlyerRequest,
            crate::app::entities::users::Model,
            crate::app::requests::store_user_request::StoreUserRequest,
            crate::app::requests::store_user_request::UpdateUserRequest,
            crate::app::entities::website_previews::Model,
            crate::app::requests::store_website_preview_request::StoreWebsitePreviewRequest,
            crate::app::requests::store_website_preview_request::UpdateWebsitePreviewRequest,
        )
    ),
    tags(
        (name = "nelf", description = "Nelf API")
    )
)]
struct ApiDoc;

fn env_flag(name: &str, default: bool) -> bool {
    env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default)
}

fn parse_allowed_origins(value: &str) -> Vec<HeaderValue> {
    let origins = value
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .unwrap_or_else(|_| panic!("Invalid CORS_ALLOWED_ORIGINS entry: {origin}"))
        })
        .collect::<Vec<_>>();

    if origins.is_empty() {
        panic!("CORS_ALLOWED_ORIGINS must contain at least one origin");
    }

    origins
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let db = config::database::connect().await;
    let client = config::filesystems::connect().await?;

    let state = Arc::new(AppState {
        db: Arc::new(db),
        s3_client: Arc::new(client),
    });

    let app_host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
    let cors_allowed_origins =
        env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "http://localhost:4321".to_string());
    let enable_swagger = env_flag("ENABLE_SWAGGER", false);

    let cors = CorsLayer::new()
        .allow_origin(parse_allowed_origins(&cors_allowed_origins))
        .allow_methods(Any)
        .allow_headers(Any);

    let app = axum::Router::new()
        .nest("/api", routes::api::api_routes())
        .layer(cors)
        .layer(axum::extract::DefaultBodyLimit::disable())
        .layer(Extension(state));

    let app = if enable_swagger {
        let swagger_router =
            SwaggerUi::new("/swagger-ui").url("/swagger-ui/openapi.json", ApiDoc::openapi());

        app.merge(swagger_router)
    } else {
        app
    };

    let bind_address = format!("{app_host}:{app_port}");
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    println!("listening on {}", listener.local_addr()?);
    if enable_swagger {
        println!("Swagger UI available at http://{bind_address}/swagger-ui/");
    } else {
        println!("Swagger UI disabled. Set ENABLE_SWAGGER=true to enable it.");
    }

    axum::serve(listener, app).await?;

    Ok(())
}
