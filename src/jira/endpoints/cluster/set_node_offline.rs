use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct SetNodeOfflineBuilder {
	client: Arc<RestClient>,
	request: SetNodeOfflineRequest,
}

#[derive(Debug, Clone, Default)]
struct SetNodeOfflineRequest {
	node_id: String,
}

impl Endpoint for SetNodeOfflineRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("cluster/node/{}/offline", &self.node_id).into()
	}
}

impl SetNodeOfflineBuilder {
	fn new(client: Arc<RestClient>) -> SetNodeOfflineBuilder {
		SetNodeOfflineBuilder { client, request: SetNodeOfflineRequest::default() }
	}

	fn node_id(mut self, node_id: impl Into<String>) -> SetNodeOfflineBuilder {
		self.request.node_id = node_id.into();
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.put_ignore(self.request).await
	}
}

impl Jira {
	/// Change the node's state to offline if the node is reporting as active, but is not alive. 
	/// 
	/// Don't use this method as an equivalent of running ./stop-jira.sh. This method doesn't shut down 
	/// a node, but only changes its state, so that other nodes don't communicate with it.
	pub fn set_node_offline(&self, node_id: impl Into<String>) -> SetNodeOfflineBuilder {
		SetNodeOfflineBuilder::new(Arc::clone(&self.client)).node_id(node_id)
	}
}