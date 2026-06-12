use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams}};

#[derive(Debug, Clone)]
pub struct CreateGroupBuilder {
	client: Arc<RestClient>,
	request: CreateGroupRequest,
}

#[derive(Debug, Clone, Default)]
struct CreateGroupRequest {
	name: String,
}

impl Endpoint for CreateGroupRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"group".into()
	}

	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let mut body = JsonFormParams::default();
		body.push("name", &self.name);

		body.into_body()
	}
}

impl CreateGroupBuilder {
	fn new(client: Arc<RestClient>) -> CreateGroupBuilder {
		CreateGroupBuilder { client, request: CreateGroupRequest::default() }
	}

	fn name(mut self, name: impl Into<String>) -> CreateGroupBuilder {
		self.request.name = name.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.post(self.request).await
	}
}

impl Jira {
	/// Creates a group by given parameter.
	pub fn create_group(&self, name: impl Into<String>) -> CreateGroupBuilder {
		CreateGroupBuilder::new(Arc::clone(&self.client)).name(name)
	}
}