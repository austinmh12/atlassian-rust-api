use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetIssueBuilder {
	client: Arc<RestClient>,
	request: GetIssueRequest,
}

#[derive(Debug, Clone, Default)]
struct GetIssueRequest {
	key: String,
	fields: Option<Vec<String>>,
	properties: Option<Vec<String>>,
	expand: Option<Vec<String>>,
	update_history: Option<bool>,
}

impl Endpoint for GetIssueRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("issue/{}", &self.key).into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		let fields = match &self.fields {
			Some(fields) => fields.join(","),
			None => "*all".to_string()
		};
		let properties = match &self.properties {
			Some(properties) => Some(properties.join(",")),
			None => None
		};
		let expand = match &self.expand {
			Some(expand) => Some(expand.join(",")),
			None => None
		};
		params
			.push("fields", fields)
			.push_opt("properties", properties)
			.push_opt("expand", expand)
			.push_opt("updateHistory", self.update_history);
		params
	}
}

impl GetIssueBuilder {
	pub(crate) fn new(client: Arc<RestClient>) -> GetIssueBuilder {
		GetIssueBuilder { client, request: GetIssueRequest::default() }
	}

	fn key(mut self, key: impl Into<String>) -> GetIssueBuilder {
		self.request.key = key.into();
		self
	}

	pub fn field(mut self, field: impl Into<String>) -> GetIssueBuilder {
		match self.request.fields {
			Some(ref mut f) => f.push(field.into()),
			None => self.request.fields = Some(vec![field.into()]),
		};
		self
	}

	pub fn fields<S>(mut self, fields: impl IntoIterator<Item = S>) -> GetIssueBuilder 
	where
		S: Into<String>,
	{
		let fields = fields.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.fields {
			Some(ref mut f) => f.extend(fields),
			None => self.request.fields = Some(fields),
		};
		self
	}

	pub fn property(mut self, property: impl Into<String>) -> GetIssueBuilder {
		match self.request.properties {
			Some(ref mut p) => p.push(property.into()),
			None => self.request.properties = Some(vec![property.into()]),
		};
		self
	}

	pub fn properties<S>(mut self, properties: impl IntoIterator<Item = S>) -> GetIssueBuilder 
	where
		S: Into<String>,
	{
		let properties = properties.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.properties {
			Some(ref mut f) => f.extend(properties),
			None => self.request.properties = Some(properties),
		};
		self
	}

	pub fn expand(mut self, expand: impl Into<String>) -> GetIssueBuilder {
		match self.request.expand {
			Some(ref mut f) => f.push(expand.into()),
			None => self.request.expand = Some(vec![expand.into()]),
		};
		self
	}

	pub fn expands<S>(mut self, expands: impl IntoIterator<Item = S>) -> GetIssueBuilder 
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

	pub fn update_history(mut self, update_history: bool) -> GetIssueBuilder {
		self.request.update_history = Some(update_history);
		self
	}

	async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

crate::macros::futurize!(GetIssueBuilder, serde_json::Value);

impl Jira {
	/// Returns a full representation of the issue for the given issue key. By default, all
	/// fields are returned.
	pub fn get_issue(&self, key: impl Into<String>) -> GetIssueBuilder {
		GetIssueBuilder::new(Arc::clone(&self.client)).key(key)
	}

	/// Alias for [`Jira::get_issue`]
	pub fn issue(&self, key: impl Into<String>) -> GetIssueBuilder {
		self.get_issue(key)
	}
}