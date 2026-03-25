use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::videos as Video;

pub struct VideoService;

#[async_trait]
impl CrudService<Video::Entity> for VideoService {}

impl VideoService {}
