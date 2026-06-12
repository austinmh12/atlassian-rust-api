use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetDefaultShareScopeBuilder {
	client: Arc<RestClient>,
	request: GetDefaultShareScopeRequest,
}

#[derive(Debug, Clone, Default)]
struct GetDefaultShareScopeRequest {}

impl Endpoint for GetDefaultShareScopeRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"filter/defaultShareScope".into()
	}
}

impl GetDefaultShareScopeBuilder {
	fn new(client: Arc<RestClient>) -> GetDefaultShareScopeBuilder {
		GetDefaultShareScopeBuilder { client, request: GetDefaultShareScopeRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the default share scope of the logged in user.
	pub fn get_default_share_scope(&self) -> GetDefaultShareScopeBuilder {
		GetDefaultShareScopeBuilder::new(Arc::clone(&self.client))
	}
}