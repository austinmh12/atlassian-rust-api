use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct ListDashboardsBuilder {
	client: Arc<RestClient>,
	request: ListDashboardsRequest,
}

#[derive(Debug, Clone, Default)]
struct ListDashboardsRequest {
	filter: Option<String>,
	start_at: Option<u64>,
	max_results: Option<u64>,
}

impl Endpoint for ListDashboardsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"dashboard".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push_opt("filter", self.filter.as_ref())
			.push_opt("startAt", self.start_at)
			.push_opt("maxResults", self.max_results);
		params
	}
}

impl ListDashboardsBuilder {
	fn new(client: Arc<RestClient>) -> ListDashboardsBuilder {
		ListDashboardsBuilder { client, request: ListDashboardsRequest::default() }
	}

	/// An optional filter that is applied to the list of dashboards. Valid values include 
	/// "favourite" for returning only favourite dashboards, and "my" for returning dashboards 
	/// that are owned by the calling user.
	pub fn filter(mut self, filter: impl Into<String>) -> ListDashboardsBuilder {
		self.request.filter = Some(filter.into());
		self
	}

	/// The index of the first dashboard to return (0-based). must be 0 or a multiple of maxResults.
	pub fn start_at(mut self, start_at: u64) -> ListDashboardsBuilder {
		self.request.start_at = Some(start_at);
		self
	}

	/// A hint as to the maximum number of dashboards to return in each call. 
	/// 
	/// Note that the Jira server reserves the right to impose a maxResults limit that is 
	/// lower than the value that a client provides, dues to lack of resources or any other 
	/// condition. When this happens, your results will be truncated. Callers should always 
	/// check the returned maxResults to determine the value that is effectively being used.
	pub fn max_results(mut self, max_results: u64) -> ListDashboardsBuilder {
		self.request.max_results = Some(max_results);
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns a list of all dashboards, optionally filtering them.
	pub fn list_dashboards(&self) -> ListDashboardsBuilder {
		ListDashboardsBuilder::new(Arc::clone(&self.client))
	}
}