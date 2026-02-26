use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetCustomFieldOptionBuilder {
	client: Arc<RestClient>,
	request: GetCustomFieldOptionRequest,
}

#[derive(Debug, Clone, Default)]
struct GetCustomFieldOptionRequest {
	id: u64,
}

impl Endpoint for GetCustomFieldOptionRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("customFieldOption/{}", self.id).into()
	}
}

impl GetCustomFieldOptionBuilder {
	fn new(client: Arc<RestClient>) -> GetCustomFieldOptionBuilder {
		GetCustomFieldOptionBuilder { client, request: GetCustomFieldOptionRequest::default() }
	}

	fn id(mut self, id: u64) -> GetCustomFieldOptionBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns a full representation of the Custom Field Option that has the given id.
	pub fn get_custom_field_option(&self, id: u64) -> GetCustomFieldOptionBuilder {
		GetCustomFieldOptionBuilder::new(Arc::clone(&self.client)).id(id)
	}
}