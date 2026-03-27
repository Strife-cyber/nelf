use serde::Deserialize;
use sea_orm::{Set, NotSet, IntoActiveModel};
use utoipa::ToSchema;

use crate::app::entities::website_previews;

#[derive(Deserialize, ToSchema)]
pub struct StoreWebsitePreviewRequest {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub short_info: Option<String>,
    pub image_url: Option<String>,
    pub favicon_url: Option<String>,
    pub content_type: String,
    pub is_active: Option<bool>
}

impl IntoActiveModel<website_previews::ActiveModel> for StoreWebsitePreviewRequest {
    fn into_active_model(self) -> website_previews::ActiveModel {
        website_previews::ActiveModel {
            url: Set(self.url),
            title: Set(self.title),
            description: Set(self.description),
            short_info: Set(self.short_info),
            image_url: Set(self.image_url),
            favicon_url: Set(self.favicon_url),
            content_type: Set(self.content_type),
            is_active: match self.is_active {
                Some(active) => Set(active),
                None => NotSet,
            },
            ..Default::default()
        }
    }
}
