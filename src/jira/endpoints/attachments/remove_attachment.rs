use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct RemoveAttachmentBuilder {
	client: Arc<RestClient>,
	request: RemoveAttachmentRequest,
}

#[derive(Debug, Clone, Default)]
struct RemoveAttachmentRequest {
	id: u32
}

impl Endpoint for RemoveAttachmentRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("attachment/{}", self.id).into()
	}
}

impl RemoveAttachmentBuilder {
	fn new(client: Arc<RestClient>) -> RemoveAttachmentBuilder {
		RemoveAttachmentBuilder { client, request: RemoveAttachmentRequest::default() }
	}

	fn id(mut self, id: u32) -> RemoveAttachmentBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Remove an attachment from an issue.
	pub async fn remove_attachment(&self, id: u32) -> RemoveAttachmentBuilder {
		RemoveAttachmentBuilder::new(Arc::clone(&self.client)).id(id)
	}
}