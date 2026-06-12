use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct AddFilterSharePermissionBuilder {
	client: Arc<RestClient>,
	request: AddFilterSharePermissionRequest,
}

#[derive(Debug, Clone, Default)]
struct AddFilterSharePermissionRequest {
	id: u64,
	// TODO: permissions: Vec<serde_json::Value>
}

impl Endpoint for AddFilterSharePermissionRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}/permission", self.id).into()
	}

	// TODO: Body
}

impl AddFilterSharePermissionBuilder {
	fn new(client: Arc<RestClient>) -> AddFilterSharePermissionBuilder {
		AddFilterSharePermissionBuilder { client, request: AddFilterSharePermissionRequest::default() }
	}

	fn id(mut self, id: u64) -> AddFilterSharePermissionBuilder {
		self.request.id = id;
		self
	}

	// TODO: permissions

	pub async fn send(self) -> Result<()> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Adds share permissions to the given filter. Adding a global permission removes all previous permissions from
	/// the filter.
	pub fn add_filter_share_permission(&self, id: u64) -> AddFilterSharePermissionBuilder {
		AddFilterSharePermissionBuilder::new(Arc::clone(&self.client)).id(id)
	}
}