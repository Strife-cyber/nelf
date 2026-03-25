use serde::Deserialize;
use sea_orm::{Set, NotSet, IntoActiveModel};

// Make sure to adjust this import to point to your actual videos entity
use crate::app::entities::videos;

#[derive(Deserialize)]
pub struct StoreVideoRequest {
    pub name: String,
    pub url: String,
    pub event_title: Option<String>,
    pub short_info: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub is_active: Option<bool>
}

// This tells Rust exactly how to turn the JSON request into a Database Insert
impl IntoActiveModel<videos::ActiveModel> for StoreVideoRequest {
    fn into_active_model(self) -> videos::ActiveModel {
        videos::ActiveModel {
            name: Set(self.name),
            url: Set(self.url),
            event_title: Set(self.event_title),
            short_info: Set(self.short_info),
            description: Set(self.description),
            thumbnail_url: Set(self.thumbnail_url),

            // If the user provided true/false, Set it.
            // If it's missing (None), use NotSet so Postgres uses `DEFAULT true`.
            is_active: match self.is_active {
                Some(active) => Set(active),
                None => NotSet,
            },

            // Default::default() handles `id`, `created_at`, and `updated_at`
            // so SeaORM knows not to try and insert them manually.
            ..Default::default()
        }
    }
}