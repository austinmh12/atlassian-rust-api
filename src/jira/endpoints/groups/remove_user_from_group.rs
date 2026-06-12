use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct RemoveUserFromGroupBuilder {
	client: Arc<RestClient>,
	request: RemoveUserFromGroupRequest,
}

#[derive(Debug, Clone, Default)]
struct RemoveUserFromGroupRequest {
	group_name: String,
	user_name: String,
}

impl Endpoint for RemoveUserFromGroupRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"group/user".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push("groupname", &self.group_name)
			.push("username", &self.user_name);
		params
	}
}

impl RemoveUserFromGroupBuilder {
	fn new(client: Arc<RestClient>) -> RemoveUserFromGroupBuilder {
		RemoveUserFromGroupBuilder { client, request: RemoveUserFromGroupRequest::default() }
	}

	fn group_name(mut self, group_name: impl Into<String>) -> RemoveUserFromGroupBuilder {
		self.request.group_name = group_name.into();
		self
	}

	fn user_name(mut self, user_name: impl Into<String>) -> RemoveUserFromGroupBuilder {
		self.request.user_name = user_name.into();
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Removes given user from a group.
	pub fn remove_user_from_group(&self, group_name: impl Into<String>, user_name: impl Into<String>) -> RemoveUserFromGroupBuilder {
		RemoveUserFromGroupBuilder::new(Arc::clone(&self.client)).group_name(group_name).user_name(user_name)
	}
}