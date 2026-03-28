use serde::Deserialize;
use sea_orm::{Set, NotSet, IntoActiveModel};
use utoipa::ToSchema;
use aws_sdk_s3::primitives::ByteStream;

use crate::app::entities::users;
use crate::app::services::auth_service::AuthService;

#[derive(Deserialize, ToSchema)]
pub struct StoreUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub description: Option<String>,
    
    #[schema(value_type = Option<String>, format = Binary)]
    pub avatar: Option<Vec<u8>>,
    
    pub initials: Option<String>,
    pub skills: Option<Vec<String>>,
    pub is_active: Option<bool>
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone: Option<Option<String>>,
    pub role: Option<Option<String>>,
    pub description: Option<Option<String>>,
    
    #[schema(value_type = Option<String>, format = Binary)]
    pub avatar: Option<Vec<u8>>,
    
    pub initials: Option<Option<String>>,
    pub skills: Option<Option<Vec<String>>>,
    pub is_active: Option<bool>
}

pub struct ParsedUserData {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
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
            password: None,
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
        let password_hash = if let Some(pwd) = self.password {
            AuthService::hash_password(&pwd).unwrap_or_default()
        } else {
            "".to_string()
        };

        users::ActiveModel {
            name: Set(name),
            email: Set(email),
            password_hash: Set(password_hash),
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

    pub fn update_active_model(self, mut active_model: users::ActiveModel, uploaded_avatar_url: Option<String>) -> users::ActiveModel {
        if let Some(val) = self.name { active_model.name = Set(val); }
        if let Some(val) = self.email { active_model.email = Set(val); }
        if let Some(val) = self.password {
            if let Ok(hash) = AuthService::hash_password(&val) {
                active_model.password_hash = Set(hash);
            }
        }
        if let Some(val) = self.phone { active_model.phone = Set(Some(val)); }
        if let Some(val) = self.role { active_model.role = Set(Some(val)); }
        if let Some(val) = self.description { active_model.description = Set(Some(val)); }
        if let Some(val) = self.initials { active_model.initials = Set(Some(val)); }
        if let Some(val) = self.skills { active_model.skills = Set(Some(val)); }
        if let Some(val) = self.is_active { active_model.is_active = Set(val); }
        
        if let Some(val) = uploaded_avatar_url {
            active_model.avatar_url = Set(Some(val));
        }
        
        active_model
    }
}

impl IntoActiveModel<users::ActiveModel> for StoreUserRequest {
    fn into_active_model(self) -> users::ActiveModel {
        let password_hash = AuthService::hash_password(&self.password).unwrap_or_default();
        
        users::ActiveModel {
            name: Set(self.name),
            email: Set(self.email),
            password_hash: Set(password_hash),
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
