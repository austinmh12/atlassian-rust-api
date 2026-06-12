use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct ResetFilterColumnsBuilder {
	client: Arc<RestClient>,
	request: ResetFilterColumnsRequest,
}

#[derive(Debug, Clone, Default)]
struct ResetFilterColumnsRequest {
	id: u64,
}

impl Endpoint for ResetFilterColumnsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}/columns", self.id).into()
	}
}

impl ResetFilterColumnsBuilder {
	fn new(client: Arc<RestClient>) -> ResetFilterColumnsBuilder {
		ResetFilterColumnsBuilder { client, request: ResetFilterColumnsRequest::default() }
	}

	fn id(mut self, id: u64) -> ResetFilterColumnsBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Resets the columns for the given filter such that the filter no longer has its own column config.
	pub fn reset_filter_columns(&self, id: u64) -> ResetFilterColumnsBuilder {
		ResetFilterColumnsBuilder::new(Arc::clone(&self.client)).id(id)
	}
}