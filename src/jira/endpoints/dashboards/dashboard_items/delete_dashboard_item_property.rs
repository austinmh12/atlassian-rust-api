use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct DeleteDashboardItemPropertyBuilder {
	client: Arc<RestClient>,
	request: DeleteDashboardItemPropertyRequest,
}

#[derive(Debug, Clone, Default)]
struct DeleteDashboardItemPropertyRequest {
	dashboard_item: String,
	item_id: String,
	property_key: String,
}

impl Endpoint for DeleteDashboardItemPropertyRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("dashboard/{}/item/{}/properties/{}", &self.dashboard_item, &self.item_id, &self.property_key).into()
	}
}

impl DeleteDashboardItemPropertyBuilder {
	fn new(client: Arc<RestClient>) -> DeleteDashboardItemPropertyBuilder {
		DeleteDashboardItemPropertyBuilder { client, request: DeleteDashboardItemPropertyRequest::default() }
	}

	fn dashboard_item(mut self, dashboard_item: impl Into<String>) -> DeleteDashboardItemPropertyBuilder {
		self.request.dashboard_item = dashboard_item.into();
		self
	}

	fn item_id(mut self, item_id: impl Into<String>) -> DeleteDashboardItemPropertyBuilder {
		self.request.item_id = item_id.into();
		self
	}

	fn property_key(mut self, property_key: impl Into<String>) -> DeleteDashboardItemPropertyBuilder {
		self.request.property_key = property_key.into();
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Removes the property from the dashboard item identified by the key or by the id. The user removing the property is 
	/// required to have permissions to administer the dashboard item.
	pub fn delete_dashboard_item_property(
		&self,
		dashboard_item: impl Into<String>,
		item_id: impl Into<String>,
		property_key: impl Into<String>
	) -> DeleteDashboardItemPropertyBuilder {
		DeleteDashboardItemPropertyBuilder::new(Arc::clone(&self.client))
			.dashboard_item(dashboard_item)
			.item_id(item_id)
			.property_key(property_key)
	}
}