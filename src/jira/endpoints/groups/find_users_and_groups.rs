use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct FindUsersAndGroupsBuilder {
	client: Arc<RestClient>,
	request: FindUsersAndGroupsRequest,
}

#[derive(Debug, Clone, Default)]
struct FindUsersAndGroupsRequest {
	query: String,
	max_results: Option<u64>,
	show_avatar: Option<bool>,
	field_id: Option<String>,
	project_ids: Option<Vec<String>>,
	issue_type_ids: Option<Vec<String>>,
}

impl Endpoint for FindUsersAndGroupsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"groupuserpicker".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		// TODO: These will both fail as CSV is not supported.
		let project_ids = match &self.project_ids {
			Some(pids) => Some(pids.join(",")),
			None => None
		};
		let issue_type_ids = match &self.issue_type_ids {
			Some(itids) => Some(itids.join(",")),
			None => None
		};
		params
			.push("query", &self.query)
			.push_opt("maxResults", self.max_results)
			.push_opt("showAvatar", self.show_avatar)
			.push_opt("fieldId", self.field_id.as_ref())
			.push_opt("projectId", project_ids)
			.push_opt("issueTypeId", issue_type_ids);
		params
	}
}

impl FindUsersAndGroupsBuilder {
	fn new(client: Arc<RestClient>) -> FindUsersAndGroupsBuilder {
		FindUsersAndGroupsBuilder { client, request: FindUsersAndGroupsRequest::default() }
	}

	fn query(mut self, query: impl Into<String>) -> FindUsersAndGroupsBuilder {
		self.request.query = query.into();
		self
	}

	/// The maximum number of users to return. The maximum allowed value is 1000. Values will be
	/// truncated if a number above that is provided.
	pub fn max_results(mut self, max_results: u64) -> FindUsersAndGroupsBuilder {
		self.request.max_results = Some(max_results);
		self
	}

	/// Whether to show the user avatar or not.
	pub fn show_avatar(mut self, show_avatar: bool) -> FindUsersAndGroupsBuilder {
		self.request.show_avatar = Some(show_avatar);
		self
	}

	/// The custom field id, if this request comes from a custom field, such as a user picker.
	pub fn field_id(mut self, field_id: impl Into<String>) -> FindUsersAndGroupsBuilder {
		self.request.field_id = Some(field_id.into());
		self
	}

	/// A project ID to further restrict the search. This parameter can occur multiple times to pass
	/// in multiple project IDs. Comma separated value is not supported. This parameter is only used when 
	/// [`FindUsersAndGroupsBuilder::field_id`] is present.
	/// 
	/// TODO: Support passing same parameter multiple times.
	pub fn project_id(mut self, project_id: impl Into<String>) -> FindUsersAndGroupsBuilder {
		match self.request.project_ids {
			Some(ref mut p) => p.push(project_id.into()),
			None => self.request.project_ids = Some(vec![project_id.into()]),
		};
		self
	}

	/// The list of project IDs to further restrict the search. This parameter can occur multiple times to pass
	/// in multiple project IDs. Comma separated value is not supported. This parameter is only used when 
	/// [`FindUsersAndGroupsBuilder::field_id`] is present.
	/// 
	/// TODO: Support passing same parameter multiple times.
	pub fn project_ids<S>(mut self, project_ids: impl IntoIterator<Item = S>) -> FindUsersAndGroupsBuilder 
	where
		S: Into<String>,
	{
		let project_ids = project_ids.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.project_ids {
			Some(ref mut p) => p.extend(project_ids),
			None => self.request.project_ids = Some(project_ids),
		};
		self
	}

	/// An issue type id to further restrict the search. This parameter can occur multiple times to pass 
	/// in multiple issue type ids. Comma separated value is not supported. Special values such as -1 (all standard 
	/// issue types), -2 (all subtask issue types) are supported. This parameter is only used when 
	/// [`FindUsersAndGroupsBuilder::field_id`] is present.
	/// 
	/// TODO: Support passing same parameter multiple times.
	pub fn issue_type_id(mut self, issue_type_id: impl Into<String>) -> FindUsersAndGroupsBuilder {
		match self.request.issue_type_ids {
			Some(ref mut p) => p.push(issue_type_id.into()),
			None => self.request.issue_type_ids = Some(vec![issue_type_id.into()]),
		};
		self
	}

	/// The list of issue type ids to further restrict the search. This parameter can occur multiple times to pass 
	/// in multiple issue type ids. Comma separated value is not supported. Special values such as -1 (all standard 
	/// issue types), -2 (all subtask issue types) are supported. This parameter is only used when 
	/// [`FindUsersAndGroupsBuilder::field_id`] is present.
	/// 
	/// TODO: Support passing same parameter multiple times.
	pub fn issue_type_ids<S>(mut self, issue_type_ids: impl IntoIterator<Item = S>) -> FindUsersAndGroupsBuilder 
	where
		S: Into<String>,
	{
		let issue_type_ids = issue_type_ids.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.issue_type_ids {
			Some(ref mut p) => p.extend(issue_type_ids),
			None => self.request.issue_type_ids = Some(issue_type_ids),
		};
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns a list of users and groups matching query with highlighting. This resource cannot be accessed anonymously.
	pub fn find_users_and_groups(&self, query: impl Into<String>) -> FindUsersAndGroupsBuilder {
		FindUsersAndGroupsBuilder::new(Arc::clone(&self.client)).query(query)
	}
}