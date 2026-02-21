use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct SetCommentPropertyBuilder {
	client: Arc<RestClient>,
	request: SetCommentPropertyRequest,
}

#[derive(Debug, Clone, Default)]
struct SetCommentPropertyRequest {
	comment_id: String,
	property_key: String,
}

impl Endpoint for SetCommentPropertyRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("comment/{}/properties/{}", &self.comment_id, &self.property_key).into()
	}
}

impl SetCommentPropertyBuilder {
	fn new(client: Arc<RestClient>) -> SetCommentPropertyBuilder {
		SetCommentPropertyBuilder { client, request: SetCommentPropertyRequest::default() }
	}

	fn comment_id(mut self, comment_id: impl Into<String>) -> SetCommentPropertyBuilder {
		self.request.comment_id = comment_id.into();
		self
	}

	fn property_key(mut self, property_key: impl Into<String>) -> SetCommentPropertyBuilder {
		self.request.property_key = property_key.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.put(self.request).await
	}
}

impl Jira {
	/// Sets the value of the specified comment's property.
	/// 
	/// You can use this resource to store a custom data against the comment identified by the key or by the id. 
	/// The user who stores the data is required to have permissions to administer the comment. 
	pub fn set_comment_property(&self, comment_id: impl Into<String>, property_key: impl Into<String>) -> SetCommentPropertyBuilder {
		SetCommentPropertyBuilder::new(Arc::clone(&self.client)).comment_id(comment_id).property_key(property_key)
	}
}