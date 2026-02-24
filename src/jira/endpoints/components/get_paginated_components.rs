use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetComponentsBuilder {
	client: Arc<RestClient>,
	request: GetComponentsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetComponentsRequest {
	start_at: Option<u64>,
	max_results: Option<u64>,
	query: Option<String>,
	project_ids: Option<Vec<String>>,
}

impl Endpoint for GetComponentsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"component/page".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		let project_ids = match &self.project_ids {
			Some(pids) => Some(pids.join(",")),
			None => None
		};
		params
			.push_opt("startAt", self.start_at)
			.push_opt("maxResults", self.max_results)
			.push_opt("query", self.query.as_ref())
			.push_opt("projectIds", project_ids);
		params
	}
}

impl GetComponentsBuilder {
	fn new(client: Arc<RestClient>) -> GetComponentsBuilder {
		GetComponentsBuilder { client, request: GetComponentsRequest::default() }
	}

	/// The index of the first components to return.
	pub fn start_at(mut self, start_at: u64) -> GetComponentsBuilder {
		self.request.start_at = Some(start_at);
		self
	}

	/// The maximum number of components to return.
	pub fn max_results(mut self, max_results: u64) -> GetComponentsBuilder {
		self.request.max_results = Some(max_results);
		self
	}

	/// A string that componentns names will be matched with.
	pub fn query(mut self, query: impl Into<String>) -> GetComponentsBuilder {
		self.request.query = Some(query.into());
		self
	}

	/// Add a project ID to filter components with.
	pub fn project_id(mut self, project_id: impl Into<String>) -> GetComponentsBuilder {
		match self.request.project_ids {
			Some(ref mut p) => p.push(project_id.into()),
			None => self.request.project_ids = Some(vec![project_id.into()]),
		};
		self
	}

	/// Add multiple project IDs to filter components with.
	pub fn project_ids<S>(mut self, project_ids: impl IntoIterator<Item = S>) -> GetComponentsBuilder 
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

	pub async fn send(self) -> Result<()> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns paginated list of filtered active components.
	pub fn get_paginated_components(&self) -> GetComponentsBuilder {
		GetComponentsBuilder::new(Arc::clone(&self.client))
	}

	/// Alias for [`Jira::get_paginated_components`].
	pub fn get_components(&self) -> GetComponentsBuilder {
		self.get_paginated_components()
	}
}