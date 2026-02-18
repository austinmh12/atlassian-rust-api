use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetAttachmentBuilder {
	client: Arc<RestClient>,
	request: GetAttachmentRequest,
}

#[derive(Debug, Clone, Default)]
struct GetAttachmentRequest {
	id: u32,
}

impl Endpoint for GetAttachmentRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("attachment/{}", self.id).into()
	}
}

impl GetAttachmentBuilder {
	fn new(client: Arc<RestClient>) -> GetAttachmentBuilder {
		GetAttachmentBuilder { client, request: GetAttachmentRequest::default() }
	}

	fn id(mut self, id: u32) -> GetAttachmentBuilder {
		self.request.id = id;
		self
	}

	async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

crate::macros::futurize!(GetAttachmentBuilder, serde_json::Value);

impl Jira {
	/// Returns the meta-data for an attachment, including the URI of the actual attached file.
	pub async fn get_attachment(&self, id: u32) -> Result<serde_json::Value> {
		GetAttachmentBuilder::new(Arc::clone(&self.client)).id(id).await
	}
}