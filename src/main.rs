use std::sync::Arc;
use axum::Extension;
use crate::state::AppState;

pub mod config;
pub mod routes;
pub mod app;
mod state;

#[tokio::main]
async fn main() {
    let db = config::database::connect().await;

    let state = Arc::new(AppState {
        db: Arc::new(db)
    });

    let app = axum::Router::new()
        .nest("/api", routes::api::api_routes())
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
