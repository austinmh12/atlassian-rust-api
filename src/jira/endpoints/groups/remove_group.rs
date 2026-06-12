use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct RemoveGroupBuilder {
	client: Arc<RestClient>,
	request: RemoveGroupRequest,
}

#[derive(Debug, Clone, Default)]
struct RemoveGroupRequest {
	group_name: String,
	swap_group: Option<String>
}

impl Endpoint for RemoveGroupRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"group".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push("groupname", &self.group_name)
			.push_opt("swapGroup", self.swap_group.as_ref());

		params
	}
}

impl RemoveGroupBuilder {
	fn new(client: Arc<RestClient>) -> RemoveGroupBuilder {
		RemoveGroupBuilder { client, request: RemoveGroupRequest::default() }
	}

	fn group_name(mut self, group_name: impl Into<String>) -> RemoveGroupBuilder {
		self.request.group_name = group_name.into();
		self
	}

	/// If you delete a group and content is restricted to that group, the content will be hidden from all users. 
	/// To prevent this, use this parameter to specify a different group to transfer the restrictions 
	/// (comments and worklogs only) to.
	pub fn swap_group(mut self, swap_group: impl Into<String>) -> RemoveGroupBuilder {
		self.request.swap_group = Some(swap_group.into());
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Deletes a group by given group parameter.
	pub fn remove_group(&self, group_name: impl Into<String>) -> RemoveGroupBuilder {
		RemoveGroupBuilder::new(Arc::clone(&self.client)).group_name(group_name)
	}
}