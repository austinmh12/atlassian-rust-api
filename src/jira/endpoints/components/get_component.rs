use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetComponentBuilder {
	client: Arc<RestClient>,
	request: GetComponentRequest,
}

#[derive(Debug, Clone, Default)]
struct GetComponentRequest {
	id: u64
}

impl Endpoint for GetComponentRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("component/{}", self.id).into()
	}
}

impl GetComponentBuilder {
	fn new(client: Arc<RestClient>) -> GetComponentBuilder {
		GetComponentBuilder { client, request: GetComponentRequest::default() }
	}

	fn id(mut self, id: u64) -> GetComponentBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns a project component.
	pub fn get_component(&self, id: u64) -> GetComponentBuilder {
		GetComponentBuilder::new(Arc::clone(&self.client)).id(id)
	}
}