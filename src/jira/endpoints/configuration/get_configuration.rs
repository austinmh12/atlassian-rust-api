use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetConfigurationBuilder {
	client: Arc<RestClient>,
	request: GetConfigurationRequest,
}

#[derive(Debug, Clone, Default)]
struct GetConfigurationRequest;

impl Endpoint for GetConfigurationRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"configuration".into()
	}
}

impl GetConfigurationBuilder {
	fn new(client: Arc<RestClient>) -> GetConfigurationBuilder {
		GetConfigurationBuilder { client, request: GetConfigurationRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the information if the optional features in Jira are enabled or disabled. If the time 
	/// tracking is enabled, it also returns the detailed information about time tracking configuration.
	pub fn get_configuration(&self) -> GetConfigurationBuilder {
		GetConfigurationBuilder::new(Arc::clone(&self.client))
	}
}