use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetComponentRelatedIssuesBuilder {
	client: Arc<RestClient>,
	request: GetComponentRelatedIssuesRequest,
}

#[derive(Debug, Clone, Default)]
struct GetComponentRelatedIssuesRequest {
	id: u64
}

impl Endpoint for GetComponentRelatedIssuesRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("component/{}/relatedIssueCounts", self.id).into()
	}
}

impl GetComponentRelatedIssuesBuilder {
	fn new(client: Arc<RestClient>) -> GetComponentRelatedIssuesBuilder {
		GetComponentRelatedIssuesBuilder { client, request: GetComponentRelatedIssuesRequest::default() }
	}

	fn id(mut self, id: u64) -> GetComponentRelatedIssuesBuilder {
		self.request.id = id;
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns counts of issues related to this component.
	pub fn get_component_related_issues(&self, id: u64) -> GetComponentRelatedIssuesBuilder {
		GetComponentRelatedIssuesBuilder::new(Arc::clone(&self.client)).id(id)
	}
}