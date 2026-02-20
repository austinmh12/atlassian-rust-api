use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams}};

#[derive(Debug, Clone)]
pub struct UpdateApplicationRoleBuilder {
	client: Arc<RestClient>,
	request: UpdateApplicationRoleRequest,
}

#[derive(Debug, Clone, Default)]
struct UpdateApplicationRoleRequest {
	key: String,
	groups: Vec<String>,
	default_groups: Vec<String>,
	//TODO: if_match: Option<String>,
}

impl Endpoint for UpdateApplicationRoleRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("applicationrole/{}", &self.key).into()
	}

	// TODO: This has a header parameter of If-Match, client doesn't currently support that

	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let mut body = JsonFormParams::default();
		body
			.push("key", &self.key)
			.push("groups", &self.groups)
			.push("defaultGroups", &self.default_groups);
		body.into_body()
	}
}

impl UpdateApplicationRoleBuilder {
	fn new(client: Arc<RestClient>) -> UpdateApplicationRoleBuilder {
		UpdateApplicationRoleBuilder { client, request: UpdateApplicationRoleRequest::default() }
	}

	fn key(mut self, key: impl Into<String>) -> UpdateApplicationRoleBuilder {
		self.request.key = key.into();
		self
	}

	/// Add a group to the update request.
	pub fn group(mut self, group: impl Into<String>) -> UpdateApplicationRoleBuilder {
		self.request.groups.push(group.into());
		self
	}

	/// Add multiple groups to the update request.
	pub fn groups<S>(mut self, groups: impl IntoIterator<Item = S>) -> UpdateApplicationRoleBuilder 
	where
		S: Into<String>,
	{
		let groups = groups.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		self.request.groups.extend(groups);
		self
	}

	/// Add a default group to the update request.
	pub fn default_group(mut self, default_group: impl Into<String>) -> UpdateApplicationRoleBuilder {
		self.request.default_groups.push(default_group.into());
		self
	}

	/// Add multiple default groups to the update request.
	pub fn default_groups<S>(mut self, default_groups: impl IntoIterator<Item = S>) -> UpdateApplicationRoleBuilder 
	where
		S: Into<String>,
	{
		let default_groups = default_groups.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		self.request.default_groups.extend(default_groups);
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.put(self.request).await
	}
}

impl Jira {
	/// Updates the ApplicationRole with the passed data. Only the groups and default groups setting of the role may be updated.
	/// 
	/// **NOT IMPLEMENTED YET** *Optional*: if `versionHash` is passed through the `If-Match` header, the request will be rejected if not the same
	/// as the server.
	// TODO: If-Match header
	pub fn update_role(&self, key: impl Into<String>) -> UpdateApplicationRoleBuilder {
		UpdateApplicationRoleBuilder::new(Arc::clone(&self.client)).key(key)
	}
}