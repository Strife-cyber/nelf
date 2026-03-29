use utoipa::ToSchema;
use sea_orm::{Set, NotSet};
use aws_sdk_s3::primitives::ByteStream;
use serde::Deserialize;

use crate::app::entities::flyers;

#[derive(ToSchema, Deserialize)]
pub struct StoreFlyerRequest {
    pub name: String,

    #[schema(value_type = String, format = Binary)]
    pub file: Vec<u8>,

    pub event_title: Option<String>,
    pub short_info: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>
}

#[derive(ToSchema, Deserialize)]
pub struct UpdateFlyerRequest {
    pub name: Option<String>,

    #[schema(value_type = Option<String>, format = Binary)]
    pub file: Option<Vec<u8>>,

    pub event_title: Option<Option<String>>,
    pub short_info: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>
}

pub struct ParsedFlyerData {
    pub name: Option<String>,
    pub event_title: Option<String>,
    pub short_info: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub file_stream: Option<ByteStream>,
    pub file_name: Option<String>,
    pub temp_file: Option<tempfile::NamedTempFile>,
}

impl Default for ParsedFlyerData {
    fn default() -> Self {
        Self {
            name: None,
            event_title: None,
            short_info: None,
            description: None,
            is_active: None,
            file_stream: None,
            file_name: None,
            temp_file: None,
        }
    }
}

impl ParsedFlyerData {
    pub fn into_active_model(self, name: String, uploaded_url: String) -> flyers::ActiveModel {
        flyers::ActiveModel {
            name: Set(name),
            url: Set(uploaded_url),
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
            is_active: match self.is_active {
                Some(active) => Set(active),
                None => NotSet,
            },
            ..Default::default()

        }
    }

    pub fn update_active_model(self, mut active_model: flyers::ActiveModel, uploaded_url: Option<String>) -> flyers::ActiveModel {
        if let Some(val) = self.name { active_model.name = Set(val); }
        if let Some(val) = self.event_title { active_model.event_title = Set(Some(val)); }
        if let Some(val) = self.short_info { active_model.short_info = Set(Some(val)); }
        if let Some(val) = self.description { active_model.description = Set(Some(val)); }
        if let Some(val) = self.is_active { active_model.is_active = Set(val); }
        
        if let Some(val) = uploaded_url {
            active_model.url = Set(val);
        }
        
        active_model
    }
}
