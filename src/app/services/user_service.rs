use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::users as User;

pub struct UserService;

#[async_trait]
impl CrudService<User::Entity> for UserService {}

impl UserService {}
