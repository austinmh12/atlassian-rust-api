use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetPropertyBuilder {
	client: Arc<RestClient>,
	request: GetPropertyRequest,
}

#[derive(Debug, Clone, Default)]
struct GetPropertyRequest {
	key: Option<String>,
	permission_level: Option<String>,
	key_filter: Option<String>,
}

impl Endpoint for GetPropertyRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"application-properties".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push_opt("key", self.key.as_ref())
			.push_opt("permissionLevel", self.permission_level.as_ref())
			.push_opt("keyFilter", self.key_filter.as_ref());
		params
	}
}

impl GetPropertyBuilder {
	fn new(client: Arc<RestClient>) -> GetPropertyBuilder {
		GetPropertyBuilder { client, request: GetPropertyRequest::default() }
	}

	/// A String containing the property key.
	pub fn key(mut self, key: impl Into<String>) -> GetPropertyBuilder {
		self.request.key = Some(key.into());
		self
	}

	/// When fetching a list, specifies the permission level of all items in the list.
	pub fn permission_level(mut self, permission_level: impl Into<String>) -> GetPropertyBuilder {
		self.request.permission_level = Some(permission_level.into());
		self
	}

	/// When fetching a list, allows the list to be filtered by the property's start of key e.g. `"jira.if.*"` would
	/// fetch only those permissions that are editable and whose keys start with `"jira.if."`. This is a regex.
	pub fn key_filter(mut self, key_filter: impl Into<String>) -> GetPropertyBuilder {
		self.request.key_filter = Some(key_filter.into());
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns an application property or a list of application properties.
	/// 
	/// ```no_run
	/// let jira = Jira::builder().url("http://jira.example.com")
	/// 	.username("user")
	/// 	.password("password")
	/// 	.build()?;
	/// // Single property
	/// let prop = jira.get_properties()
	/// 	.key("jira.home").await?;
	/// // Mutlitple properties
	/// let props = jira.get_properties().await?;
	/// ```
	pub fn get_properties(&self) -> GetPropertyBuilder {
		GetPropertyBuilder::new(Arc::clone(&self.client))
	}
}