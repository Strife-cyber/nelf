use serde::Deserialize;
use sea_orm::{Set, NotSet, IntoActiveModel};
use utoipa::ToSchema;

use crate::app::entities::users;

#[derive(Deserialize, ToSchema)]
pub struct StoreUserRequest {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub initials: Option<String>,
    pub skills: Option<Vec<String>>,
    pub is_active: Option<bool>
}

impl IntoActiveModel<users::ActiveModel> for StoreUserRequest {
    fn into_active_model(self) -> users::ActiveModel {
        users::ActiveModel {
            name: Set(self.name),
            email: Set(self.email),
            phone: Set(self.phone),
            role: Set(self.role),
            description: Set(self.description),
            avatar_url: Set(self.avatar_url),
            initials: Set(self.initials),
            skills: Set(self.skills),
            is_active: match self.is_active {
                Some(active) => Set(active),
                None => NotSet,
            },
            ..Default::default()
        }
    }
}
