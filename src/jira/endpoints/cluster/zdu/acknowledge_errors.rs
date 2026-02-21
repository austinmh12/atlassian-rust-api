use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct AcknowledgeErrorsBuilder {
	client: Arc<RestClient>,
	request: AcknowledgeErrorsRequest,
}

#[derive(Debug, Clone, Default)]
struct AcknowledgeErrorsRequest;

impl Endpoint for AcknowledgeErrorsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"cluster/zdu/retryUpgrade".into()
	}
}

impl AcknowledgeErrorsBuilder {
	fn new(client: Arc<RestClient>) -> AcknowledgeErrorsBuilder {
		AcknowledgeErrorsBuilder { client, request: AcknowledgeErrorsRequest::default() }
	}

	pub async fn send(self) -> Result<()> {
		self.client.post_ignore(self.request).await
	}
}

impl Jira {
	pub fn acknowledge_errors(&self) -> AcknowledgeErrorsBuilder {
		AcknowledgeErrorsBuilder::new(Arc::clone(&self.client))
	}
}