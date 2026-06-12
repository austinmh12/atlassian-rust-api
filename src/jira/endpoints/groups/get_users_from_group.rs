use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetUsersFromGroupBuilder {
	client: Arc<RestClient>,
	request: GetUsersFromGroupRequest,
}

#[derive(Debug, Clone, Default)]
struct GetUsersFromGroupRequest {
	group_name: String,
	include_inactive_users: Option<bool>,
	start_at: Option<u64>,
	max_results: Option<u64>,
}

impl Endpoint for GetUsersFromGroupRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"group/member".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push("groupname", &self.group_name)
			.push_opt("includeInactiveUsers", self.include_inactive_users)
			.push_opt("startAt", self.start_at)
			.push_opt("maxResults", self.max_results);

		params
	}
}

impl GetUsersFromGroupBuilder {
	fn new(client: Arc<RestClient>) -> GetUsersFromGroupBuilder {
		GetUsersFromGroupBuilder { client, request: GetUsersFromGroupRequest::default() }
	}

	fn group_name(mut self, group_name: impl Into<String>) -> GetUsersFromGroupBuilder {
		self.request.group_name = group_name.into();
		self
	}

	/// Inactive users will be included in the response if set to `true`.
	pub fn include_inactive_users(mut self, include_inactive_users: bool) -> GetUsersFromGroupBuilder {
		self.request.include_inactive_users = Some(include_inactive_users);
		self
	}

	/// Index of the first user in the group to return.
	pub fn start_at(mut self, start_at: u64) -> GetUsersFromGroupBuilder {
		self.request.start_at = Some(start_at);
		self
	}

	/// Maximum number of users to return.
	pub fn max_results(mut self, max_results: u64) -> GetUsersFromGroupBuilder {
		self.request.max_results = Some(max_results);
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// This resource returns a paginated list of users who are members of the specified group and its subgroups. 
	/// Users in the page are ordered by user names. User of this resource is required to have sysadmin or admin permissions.
	pub fn get_users_from_group(&self, group_name: impl Into<String>) -> GetUsersFromGroupBuilder {
		GetUsersFromGroupBuilder::new(Arc::clone(&self.client)).group_name(group_name)
	}
}