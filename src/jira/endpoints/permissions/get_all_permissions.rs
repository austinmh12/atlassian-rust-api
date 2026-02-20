use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetAllPermissionsBuilder {
	client: Arc<RestClient>,
	request: GetAllPermissionsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetAllPermissionsRequest;

impl Endpoint for GetAllPermissionsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"permissions".into()
	}
}

impl GetAllPermissionsBuilder {
	fn new(client: Arc<RestClient>) -> GetAllPermissionsBuilder {
		GetAllPermissionsBuilder { client, request: GetAllPermissionsRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns all permissions that are present in the Jira instance
	pub fn get_all_permissions(&self) -> GetAllPermissionsBuilder {
		GetAllPermissionsBuilder::new(Arc::clone(&self.client))
	}
}