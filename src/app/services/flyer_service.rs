use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::flyers as Flyer;

pub struct FlyerService;

#[async_trait]
impl CrudService<Flyer::Entity> for FlyerService {}

impl FlyerService {}
