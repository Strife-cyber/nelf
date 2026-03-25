use serde::Deserialize;
use sea_orm::{Set, NotSet, IntoActiveModel};

use crate::app::entities::flyers;

#[derive(Deserialize)]
pub struct StoreFlyerRequest {
    pub name: String,
    pub url: String,
    pub event_title: Option<String>,
    pub short_info: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>
}

impl IntoActiveModel<flyers::ActiveModel> for StoreFlyerRequest {
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
}
