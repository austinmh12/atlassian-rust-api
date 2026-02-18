use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetAttachmentMetaBuilder {
	client: Arc<RestClient>,
	request: GetAttachmentMetaRequest,
}

#[derive(Debug, Clone, Default)]
struct GetAttachmentMetaRequest;

impl Endpoint for GetAttachmentMetaRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"attachment/meta".into()
	}
}

impl GetAttachmentMetaBuilder {
	fn new(client: Arc<RestClient>) -> GetAttachmentMetaBuilder {
		GetAttachmentMetaBuilder { client, request: GetAttachmentMetaRequest::default() }
	}

	async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

crate::macros::futurize!(GetAttachmentMetaBuilder, serde_json::Value);

impl Jira {
	/// Returns the meta information for attachments, specifically if they are enabled and the maximum upload size allowed.
	pub async fn get_attachment_meta(&self) -> Result<serde_json::Value> {
		GetAttachmentMetaBuilder::new(Arc::clone(&self.client)).await
	}
}