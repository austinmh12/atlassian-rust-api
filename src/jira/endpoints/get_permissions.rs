use std::sync::Arc;

use crate::{Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetPermissionsBuilder {
	client: Arc<RestClient>,
	request: GetPermissionsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetPermissionsRequest {
	permissions: Vec<String>,
	project_id: Option<u32>,
	project_key: Option<String>,
	issue_id: Option<u32>,
	issue_key: Option<String>,
}

impl Endpoint for GetPermissionsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"mypermissions".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push("permissions", self.permissions.join(","))
			.push_opt("projectId", self.project_id)
			.push_opt("projectKey", self.project_key.as_ref())
			.push_opt("issueId", self.issue_id)
			.push_opt("issueKey", self.issue_key.as_ref());
		params
	}
}

impl GetPermissionsBuilder {
	pub(crate) fn new(client: Arc<RestClient>) -> GetPermissionsBuilder {
		GetPermissionsBuilder { client, request: GetPermissionsRequest::default() }
	}

	/// Add a single permission to the request
	pub fn permission(mut self, permission: impl Into<String>) -> GetPermissionsBuilder {
		self.request.permissions.push(permission.into());
		self
	}

	/// Add multiple permissions to the request
	pub fn permissions(mut self, permissions: Vec<impl Into<String>>) -> GetPermissionsBuilder {
		let permissions = permissions.into_iter().map(|p| p.into()).collect::<Vec<String>>();
		self.request.permissions.extend(permissions);
		self
	}

	pub fn project_id(mut self, project_id: u32) -> GetPermissionsBuilder {
		self.request.project_id = Some(project_id);
		self
	}

	pub fn project_key(mut self, project_key: impl Into<String>) -> GetPermissionsBuilder {
		self.request.project_key = Some(project_key.into());
		self
	}

	pub fn issue_id(mut self, issue_id: u32) -> GetPermissionsBuilder {
		self.request.issue_id = Some(issue_id);
		self
	}

	pub fn issue_key(mut self, issue_key: impl Into<String>) -> GetPermissionsBuilder {
		self.request.issue_key = Some(issue_key.into());
		self
	}

	async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}