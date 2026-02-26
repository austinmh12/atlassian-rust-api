use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetDashboardItemPropertyBuilder {
	client: Arc<RestClient>,
	request: GetDashboardItemPropertyRequest,
}

#[derive(Debug, Clone, Default)]
struct GetDashboardItemPropertyRequest {
	dashboard_id: String,
	item_id: String,
	property_key: String,
}

impl Endpoint for GetDashboardItemPropertyRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("dashboard/{}/items/{}/properties/{}", &self.dashboard_id, &self.item_id, &self.property_key).into()
	}
}

impl GetDashboardItemPropertyBuilder {
	fn new(client: Arc<RestClient>) -> GetDashboardItemPropertyBuilder {
		GetDashboardItemPropertyBuilder { client, request: GetDashboardItemPropertyRequest::default() }
	}

	fn dashboard_id(mut self, dashboard_id: impl Into<String>) -> GetDashboardItemPropertyBuilder {
		self.request.dashboard_id = dashboard_id.into();
		self
	}

	fn item_id(mut self, item_id: impl Into<String>) -> GetDashboardItemPropertyBuilder {
		self.request.item_id = item_id.into();
		self
	}

	fn property_key(mut self, property_key: impl Into<String>) -> GetDashboardItemPropertyBuilder {
		self.request.property_key = property_key.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the value of the property with a given key from the dashboard item identified by the key or by the id. 
	/// The user who retrieves the property is required to have permissions to read the dashboard item.
	pub fn get_dashboard_item_property(
		&self,
		dashboard_id: impl Into<String>,
		item_id: impl Into<String>,
		property_key: impl Into<String>
	) -> GetDashboardItemPropertyBuilder {
		GetDashboardItemPropertyBuilder::new(Arc::clone(&self.client))
			.dashboard_id(dashboard_id)
			.item_id(item_id)
			.property_key(property_key)
	}
}