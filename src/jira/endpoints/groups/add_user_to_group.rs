use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams, QueryParams}};

#[derive(Debug, Clone)]
pub struct AddUserToGroupBuilder {
	client: Arc<RestClient>,
	request: AddUserToGroupRequest,
}

#[derive(Debug, Clone, Default)]
struct AddUserToGroupRequest {
	group_name: String,
	user_name: String,
}

impl Endpoint for AddUserToGroupRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"group/user".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params.push("groupname", &self.group_name);
		params
	}

	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let mut body = JsonFormParams::default();
		body.push("name", &self.user_name);
		body.into_body()
	}
}

impl AddUserToGroupBuilder {
	fn new(client: Arc<RestClient>) -> AddUserToGroupBuilder {
		AddUserToGroupBuilder { client, request: AddUserToGroupRequest::default() }
	}

	fn group_name(mut self, group_name: impl Into<String>) -> AddUserToGroupBuilder {
		self.request.group_name = group_name.into();
		self
	}

	fn user_name(mut self, user_name: impl Into<String>) -> AddUserToGroupBuilder {
		self.request.user_name = user_name.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.post(self.request).await
	}
}

impl Jira {
	/// Adds given user to a group. Returns the current state of the group.
	pub fn add_user_to_group(&self, group_name: impl Into<String>, user_name: impl Into<String>) -> AddUserToGroupBuilder {
		AddUserToGroupBuilder::new(Arc::clone(&self.client)).group_name(group_name).user_name(user_name)
	}
}