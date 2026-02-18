use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetRolesBuilder {
	client: Arc<RestClient>,
	request: GetRolesRequest,
}

#[derive(Debug, Clone, Default)]
struct GetRolesRequest;

impl Endpoint for GetRolesRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"applicationrole".into()
	}
}

impl GetRolesBuilder {
	fn new(client: Arc<RestClient>) -> GetRolesBuilder {
		GetRolesBuilder { client, request: GetRolesRequest::default() }
	}

	async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

crate::macros::futurize!(GetRolesBuilder, serde_json::Value);

impl Jira {
	/// Returns all ApplicationRoles in the system.
	pub async fn get_roles(&self) -> Result<serde_json::Value> {
		GetRolesBuilder::new(Arc::clone(&self.client)).await
	}
}