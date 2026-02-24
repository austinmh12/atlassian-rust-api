use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams}};

#[derive(Debug, Clone)]
pub struct CreateComponentBuilder {
	client: Arc<RestClient>,
	request: CreateComponentRequest,
}

#[derive(Debug, Clone, Default)]
struct CreateComponentRequest {
	name: String,
	description: Option<String>,
	lead_user_name: Option<String>,
	assignee_type: Option<String>,
	is_assignee_type_valid: bool,
	project_key: String,
	project_id: u64,
}

impl Endpoint for CreateComponentRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"component".into()
	}

	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let mut body = JsonFormParams::default();
		body
			.push("name", &self.name)
			.push("project", &self.project_key)
			.push("projectId", self.project_id)
			.push("isAssigneeTypeValue", self.is_assignee_type_valid)
			.push_opt("description", self.description.as_ref())
			.push_opt("leadUserName", self.lead_user_name.as_ref())
			.push_opt("assigneeType", self.assignee_type.as_ref());
		body.into_body()
	}
}

impl CreateComponentBuilder {
	fn new(client: Arc<RestClient>) -> CreateComponentBuilder {
		CreateComponentBuilder { client, request: CreateComponentRequest::default() }
	}

	fn name(mut self, name: impl Into<String>) -> CreateComponentBuilder {
		self.request.name = name.into();
		self
	}

	/// Set a description on the new component.
	pub fn description(mut self, description: impl Into<String>) -> CreateComponentBuilder {
		self.request.description = Some(description.into());
		self
	}

	/// Set the lead of the component.
	pub fn lead_user_name(mut self, lead_user_name: impl Into<String>) -> CreateComponentBuilder {
		self.request.lead_user_name = Some(lead_user_name.into());
		self
	}

	/// Set the assignee type of the component.
	pub fn assignee_type(mut self, assignee_type: impl Into<String>) -> CreateComponentBuilder {
		self.request.assignee_type = Some(assignee_type.into());
		self
	}

	/// Set whether the assignee type is valid or not.
	pub fn is_assignee_type_valid(mut self, is_assignee_type_valid: bool) -> CreateComponentBuilder {
		self.request.is_assignee_type_valid = is_assignee_type_valid;
		self
	}

	fn project_key(mut self, project_key: impl Into<String>) -> CreateComponentBuilder {
		self.request.project_key = project_key.into();
		self
	}

	fn project_id(mut self, project_id: u64) -> CreateComponentBuilder {
		self.request.project_id = project_id;
		self
	}


	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.post(self.request).await
	}
}

impl Jira {
	/// Create a component.
	pub fn create_component(
		&self,
		name: impl Into<String>,
		project: impl Into<String>,
		project_id: u64
	) -> CreateComponentBuilder {
		CreateComponentBuilder::new(Arc::clone(&self.client)).name(name).project_key(project).project_id(project_id)
	}
}