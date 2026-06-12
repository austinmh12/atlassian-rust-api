use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetFilterSharePermissionBuilder {
	client: Arc<RestClient>,
	request: GetFilterSharePermissionRequest,
}

#[derive(Debug, Clone, Default)]
struct GetFilterSharePermissionRequest {
	id: u64,
	permission_id: u64,
}

impl Endpoint for GetFilterSharePermissionRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}/permission/{}", self.id, self.permission_id).into()
	}
}

impl GetFilterSharePermissionBuilder {
	fn new(client: Arc<RestClient>) -> GetFilterSharePermissionBuilder {
		GetFilterSharePermissionBuilder { client, request: GetFilterSharePermissionRequest::default() }
	}

	fn id(mut self, id: u64) -> GetFilterSharePermissionBuilder {
		self.request.id = id;
		self
	}

	fn permission_id(mut self, permission_id: u64) -> GetFilterSharePermissionBuilder {
		self.request.permission_id = permission_id;
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns a single share permission of the given filter
	pub fn get_filter_share_permission(&self, id: u64, permission_id: u64) -> GetFilterSharePermissionBuilder {
		GetFilterSharePermissionBuilder::new(Arc::clone(&self.client)).id(id).permission_id(permission_id)
	}
}