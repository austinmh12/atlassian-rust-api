use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetFilterDefaultColumnsBuilder {
	client: Arc<RestClient>,
	request: GetFilterDefaultColumnsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetFilterDefaultColumnsRequest {
	id: u64
}

impl Endpoint for GetFilterDefaultColumnsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}/columns", self.id).into()
	}
}

impl GetFilterDefaultColumnsBuilder {
	fn new(client: Arc<RestClient>) -> GetFilterDefaultColumnsBuilder {
		GetFilterDefaultColumnsBuilder { client, request: GetFilterDefaultColumnsRequest::default() }
	}

	fn id(mut self, id: u64) -> GetFilterDefaultColumnsBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the default columns for the given filter. Currently logged in user will be used as the user making
	/// such request
	pub fn get_filter_default_columns(&self, id: u64) -> GetFilterDefaultColumnsBuilder {
		GetFilterDefaultColumnsBuilder::new(Arc::clone(&self.client)).id(id)
	}
}