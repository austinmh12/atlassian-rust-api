use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams}};

#[derive(Debug, Clone)]
pub struct SetDefaultShareScopeBuilder {
	client: Arc<RestClient>,
	request: SetDefaultShareScopeRequest,
}

#[derive(Debug, Clone, Default)]
struct SetDefaultShareScopeRequest {
	scope: String,
}

impl Endpoint for SetDefaultShareScopeRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"filter/defaultShareScope".into()
	}

	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let mut body = JsonFormParams::default();
		body.push("scope", &self.scope);

		body.into_body()
	}
}

impl SetDefaultShareScopeBuilder {
	fn new(client: Arc<RestClient>) -> SetDefaultShareScopeBuilder {
		SetDefaultShareScopeBuilder { client, request: SetDefaultShareScopeRequest::default() }
	}

	fn scope(mut self, scope: impl Into<String>) -> SetDefaultShareScopeBuilder {
		self.request.scope = scope.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.post(self.request).await
	}
}

impl Jira {
	/// Sets the default share scope of the logged-in user.
	///
	/// Available values are: `AUTHENTICATED` (for sharing with all logged-in users) and `PRIVATE` (for no shares).
	pub fn set_default_share_scope(&self, scope: impl Into<String>) -> SetDefaultShareScopeBuilder {
		SetDefaultShareScopeBuilder::new(Arc::clone(&self.client)).scope(scope)
	}
}