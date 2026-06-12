use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetFilterSharePermissionsBuilder {
	client: Arc<RestClient>,
	request: GetFilterSharePermissionsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetFilterSharePermissionsRequest {
	id: u64,
}

impl Endpoint for GetFilterSharePermissionsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}/permission", self.id).into()
	}
}

impl GetFilterSharePermissionsBuilder {
	fn new(client: Arc<RestClient>) -> GetFilterSharePermissionsBuilder {
		GetFilterSharePermissionsBuilder { client, request: GetFilterSharePermissionsRequest::default() }
	}

	fn id(mut self, id: u64) -> GetFilterSharePermissionsBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns all share permissions of the given filter.
	pub fn get_filter_share_permissions(&self, id: u64) -> GetFilterSharePermissionsBuilder {
		GetFilterSharePermissionsBuilder::new(Arc::clone(&self.client)).id(id)
	}
}