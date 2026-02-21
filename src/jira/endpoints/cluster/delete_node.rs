use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct DeleteNodeBuilder {
	client: Arc<RestClient>,
	request: DeleteNodeRequest,
}

#[derive(Debug, Clone, Default)]
struct DeleteNodeRequest {
	node_id: String,
}

impl Endpoint for DeleteNodeRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("cluster/node/{}", &self.node_id).into()
	}
}

impl DeleteNodeBuilder {
	fn new(client: Arc<RestClient>) -> DeleteNodeBuilder {
		DeleteNodeBuilder { client, request: DeleteNodeRequest::default() }
	}

	fn node_id(mut self, node_id: impl Into<String>) -> DeleteNodeBuilder {
		self.request.node_id = node_id.into();
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Delete the node from the cluster if the state of the node is OFFLINE.
	pub fn delete_node(&self, node_id: impl Into<String>) -> DeleteNodeBuilder {
		DeleteNodeBuilder::new(Arc::clone(&self.client)).node_id(node_id)
	}
}