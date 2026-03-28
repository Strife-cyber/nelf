use serde::Deserialize;
use sea_orm::{Set, NotSet, IntoActiveModel};
use utoipa::ToSchema;
use aws_sdk_s3::primitives::ByteStream;

use crate::app::entities::website_previews;

#[derive(Deserialize, ToSchema)]
pub struct StoreWebsitePreviewRequest {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub short_info: Option<String>,
    
    #[schema(value_type = Option<String>, format = Binary)]
    pub image: Option<Vec<u8>>,
    
    pub favicon_url: Option<String>,
    pub content_type: String,
    pub is_active: Option<bool>
}

pub struct ParsedWebsitePreviewData {
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub short_info: Option<String>,
    pub content_type: Option<String>,
    pub favicon_url: Option<String>,
    pub is_active: Option<bool>,
    pub image_stream: Option<ByteStream>,
    pub image_name: Option<String>,
}

impl Default for ParsedWebsitePreviewData {
    fn default() -> Self {
        Self {
            url: None,
            title: None,
            description: None,
            short_info: None,
            content_type: None,
            favicon_url: None,
            is_active: None,
            image_stream: None,
            image_name: None,
        }
    }
}

impl ParsedWebsitePreviewData {
    pub fn into_active_model(self, url: String, uploaded_image_url: Option<String>) -> website_previews::ActiveModel {
        website_previews::ActiveModel {
            url: Set(url),
            title: Set(self.title),
            description: Set(self.description),
            short_info: Set(self.short_info),
            image_url: match uploaded_image_url {
                Some(val) => Set(Some(val)),
                None => NotSet,
            },
            favicon_url: Set(self.favicon_url),
            content_type: Set(self.content_type.unwrap_or_else(|| "website".to_string())),
            is_active: match self.is_active {
                Some(active) => Set(active),
                None => NotSet,
            },
            ..Default::default()
        }
    }
}

impl IntoActiveModel<website_previews::ActiveModel> for StoreWebsitePreviewRequest {
    fn into_active_model(self) -> website_previews::ActiveModel {
        website_previews::ActiveModel {
            url: Set(self.url),
            title: Set(self.title),
            description: Set(self.description),
            short_info: Set(self.short_info),
            image_url: NotSet, // Handled separately in create_with_file
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
