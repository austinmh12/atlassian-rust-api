use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetCustomFieldsBuilder {
	client: Arc<RestClient>,
	request: GetCustomFieldsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetCustomFieldsRequest {
	start_at: Option<u64>,
	max_results: Option<u64>,
	search: Option<String>,
	project_ids: Option<Vec<String>>,
	screen_ids: Option<Vec<String>>,
	types: Option<Vec<String>>,
	sort_order: Option<String>,
	sort_column: Option<String>,
	last_value_update: Option<u64>,
}

impl Endpoint for GetCustomFieldsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"customFields".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		let project_ids = match &self.project_ids {
			Some(pids) => Some(pids.join(",")),
			None => None
		};
		let screen_ids = match &self.screen_ids {
			Some(sids) => Some(sids.join(",")),
			None => None
		};
		let types = match &self.types {
			Some(typs) => Some(typs.join(",")),
			None => None
		};
		params
			.push_opt("startAt", self.start_at)
			.push_opt("maxResults", self.max_results)
			.push_opt("search", self.search.as_ref())
			.push_opt("projectIds", project_ids)
			.push_opt("screenIds", screen_ids)
			.push_opt("types", types)
			.push_opt("sortOrder", self.sort_order.as_ref())
			.push_opt("sortColumn", self.sort_column.as_ref())
			.push_opt("lastValueUpdate", self.last_value_update);
		params
	}
}

impl GetCustomFieldsBuilder {
	fn new(client: Arc<RestClient>) -> GetCustomFieldsBuilder {
		GetCustomFieldsBuilder { client, request: GetCustomFieldsRequest::default() }
	}

	/// Index of the first custom field to return.
	pub fn start_at(mut self, start_at: u64) -> GetCustomFieldsBuilder {
		self.request.start_at = Some(start_at);
		self
	}

	/// Maximum number of fields to return.
	pub fn max_results(mut self, max_results: u64) -> GetCustomFieldsBuilder {
		self.request.max_results = Some(max_results);
		self
	}

	/// A string that custom field names will be matched with.
	pub fn search(mut self, search: impl Into<String>) -> GetCustomFieldsBuilder {
		self.request.search = Some(search.into());
		self
	}

	/// Add a project ID to filter custom fields with.
	pub fn project_id(mut self, project_id: impl Into<String>) -> GetCustomFieldsBuilder {
		match self.request.project_ids {
			Some(ref mut p) => p.push(project_id.into()),
			None => self.request.project_ids = Some(vec![project_id.into()]),
		};
		self
	}

	/// Add multiple project IDs to filter custom fields with.
	pub fn project_ids<S>(mut self, project_ids: impl IntoIterator<Item = S>) -> GetCustomFieldsBuilder 
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

	/// Add a screen ID to filter custom fields with.
	pub fn screen_id(mut self, screen_id: impl Into<String>) -> GetCustomFieldsBuilder {
		match self.request.screen_ids {
			Some(ref mut p) => p.push(screen_id.into()),
			None => self.request.screen_ids = Some(vec![screen_id.into()]),
		};
		self
	}

	/// Add multiple screen IDs to filter custom fields with.
	pub fn screen_ids<S>(mut self, screen_ids: impl IntoIterator<Item = S>) -> GetCustomFieldsBuilder 
	where
		S: Into<String>,
	{
		let screen_ids = screen_ids.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.screen_ids {
			Some(ref mut p) => p.extend(screen_ids),
			None => self.request.screen_ids = Some(screen_ids),
		};
		self
	}

	/// Add a field type to filter custom fields with.
	pub fn field_type(mut self, field_type: impl Into<String>) -> GetCustomFieldsBuilder {
		match self.request.types {
			Some(ref mut p) => p.push(field_type.into()),
			None => self.request.types = Some(vec![field_type.into()]),
		};
		self
	}

	/// Add multiple field types to filter custom fields with.
	pub fn field_types<S>(mut self, field_types: impl IntoIterator<Item = S>) -> GetCustomFieldsBuilder 
	where
		S: Into<String>,
	{
		let field_types = field_types.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.types {
			Some(ref mut p) => p.extend(field_types),
			None => self.request.types = Some(field_types),
		};
		self
	}

	/// The order to sort the custom fields with.
	pub fn sort_order(mut self, sort_order: impl Into<String>) -> GetCustomFieldsBuilder {
		self.request.sort_order = Some(sort_order.into());
		self
	}

	/// The column to sort the custom fields against.
	pub fn sort_column(mut self, sort_column: impl Into<String>) -> GetCustomFieldsBuilder {
		self.request.sort_column = Some(sort_column.into());
		self
	}

	/// The last updated timestamp to filter custom fields with.
	pub fn last_value_update(mut self, last_value_update: u64) -> GetCustomFieldsBuilder {
		self.request.last_value_update = Some(last_value_update);
		self
	}


	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Get custom fields.
	pub fn get_custom_fields(&self) -> GetCustomFieldsBuilder {
		GetCustomFieldsBuilder::new(Arc::clone(&self.client))
	}
}