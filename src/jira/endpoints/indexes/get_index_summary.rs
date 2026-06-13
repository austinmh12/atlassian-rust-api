use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetIndexSummaryBuilder {
	client: Arc<RestClient>,
	request: GetIndexSummaryRequest,
}

#[derive(Debug, Clone, Default)]
struct GetIndexSummaryRequest {}

impl Endpoint for GetIndexSummaryRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"index/summary".into()
	}
}

impl GetIndexSummaryBuilder {
	fn new(client: Arc<RestClient>) -> GetIndexSummaryBuilder {
		GetIndexSummaryBuilder { client, request: GetIndexSummaryRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// REST resource for index summary
	pub fn get_index_summary(&self) -> GetIndexSummaryBuilder {
		GetIndexSummaryBuilder::new(Arc::clone(&self.client))
	}
}