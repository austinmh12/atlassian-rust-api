use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct FindGroupsBuilder {
	client: Arc<RestClient>,
	request: FindGroupsRequest,
}

#[derive(Debug, Clone, Default)]
struct FindGroupsRequest {
	query: String,
	exclude: Option<String>,
	max_results: Option<u64>,
	user_name: Option<String>,
}

impl Endpoint for FindGroupsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"groups/picker".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push("query", &self.query)
			.push_opt("exclude", self.exclude.as_ref())
			.push_opt("maxResults", self.max_results)
			.push_opt("userName", self.user_name.as_ref());
		params
	}
}

impl FindGroupsBuilder {
	fn new(client: Arc<RestClient>) -> FindGroupsBuilder {
		FindGroupsBuilder { client, request: FindGroupsRequest::default() }
	}

	fn query(mut self, query: impl Into<String>) -> FindGroupsBuilder {
		self.request.query = query.into();
		self
	}

	/// TODO
	pub fn exclude(mut self, exclude: impl Into<String>) -> FindGroupsBuilder {
		self.request.exclude = Some(exclude.into());
		self
	}

	/// Maximum number of groups to return.
	pub fn max_results(mut self, max_results: u64) -> FindGroupsBuilder {
		self.request.max_results = Some(max_results);
		self
	}

	/// TODO
	pub fn user_name(mut self, user_name: impl Into<String>) -> FindGroupsBuilder {
		self.request.user_name = Some(user_name.into());
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns groups with substrings matching a given query. This is mainly for use with the group picker, 
	/// so the returned groups contain html to be used as picker suggestions. The groups are also wrapped in a 
	/// single response object that also contains a header for use in the picker, specifically Showing X of Y matching groups.
	///
	/// The number of groups returned is limited by the system property "jira.ajax.autocomplete.limit"
	///
	/// The groups will be unique and sorted.
	pub fn find_groups(&self, query: impl Into<String>) -> FindGroupsBuilder {
		FindGroupsBuilder::new(Arc::clone(&self.client)).query(query)
	}
}