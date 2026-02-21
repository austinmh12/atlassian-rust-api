use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetCurrentIndexBuilder {
	client: Arc<RestClient>,
	request: GetCurrentIndexRequest,
}

#[derive(Debug, Clone, Default)]
struct GetCurrentIndexRequest {
	node_id: String,
}

impl Endpoint for GetCurrentIndexRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("cluster/index-snapshot/{}", &self.node_id).into()
	}
}

impl GetCurrentIndexBuilder {
	fn new(client: Arc<RestClient>) -> GetCurrentIndexBuilder {
		GetCurrentIndexBuilder { client, request: GetCurrentIndexRequest::default() }
	}

	fn node_id(mut self, node_id: impl Into<String>) -> GetCurrentIndexBuilder {
		self.request.node_id = node_id.into();
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.put(self.request).await
	}
}

impl Jira {
	/// Request current index from node (the request is processed asynchronously).
	pub fn get_current_index(&self, node_id: impl Into<String>) -> GetCurrentIndexBuilder {
		GetCurrentIndexBuilder::new(Arc::clone(&self.client)).node_id(node_id)
	}
}