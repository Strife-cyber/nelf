use serde::Deserialize;
use sea_orm::{Set, NotSet, IntoActiveModel};
use utoipa::ToSchema;
use aws_sdk_s3::primitives::ByteStream;

use crate::app::entities::videos;

#[derive(Deserialize, ToSchema)]
pub struct StoreVideoRequest {
    pub name: String,
    pub url: String,
    pub event_title: Option<String>,
    pub short_info: Option<String>,
    pub description: Option<String>,
    
    #[schema(value_type = Option<String>, format = Binary)]
    pub thumbnail: Option<Vec<u8>>,
    
    pub is_active: Option<bool>
}

pub struct ParsedVideoData {
    pub name: Option<String>,
    pub url: Option<String>,
    pub event_title: Option<String>,
    pub short_info: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub thumbnail_stream: Option<ByteStream>,
    pub thumbnail_name: Option<String>,
}

impl Default for ParsedVideoData {
    fn default() -> Self {
        Self {
            name: None,
            url: None,
            event_title: None,
            short_info: None,
            description: None,
            is_active: None,
            thumbnail_stream: None,
            thumbnail_name: None,
        }
    }
}

impl ParsedVideoData {
    pub fn into_active_model(self, name: String, url: String, uploaded_thumbnail_url: Option<String>) -> videos::ActiveModel {
        videos::ActiveModel {
            name: Set(name),
            url: Set(url),
            event_title: match self.event_title {
                Some(val) => Set(Some(val)),
                None => NotSet,
            },
            short_info: match self.short_info {
                Some(val) => Set(Some(val)),
                None => NotSet,
            },
            description: match self.description {
                Some(val) => Set(Some(val)),
                None => NotSet,
            },
            thumbnail_url: match uploaded_thumbnail_url {
                Some(val) => Set(Some(val)),
                None => NotSet,
            },
            is_active: match self.is_active {
                Some(active) => Set(active),
                None => NotSet,
            },
            ..Default::default()
        }
    }
}

impl IntoActiveModel<videos::ActiveModel> for StoreVideoRequest {
    fn into_active_model(self) -> videos::ActiveModel {
        videos::ActiveModel {
            name: Set(self.name),
            url: Set(self.url),
            event_title: Set(self.event_title),
            short_info: Set(self.short_info),
            description: Set(self.description),
            thumbnail_url: Set(None), // Handled separately
            is_active: match self.is_active {
                Some(active) => Set(active),
                None => NotSet,
            },
            ..Default::default()
        }
    }
}