use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetCustomFieldOptionsBuilder {
	client: Arc<RestClient>,
	request: GetCustomFieldOptionsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetCustomFieldOptionsRequest {
	custom_field_id: String,
	project_ids: Option<Vec<String>>,
	issue_type_ids: Option<Vec<String>>,
	query: Option<String>,
	max_results: Option<u64>,
	page: Option<u64>,

}

impl Endpoint for GetCustomFieldOptionsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("customFields/{}/options", &self.custom_field_id).into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		let project_ids = match &self.project_ids {
			Some(pids) => Some(pids.join(",")),
			None => None
		};
		let issue_type_ids = match &self.issue_type_ids {
			Some(itids) => Some(itids.join(",")),
			None => None
		};
		params
			.push_opt("projectIds", project_ids)
			.push_opt("issueTypeIds", issue_type_ids)
			.push_opt("query", self.query.as_ref())
			.push_opt("maxResults", self.max_results)
			.push_opt("page", self.page);
		params
	}
}

impl GetCustomFieldOptionsBuilder {
	fn new(client: Arc<RestClient>) -> GetCustomFieldOptionsBuilder {
		GetCustomFieldOptionsBuilder { client, request: GetCustomFieldOptionsRequest::default() }
	}

	fn custom_field_id(mut self, custom_field_id: impl Into<String>) -> GetCustomFieldOptionsBuilder {
		self.request.custom_field_id = custom_field_id.into();
		self
	}

	/// Add a project ID to filter custom fields options with.
	pub fn project_id(mut self, project_id: impl Into<String>) -> GetCustomFieldOptionsBuilder {
		match self.request.project_ids {
			Some(ref mut p) => p.push(project_id.into()),
			None => self.request.project_ids = Some(vec![project_id.into()]),
		};
		self
	}

	/// Add multiple project IDs to filter custom fields options with.
	pub fn project_ids<S>(mut self, project_ids: impl IntoIterator<Item = S>) -> GetCustomFieldOptionsBuilder 
	where
		S: Into<String>,
	{
		let project_ids = project_ids.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.project_ids {
			Some(ref mut p) => p.extend(project_ids),
			None => self.request.project_ids = Some(project_ids),
		};
		self
	}

	/// Add a issue type ID to filter custom fields options with.
	pub fn issue_type_id(mut self, issue_type_id: impl Into<String>) -> GetCustomFieldOptionsBuilder {
		match self.request.issue_type_ids {
			Some(ref mut p) => p.push(issue_type_id.into()),
			None => self.request.issue_type_ids = Some(vec![issue_type_id.into()]),
		};
		self
	}

	/// Add multiple issue type IDs to filter custom fields options with.
	pub fn issue_type_ids<S>(mut self, issue_type_ids: impl IntoIterator<Item = S>) -> GetCustomFieldOptionsBuilder 
	where
		S: Into<String>,
	{
		let issue_type_ids = issue_type_ids.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.issue_type_ids {
			Some(ref mut p) => p.extend(issue_type_ids),
			None => self.request.issue_type_ids = Some(issue_type_ids),
		};
		self
	}

	/// A string used to filter options. An option matches the query if any word in option's 
	/// name starts with the given query.
	pub fn query(mut self, query: impl Into<String>) -> GetCustomFieldOptionsBuilder {
		self.request.query = Some(query.into());
		self
	}

	/// The maximum number of results to return
	pub fn max_results(mut self, max_results: u64) -> GetCustomFieldOptionsBuilder {
		self.request.max_results = Some(max_results);
		self
	}

	/// The page of the options to return.
	pub fn page(mut self, page: u64) -> GetCustomFieldOptionsBuilder {
		self.request.page = Some(page);
		self
	}


	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns custom field's options defined in a given context composed of projects and issue types.
	/// 
	/// If the projects and issue types match more than one context or the context for such a combination 
	/// does not exist then no options are returned.
	pub fn get_custom_field_options(&self, custom_field_id: impl Into<String>) -> GetCustomFieldOptionsBuilder {
		GetCustomFieldOptionsBuilder::new(Arc::clone(&self.client)).custom_field_id(custom_field_id)
	}
}