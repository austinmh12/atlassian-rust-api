use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetAdvancedSettingsBuilder {
	client: Arc<RestClient>,
	request: GetAdvancedSettingsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetAdvancedSettingsRequest;

impl Endpoint for GetAdvancedSettingsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"application-properties/advanced-settings".into()
	}
}

impl GetAdvancedSettingsBuilder {
	fn new(client: Arc<RestClient>) -> GetAdvancedSettingsBuilder {
		GetAdvancedSettingsBuilder { client, request: GetAdvancedSettingsRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the properties that are displayed on the "General Configuration > Advanced Settings" page.
	pub async fn get_advanced_settings(&self) -> GetAdvancedSettingsBuilder {
		GetAdvancedSettingsBuilder::new(Arc::clone(&self.client))
	}
}