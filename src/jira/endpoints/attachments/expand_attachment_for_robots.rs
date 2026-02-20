use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct ExpandAttachmentForRobotsBuilder {
	client: Arc<RestClient>,
	request: ExpandAttachmentForRobotsRequest,
}

#[derive(Debug, Clone, Default)]
struct ExpandAttachmentForRobotsRequest {
	id: u32,
}

impl Endpoint for ExpandAttachmentForRobotsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("attachment/{}/expand/raw", self.id).into()
	}
}

impl ExpandAttachmentForRobotsBuilder {
	fn new(client: Arc<RestClient>) -> ExpandAttachmentForRobotsBuilder {
		ExpandAttachmentForRobotsBuilder { client, request: ExpandAttachmentForRobotsRequest::default() }
	}

	fn id(mut self, id: u32) -> ExpandAttachmentForRobotsBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Tries to expand an attachment. Output is raw and should be backwards-compatible through the course of time.
	pub fn expand_attachment_for_robots(&self, id: u32) -> ExpandAttachmentForRobotsBuilder {
		ExpandAttachmentForRobotsBuilder::new(Arc::clone(&self.client)).id(id)
	}
}