use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams, QueryParams}};

#[derive(Debug, Clone)]
pub struct EditFilterBuilder {
	client: Arc<RestClient>,
	request: EditFilterRequest,
}

#[derive(Debug, Clone, Default)]
struct EditFilterRequest {
	id: u64,
	name: Option<String>,
	description: Option<String>,
	jql: Option<String>,
	expand: Option<Vec<String>>,
}

impl Endpoint for EditFilterRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}", self.id).into()
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
			.push_opt("name", self.name.as_ref())
			.push_opt("description", self.description.as_ref())
			.push_opt("jql", self.jql.as_ref());
		body.into_body()
	}
}

impl EditFilterBuilder {
	fn new(client: Arc<RestClient>) -> EditFilterBuilder {
		EditFilterBuilder { client, request: EditFilterRequest::default() }
	}

	fn id(mut self, id: u64) -> EditFilterBuilder {
		self.request.id = id;
		self
	}

	/// Change the name of the filter.
	pub fn name(mut self, name: impl Into<String>) -> EditFilterBuilder {
		self.request.name = Some(name.into());
		self
	}

	/// Change the description of the filter.
	pub fn description(mut self, description: impl Into<String>) -> EditFilterBuilder {
		self.request.description = Some(description.into());
		self
	}

	/// Change the query of the filter.
	pub fn jql(mut self, jql: impl Into<String>) -> EditFilterBuilder {
		self.request.jql = Some(jql.into());
		self
	}

	/// Add an expand field to the returned data.
	pub fn expand(mut self, expand: impl Into<String>) -> EditFilterBuilder {
		match self.request.expand {
			Some(ref mut f) => f.push(expand.into()),
			None => self.request.expand = Some(vec![expand.into()]),
		};
		self
	}

	/// Add multiple expand fields to the returned data.
	pub fn expands<S>(mut self, expands: impl IntoIterator<Item = S>) -> EditFilterBuilder 
	where
		S: Into<String>,
	{
		let expands = expands.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.expand {
			Some(ref mut f) => f.extend(expands),
			None => self.request.expand = Some(expands),
		};
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.put(self.request).await
	}
}

impl Jira {
	/// Updates an existing filter, and returns its new value. 
	/// 
	/// The following properties of
	/// a filter can be updated: `jql`, `name`, `description`. Additionally, administrators 
	/// can also update the `owner` field. To get, set or unset `favourite`, use 
	/// rest/api/1.0/filters/{id}/favourite with GET, PUT and DELETE methods instead. 
	pub fn edit_filter(&self, id: u64) -> EditFilterBuilder {
		EditFilterBuilder::new(Arc::clone(&self.client)).id(id)
	}
}