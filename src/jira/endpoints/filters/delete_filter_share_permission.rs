use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct DeleteFilterSharePermissionBuilder {
	client: Arc<RestClient>,
	request: DeleteFilterSharePermissionRequest,
}

#[derive(Debug, Clone, Default)]
struct DeleteFilterSharePermissionRequest {
	id: u64,
	permission_id: u64,
}

impl Endpoint for DeleteFilterSharePermissionRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}/permission/{}", self.id, self.permission_id).into()
	}
}

impl DeleteFilterSharePermissionBuilder {
	fn new(client: Arc<RestClient>) -> DeleteFilterSharePermissionBuilder {
		DeleteFilterSharePermissionBuilder { client, request: DeleteFilterSharePermissionRequest::default() }
	}

	fn id(mut self, id: u64) -> DeleteFilterSharePermissionBuilder {
		self.request.id = id;
		self
	}

	fn permission_id(mut self, permission_id: u64) -> DeleteFilterSharePermissionBuilder {
		self.request.permission_id = permission_id;
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Removes a share permission from the given filter.
	pub fn delete_filter_share_permission(&self, id: u64, permission_id: u64) -> DeleteFilterSharePermissionBuilder {
		DeleteFilterSharePermissionBuilder::new(Arc::clone(&self.client)).id(id).permission_id(permission_id)
	}
}