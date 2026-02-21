use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetAllNodesBuilder {
	client: Arc<RestClient>,
	request: GetAllNodesRequest,
}

#[derive(Debug, Clone, Default)]
struct GetAllNodesRequest;

impl Endpoint for GetAllNodesRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"cluster/nodes".into()
	}
}

impl GetAllNodesBuilder {
	fn new(client: Arc<RestClient>) -> GetAllNodesBuilder {
		GetAllNodesBuilder { client, request: GetAllNodesRequest::default() }
	}

	pub async fn send(self) -> Result<()> {
		self.client.get(self.request).await
	}
}

impl Jira {
	pub fn get_all_nodes(&self) -> GetAllNodesBuilder {
		GetAllNodesBuilder::new(Arc::clone(&self.client))
	}
}