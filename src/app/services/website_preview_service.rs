use async_trait::async_trait;
use super::crud::CrudService;

use super::super::entities::website_previews as WebsitePreview;

pub struct WebsitePreviewService;

#[async_trait]
impl CrudService<WebsitePreview::Entity> for WebsitePreviewService {}

impl WebsitePreviewService {}
