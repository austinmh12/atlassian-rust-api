use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetStateBuilder {
	client: Arc<RestClient>,
	request: GetStateRequest,
}

#[derive(Debug, Clone, Default)]
struct GetStateRequest;

impl Endpoint for GetStateRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"cluster/zdu/state".into()
	}
}

impl GetStateBuilder {
	fn new(client: Arc<RestClient>) -> GetStateBuilder {
		GetStateBuilder { client, request: GetStateRequest::default() }
	}

	pub async fn send(self) -> Result<()> {
		self.client.post_ignore(self.request).await
	}
}

impl Jira {
	pub fn get_state(&self) -> GetStateBuilder {
		GetStateBuilder::new(Arc::clone(&self.client))
	}
}