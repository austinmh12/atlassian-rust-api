use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams}};

#[derive(Debug, Clone)]
pub struct UpdateComponentBuilder {
	client: Arc<RestClient>,
	request: UpdateComponentRequest,
}

#[derive(Debug, Clone, Default)]
struct UpdateComponentRequest {
	id: u64,
	name: Option<String>,
	description: Option<String>,
	lead_user_name: Option<String>,
	assignee_type: Option<String>,
	is_assignee_type_valid: Option<bool>,
	project_key: Option<String>,
	project_id: Option<u64>,
}

impl Endpoint for UpdateComponentRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("component/{}", self.id).into()
	}

	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let mut body = JsonFormParams::default();
		body
			.push_opt("name", self.name.as_ref())
			.push_opt("project", self.project_key.as_ref())
			.push_opt("projectId", self.project_id)
			.push_opt("isAssigneeTypeValue", self.is_assignee_type_valid)
			.push_opt("description", self.description.as_ref())
			.push_opt("leadUserName", self.lead_user_name.as_ref())
			.push_opt("assigneeType", self.assignee_type.as_ref());
		body.into_body()
	}
}

impl UpdateComponentBuilder {
	fn new(client: Arc<RestClient>) -> UpdateComponentBuilder {
		UpdateComponentBuilder { client, request: UpdateComponentRequest::default() }
	}

	fn id(mut self, id: u64) -> UpdateComponentBuilder {
		self.request.id = id;
		self
	}

	/// Update the name of the component.
	pub fn name(mut self, name: impl Into<String>) -> UpdateComponentBuilder {
		self.request.name = Some(name.into());
		self
	}

	/// Update the description on the new component.
	pub fn description(mut self, description: impl Into<String>) -> UpdateComponentBuilder {
		self.request.description = Some(description.into());
		self
	}

	/// Update the lead of the component.
	pub fn lead_user_name(mut self, lead_user_name: impl Into<String>) -> UpdateComponentBuilder {
		self.request.lead_user_name = Some(lead_user_name.into());
		self
	}

	/// Update the assignee type of the component.
	pub fn assignee_type(mut self, assignee_type: impl Into<String>) -> UpdateComponentBuilder {
		self.request.assignee_type = Some(assignee_type.into());
		self
	}

	/// Update whether the assignee type is valid or not.
	pub fn is_assignee_type_valid(mut self, is_assignee_type_valid: bool) -> UpdateComponentBuilder {
		self.request.is_assignee_type_valid = Some(is_assignee_type_valid);
		self
	}

	/// Update the project key of the component.
	pub fn project_key(mut self, project_key: impl Into<String>) -> UpdateComponentBuilder {
		self.request.project_key = Some(project_key.into());
		self
	}

	/// Update the project ID of the component.
	pub fn project_id(mut self, project_id: u64) -> UpdateComponentBuilder {
		self.request.project_id = Some(project_id);
		self
	}


	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.put(self.request).await
	}
}

impl Jira {
	/// Modify a component. Any fields present in the request will override existing values. As a convenience, 
	/// if a field is not present, it is silently ignored.
	/// 
	/// *Note*: If `lead_user_name` is an empty string (`""`) the component lead will be removed.
	pub fn update_component(&self, id: u64) -> UpdateComponentBuilder {
		UpdateComponentBuilder::new(Arc::clone(&self.client)).id(id)
	}
}