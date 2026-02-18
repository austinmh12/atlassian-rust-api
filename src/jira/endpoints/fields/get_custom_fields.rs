use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetCustomFieldsBuilder {
	client: Arc<RestClient>,
	request: GetCustomFieldsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetCustomFieldsRequest {
	search: Option<String>,
	start_at: Option<u32>,
	limit: Option<u32>,
	// Provided by GetCustomFieldsBuilder
	cloud: bool
}

impl Endpoint for GetCustomFieldsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		match self.cloud {
			true => "field/search".into(),
			false => "customFields".into(),
		}
	}
	
	fn parameters(&self) -> QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push("startAt", self.start_at.unwrap_or(1))
			.push("maxResults", self.limit.unwrap_or(50))
			.push_opt("search", self.search.as_ref());
		params
	}
}

impl GetCustomFieldsBuilder {
	fn new(client: Arc<RestClient>) -> GetCustomFieldsBuilder {
		let cloud = client.cloud.clone();
		GetCustomFieldsBuilder { client, request: GetCustomFieldsRequest { cloud, ..Default::default() } }
	}

	pub fn search(mut self, search: impl Into<String>) -> GetCustomFieldsBuilder {
		self.request.search = Some(search.into());
		self
	}

	pub fn start_at(mut self, start_at: u32) -> GetCustomFieldsBuilder {
		self.request.start_at = Some(start_at);
		self
	}

	pub fn limit(mut self, limit: u32) -> GetCustomFieldsBuilder {
		self.request.limit = Some(limit);
		self
	}

	async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

crate::macros::futurize!(GetCustomFieldsBuilder, serde_json::Value);

impl Jira {
	/// Get custom fields
	pub fn get_custom_fields(&self) -> GetCustomFieldsBuilder {
		GetCustomFieldsBuilder::new(Arc::clone(&self.client))
	}
}