use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams, QueryParams}};

#[derive(Debug, Clone)]
pub struct CreateFilterBuilder {
	client: Arc<RestClient>,
	request: CreateFilterRequest,
}

#[derive(Debug, Clone, Default)]
struct CreateFilterRequest {
	name: String,
	description: Option<String>,
	jql: Option<String>,
	favorite: bool,
	editable: bool,
	expand: Option<Vec<String>>,
}

impl Endpoint for CreateFilterRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"filter".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		let expand = match &self.expand {
			Some(e) => Some(e.join(",")),
			None => None
		};
		params.push_opt("expand", expand);
		params
	}

	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let mut body = JsonFormParams::default();
		body
			.push("name", &self.name)
			.push_opt("description", self.description.as_ref())
			.push_opt("jql", self.jql.as_ref())
			.push("favourite", self.favorite)
			.push("editable", self.editable);
		body.into_body()
	}
}

impl CreateFilterBuilder {
	fn new(client: Arc<RestClient>) -> CreateFilterBuilder {
		CreateFilterBuilder { client, request: CreateFilterRequest::default() }
	}

	fn name(mut self, name: impl Into<String>) -> CreateFilterBuilder {
		self.request.name = name.into();
		self
	}

	/// Set a description for the new filter.
	pub fn description(mut self, description: impl Into<String>) -> CreateFilterBuilder {
		self.request.description = Some(description.into());
		self
	}

	/// Set the JQL query for the new filter.
	pub fn jql(mut self, jql: impl Into<String>) -> CreateFilterBuilder {
		self.request.jql = Some(jql.into());
		self
	}

	/// Set whether the filter should be a favourite filter or not.
	pub fn favorite(mut self, favorite: bool) -> CreateFilterBuilder {
		self.request.favorite = favorite;
		self
	}

	/// Set whether the filter should be editable or not.
	pub fn editable(mut self, editable: bool) -> CreateFilterBuilder {
		self.request.editable = editable;
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.post(self.request).await
	}
}

impl Jira {
	/// Creates a new filter, and returns newly created filter. Currently sets
	/// permissions just using the users default sharing permissions.
	pub fn create_filter(&self, name: impl Into<String>) -> CreateFilterBuilder {
		CreateFilterBuilder::new(Arc::clone(&self.client)).name(name)
	}
}