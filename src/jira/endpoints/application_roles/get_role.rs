use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetRoleBuilder {
	client: Arc<RestClient>,
	request: GetRoleRequest,
}

#[derive(Debug, Clone, Default)]
struct GetRoleRequest {
	key: String,
}

impl Endpoint for GetRoleRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("applicationrole/{}", &self.key).into()
	}
}

impl GetRoleBuilder {
	fn new(client: Arc<RestClient>) -> GetRoleBuilder {
		GetRoleBuilder { client, request: GetRoleRequest::default() }
	}

	fn key(mut self, key: impl Into<String>) -> GetRoleBuilder {
		self.request.key = key.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the ApplicationRole with the given key if it exists. Returns a 404 if the ApplicationRole
	/// is not found.
	pub fn get_role(&self, key: impl Into<String>) -> GetRoleBuilder {
		GetRoleBuilder::new(Arc::clone(&self.client)).key(key)
	}
}