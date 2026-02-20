use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct ExpandAttachmentForHumansBuilder {
	client: Arc<RestClient>,
	request: ExpandAttachmentForHumansRequest,
}

#[derive(Debug, Clone, Default)]
struct ExpandAttachmentForHumansRequest {
	id: u32,
}

impl Endpoint for ExpandAttachmentForHumansRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("attachment/{}/expand/human", self.id).into()
	}
}

impl ExpandAttachmentForHumansBuilder {
	fn new(client: Arc<RestClient>) -> ExpandAttachmentForHumansBuilder {
		ExpandAttachmentForHumansBuilder { client, request: ExpandAttachmentForHumansRequest::default() }
	}

	fn id(mut self, id: u32) -> ExpandAttachmentForHumansBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Tries to expand an attachment. Output is human-readable and subject to change.
	pub async fn expand_attachment_for_humans(&self, id: u32) -> ExpandAttachmentForHumansBuilder {
		ExpandAttachmentForHumansBuilder::new(Arc::clone(&self.client)).id(id)
	}
}