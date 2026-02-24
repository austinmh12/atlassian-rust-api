use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct DeleteComponentBuilder {
	client: Arc<RestClient>,
	request: DeleteComponentRequest,
}

#[derive(Debug, Clone, Default)]
struct DeleteComponentRequest {
	id: u64,
	move_issues_to: Option<u64>,
}

impl Endpoint for DeleteComponentRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("component/{}", self.id).into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params.push_opt("moveIssuesTo", self.move_issues_to);
		params
	}
}

impl DeleteComponentBuilder {
	fn new(client: Arc<RestClient>) -> DeleteComponentBuilder {
		DeleteComponentBuilder { client, request: DeleteComponentRequest::default() }
	}

	fn id(mut self, id: u64) -> DeleteComponentBuilder {
		self.request.id = id;
		self
	}

	/// The new component applied to issues whose 'id' component will be deleted. If this value is null, 
	/// then the 'id' component is simply removed from the related isues.
	pub fn move_issues_to(mut self, move_issues_to: u64) -> DeleteComponentBuilder {
		self.request.move_issues_to = Some(move_issues_to);
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Delete a project component.
	pub fn delete_component(&self, id: u64) -> DeleteComponentBuilder {
		DeleteComponentBuilder::new(Arc::clone(&self.client)).id(id)
	}
}