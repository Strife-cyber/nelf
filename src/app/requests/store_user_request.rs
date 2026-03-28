use serde::Deserialize;
use sea_orm::{Set, NotSet, IntoActiveModel};
use utoipa::ToSchema;
use aws_sdk_s3::primitives::ByteStream;

use crate::app::entities::users;

#[derive(Deserialize, ToSchema)]
pub struct StoreUserRequest {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub description: Option<String>,
    
    #[schema(value_type = Option<String>, format = Binary)]
    pub avatar: Option<Vec<u8>>,
    
    pub initials: Option<String>,
    pub skills: Option<Vec<String>>,
    pub is_active: Option<bool>
}

pub struct ParsedUserData {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub description: Option<String>,
    pub initials: Option<String>,
    pub skills: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub avatar_stream: Option<ByteStream>,
    pub avatar_name: Option<String>,
}

impl Default for ParsedUserData {
    fn default() -> Self {
        Self {
            name: None,
            email: None,
            phone: None,
            role: None,
            description: None,
            initials: None,
            skills: None,
            is_active: None,
            avatar_stream: None,
            avatar_name: None,
        }
    }
}

impl ParsedUserData {
    pub fn into_active_model(self, name: String, email: String, uploaded_avatar_url: Option<String>) -> users::ActiveModel {
        users::ActiveModel {
            name: Set(name),
            email: Set(email),
            phone: Set(self.phone),
            role: Set(self.role),
            description: Set(self.description),
            avatar_url: match uploaded_avatar_url {
                Some(val) => Set(Some(val)),
                None => NotSet,
            },
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

impl IntoActiveModel<users::ActiveModel> for StoreUserRequest {
    fn into_active_model(self) -> users::ActiveModel {
        users::ActiveModel {
            name: Set(self.name),
            email: Set(self.email),
            phone: Set(self.phone),
            role: Set(self.role),
            description: Set(self.description),
            avatar_url: Set(None), // Handled separately
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
