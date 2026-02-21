use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct DeleteCommentPropertyBuilder {
	client: Arc<RestClient>,
	request: DeleteCommentPropertyRequest,
}

#[derive(Debug, Clone, Default)]
struct DeleteCommentPropertyRequest {
	comment_id: String,
	property_key: String,
}

impl Endpoint for DeleteCommentPropertyRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("comment/{}/properties/{}", &self.comment_id, &self.property_key).into()
	}
}

impl DeleteCommentPropertyBuilder {
	fn new(client: Arc<RestClient>) -> DeleteCommentPropertyBuilder {
		DeleteCommentPropertyBuilder { client, request: DeleteCommentPropertyRequest::default() }
	}

	fn comment_id(mut self, comment_id: impl Into<String>) -> DeleteCommentPropertyBuilder {
		self.request.comment_id = comment_id.into();
		self
	}

	fn property_key(mut self, property_key: impl Into<String>) -> DeleteCommentPropertyBuilder {
		self.request.property_key = property_key.into();
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Removes the property from the comment identified by the key or by the id. The user removing the property is 
	/// required to have permissions to administer the comment.
	pub fn delete_comment_property(&self, comment_id: impl Into<String>, property_key: impl Into<String>) -> DeleteCommentPropertyBuilder {
		DeleteCommentPropertyBuilder::new(Arc::clone(&self.client)).comment_id(comment_id).property_key(property_key)
	}
}