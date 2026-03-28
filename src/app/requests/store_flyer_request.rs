use utoipa::ToSchema;
use sea_orm::{Set, NotSet, IntoActiveModel};

use crate::app::entities::flyers;

#[derive(ToSchema)]
pub struct StoreFlyerRequest {
    pub name: String,

    #[schema(value_type = String, format = Binary)]
    pub file: Vec<u8>,

    pub event_title: Option<String>,
    pub short_info: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>
}

#[derive(Default)]
pub struct ParsedFlyerData {
    pub name: Option<String>,
    pub event_title: Option<String>,
    pub short_info: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub file_bytes: Option<Vec<u8>>,
    pub file_name: Option<String>,
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
}

/*impl IntoActiveModel<flyers::ActiveModel> for StoreFlyerRequest {
    fn into_active_model(self) -> flyers::ActiveModel {
        flyers::ActiveModel {
            name: Set(self.name),
            url: Set(self.url),
            event_title: Set(self.event_title),
            short_info: Set(self.short_info),
            description: Set(self.description),
            is_active: match self.is_active {
                Some(active) => Set(active),
                None => NotSet,
            },
            ..Default::default()
        }
    }
}*/
