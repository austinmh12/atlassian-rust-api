use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams}};

#[derive(Debug, Clone)]
pub struct UpdateApplicationRolesBuilder {
	client: Arc<RestClient>,
	request: UpdateApplicationRolesRequest,
}

#[derive(Debug, Clone, Default)]
struct UpdateApplicationRolesRequest {
	key: String,
	groups: Vec<String>,
	default_groups: Vec<String>,
	//TODO: if_match: Option<String>,
}

impl Endpoint for UpdateApplicationRolesRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"applicationrole".into()
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

impl UpdateApplicationRolesBuilder {
	fn new(client: Arc<RestClient>) -> UpdateApplicationRolesBuilder {
		UpdateApplicationRolesBuilder { client, request: UpdateApplicationRolesRequest::default() }
	}

	fn key(mut self, key: impl Into<String>) -> UpdateApplicationRolesBuilder {
		self.request.key = key.into();
		self
	}

	/// Add a group to the update request.
	pub fn group(mut self, group: impl Into<String>) -> UpdateApplicationRolesBuilder {
		self.request.groups.push(group.into());
		self
	}

	/// Add multiple groups to the update request.
	pub fn groups<S>(mut self, groups: impl IntoIterator<Item = S>) -> UpdateApplicationRolesBuilder 
	where
		S: Into<String>,
	{
		let groups = groups.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		self.request.groups.extend(groups);
		self
	}

	/// Add a default group to the update request.
	pub fn default_group(mut self, default_group: impl Into<String>) -> UpdateApplicationRolesBuilder {
		self.request.default_groups.push(default_group.into());
		self
	}

	/// Add multiple default groups to the update request.
	pub fn default_groups<S>(mut self, default_groups: impl IntoIterator<Item = S>) -> UpdateApplicationRolesBuilder 
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
	/// Updates the ApplicationRoles with the passed data if the version hash is the same as the server. Only
	/// the groups and default groups setting of the role may be updated. It is acceptable to pass only the roles 
	/// that are updated as roles that are present in the server but not in data to update with, will not be deleted.
	/// 
	/// *Note*: This currently does the same as [`Jira::update_role`] since this accepts 1 role
	// TODO: Look at how I handle this in tetanus::Safe::update_issue_properties
	pub fn update_roles(&self, key: impl Into<String>) -> UpdateApplicationRolesBuilder {
		UpdateApplicationRolesBuilder::new(Arc::clone(&self.client)).key(key)
	}
}