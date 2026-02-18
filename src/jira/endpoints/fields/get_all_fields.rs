use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetAllFieldsBuilder {
	client: Arc<RestClient>,
	request: GetAllFieldsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetAllFieldsRequest;

impl Endpoint for GetAllFieldsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"field".into()
	}
}

impl GetAllFieldsBuilder {
	fn new(client: Arc<RestClient>) -> GetAllFieldsBuilder {
		GetAllFieldsBuilder { client, request: GetAllFieldsRequest::default() }
	}

	async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

crate::macros::futurize!(GetAllFieldsBuilder, serde_json::Value);

impl Jira {
	/// Returns a list of all fields, both system and custom
	pub async fn get_all_fields(&self) -> Result<serde_json::Value> {
		GetAllFieldsBuilder::new(Arc::clone(&self.client)).await
	}
}