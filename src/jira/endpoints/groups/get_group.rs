use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetGroupBuilder {
	client: Arc<RestClient>,
	request: GetGroupRequest,
}

#[derive(Debug, Clone, Default)]
struct GetGroupRequest {
	group_name: String,
	expand: Option<Vec<String>>,
}

impl Endpoint for GetGroupRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"group".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		let expand = match &self.expand {
			Some(e) => Some(e.join(",")),
			None => None
		};
		params
			.push("groupname", &self.group_name)
			.push_opt("expand", expand);

		params
	}
}

impl GetGroupBuilder {
	fn new(client: Arc<RestClient>) -> GetGroupBuilder {
		GetGroupBuilder { client, request: GetGroupRequest::default() }
	}

	fn group_name(mut self, group_name: impl Into<String>) -> GetGroupBuilder {
		self.request.group_name = group_name.into();
		self
	}

	/// Add an expand field to the returned data.
	pub fn expand(mut self, expand: impl Into<String>) -> GetGroupBuilder {
		match self.request.expand {
			Some(ref mut f) => f.push(expand.into()),
			None => self.request.expand = Some(vec![expand.into()]),
		};
		self
	}

	/// Add multiple expand fields to the returned data.
	pub fn expands<S>(mut self, expands: impl IntoIterator<Item = S>) -> GetGroupBuilder 
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
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns REST representation for the requested group. Allows to get list of active users belonging to the 
	/// specified group and its subgroups if "users" expand option is provided. You can page through users list 
	/// by using indexes in expand param. For example to get users from index 10 to index 15 use "users[10:15]" expand 
	/// value. This will return 6 users (if there are at least 16 users in this group). Indexes are 0-based and inclusive.
	///
	/// This resource is deprecated, please use group/member API instead.
	#[deprecated(since="0.1.0", note="use `get_users_from_group` instead")]
	pub fn get_group(&self, group_name: impl Into<String>) -> GetGroupBuilder {
		GetGroupBuilder::new(Arc::clone(&self.client)).group_name(group_name)
	}
}