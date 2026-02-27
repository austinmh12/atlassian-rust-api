use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct DeleteFilterBuilder {
	client: Arc<RestClient>,
	request: DeleteFilterRequest,
}

#[derive(Debug, Clone, Default)]
struct DeleteFilterRequest {
	id: u64,
}

impl Endpoint for DeleteFilterRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}", self.id).into()
	}
}

impl DeleteFilterBuilder {
	fn new(client: Arc<RestClient>) -> DeleteFilterBuilder {
		DeleteFilterBuilder { client, request: DeleteFilterRequest::default() }
	}

	fn id(mut self, id: u64) -> DeleteFilterBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Delete a filter.
	pub fn delete_filter(&self, id: u64) -> DeleteFilterBuilder {
		DeleteFilterBuilder::new(Arc::clone(&self.client)).id(id)
	}
}