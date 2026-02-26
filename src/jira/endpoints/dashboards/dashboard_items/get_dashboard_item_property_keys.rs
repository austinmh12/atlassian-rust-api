use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetDashboardItemPropertyKeysBuilder {
	client: Arc<RestClient>,
	request: GetDashboardItemPropertyKeysRequest,
}

#[derive(Debug, Clone, Default)]
struct GetDashboardItemPropertyKeysRequest {
	dashboard_id: String,
	item_id: String,
}

impl Endpoint for GetDashboardItemPropertyKeysRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("dashboard/{}/items/{}/properties", &self.dashboard_id, &self.item_id).into()
	}
}

impl GetDashboardItemPropertyKeysBuilder {
	fn new(client: Arc<RestClient>) -> GetDashboardItemPropertyKeysBuilder {
		GetDashboardItemPropertyKeysBuilder { client, request: GetDashboardItemPropertyKeysRequest::default() }
	}

	fn dashboard_id(mut self, dashboard_id: impl Into<String>) -> GetDashboardItemPropertyKeysBuilder {
		self.request.dashboard_id = dashboard_id.into();
		self
	}

	fn item_id(mut self, item_id: impl Into<String>) -> GetDashboardItemPropertyKeysBuilder {
		self.request.item_id = item_id.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the keys of all properties for the dashboard item identified by the key 
	/// or by the id.
	pub fn get_dashboard_item_property_keys(
		&self,
		dashboard_id: impl Into<String>,
		item_id: impl Into<String>,
	) -> GetDashboardItemPropertyKeysBuilder {
		GetDashboardItemPropertyKeysBuilder::new(Arc::clone(&self.client))
			.dashboard_id(dashboard_id)
			.item_id(item_id)
	}
}