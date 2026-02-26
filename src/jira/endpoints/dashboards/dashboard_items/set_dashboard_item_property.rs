use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct SetDashboardItemPropertyBuilder {
	client: Arc<RestClient>,
	request: SetDashboardItemPropertyRequest,
}

#[derive(Debug, Clone, Default)]
struct SetDashboardItemPropertyRequest {
	dashboard_id: String,
	item_id: String,
	property_key: String,
}

impl Endpoint for SetDashboardItemPropertyRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("dashboard/{}/items/{}/properties/{}", &self.dashboard_id, &self.item_id, &self.property_key).into()
	}
}

impl SetDashboardItemPropertyBuilder {
	fn new(client: Arc<RestClient>) -> SetDashboardItemPropertyBuilder {
		SetDashboardItemPropertyBuilder { client, request: SetDashboardItemPropertyRequest::default() }
	}

	fn dashboard_id(mut self, dashboard_id: impl Into<String>) -> SetDashboardItemPropertyBuilder {
		self.request.dashboard_id = dashboard_id.into();
		self
	}

	fn item_id(mut self, item_id: impl Into<String>) -> SetDashboardItemPropertyBuilder {
		self.request.item_id = item_id.into();
		self
	}

	fn property_key(mut self, property_key: impl Into<String>) -> SetDashboardItemPropertyBuilder {
		self.request.property_key = property_key.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.put(self.request).await
	}
}

impl Jira {
	/// Sets the value of the specified dashboard item's property.
	/// 
	/// You can use this resource to store a custom data against the dashboard item identified by the key or by the id. 
	/// The user who stores the data is required to have permissions to administer the dashboard item. 
	pub fn set_dashboard_item_property(
		&self,
		dashboard_id: impl Into<String>,
		item_id: impl Into<String>,
		property_key: impl Into<String>
	) -> SetDashboardItemPropertyBuilder {
		SetDashboardItemPropertyBuilder::new(Arc::clone(&self.client))
			.dashboard_id(dashboard_id)
			.item_id(item_id)
			.property_key(property_key)
	}
}