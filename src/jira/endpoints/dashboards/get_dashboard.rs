use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetDashboardBuilder {
	client: Arc<RestClient>,
	request: GetDashboardRequest,
}

#[derive(Debug, Clone, Default)]
struct GetDashboardRequest {
	id: u64,
}

impl Endpoint for GetDashboardRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("dashboard/{}", self.id).into()
	}
}

impl GetDashboardBuilder {
	fn new(client: Arc<RestClient>) -> GetDashboardBuilder {
		GetDashboardBuilder { client, request: GetDashboardRequest::default() }
	}

	fn id(mut self, id: u64) -> GetDashboardBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns a single dashboard.
	pub fn get_dashboard(&self, id: u64) -> GetDashboardBuilder {
		GetDashboardBuilder::new(Arc::clone(&self.client)).id(id)
	}
}