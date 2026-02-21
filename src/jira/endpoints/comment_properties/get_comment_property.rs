use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetCommentPropertyBuilder {
	client: Arc<RestClient>,
	request: GetCommentPropertyRequest,
}

#[derive(Debug, Clone, Default)]
struct GetCommentPropertyRequest {
	comment_id: String,
	property_key: String,
}

impl Endpoint for GetCommentPropertyRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("comment/{}/properties/{}", &self.comment_id, &self.property_key).into()
	}
}

impl GetCommentPropertyBuilder {
	fn new(client: Arc<RestClient>) -> GetCommentPropertyBuilder {
		GetCommentPropertyBuilder { client, request: GetCommentPropertyRequest::default() }
	}

	fn comment_id(mut self, comment_id: impl Into<String>) -> GetCommentPropertyBuilder {
		self.request.comment_id = comment_id.into();
		self
	}

	fn property_key(mut self, property_key: impl Into<String>) -> GetCommentPropertyBuilder {
		self.request.property_key = property_key.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the value of the property with a given key from the comment identified by the key or by the id. 
	/// The user who retrieves the property is required to have permissions to read the comment.
	pub fn get_comment_property(&self, comment_id: impl Into<String>, property_key: impl Into<String>) -> GetCommentPropertyBuilder {
		GetCommentPropertyBuilder::new(Arc::clone(&self.client)).comment_id(comment_id).property_key(property_key)
	}
}