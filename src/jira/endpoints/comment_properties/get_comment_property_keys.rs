use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetCommentPropertyKeysBuilder {
	client: Arc<RestClient>,
	request: GetCommentPropertyKeysRequest,
}

#[derive(Debug, Clone, Default)]
struct GetCommentPropertyKeysRequest {
	comment_id: String,
}

impl Endpoint for GetCommentPropertyKeysRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("comment/{}/properties", &self.comment_id).into()
	}
}

impl GetCommentPropertyKeysBuilder {
	fn new(client: Arc<RestClient>) -> GetCommentPropertyKeysBuilder {
		GetCommentPropertyKeysBuilder { client, request: GetCommentPropertyKeysRequest::default() }
	}

	fn comment_id(mut self, comment_id: impl Into<String>) -> GetCommentPropertyKeysBuilder {
		self.request.comment_id = comment_id.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the keys of all properties for the comment identified by the key or by the id.
	pub fn get_comment_property_keys(&self, comment_id: impl Into<String>) -> GetCommentPropertyKeysBuilder {
		GetCommentPropertyKeysBuilder::new(Arc::clone(&self.client)).comment_id(comment_id)
	}
}