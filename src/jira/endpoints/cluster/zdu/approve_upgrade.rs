use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct ApproveUpgradeBuilder {
	client: Arc<RestClient>,
	request: ApproveUpgradeRequest,
}

#[derive(Debug, Clone, Default)]
struct ApproveUpgradeRequest;

impl Endpoint for ApproveUpgradeRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"cluster/zdu/approve".into()
	}
}

impl ApproveUpgradeBuilder {
	fn new(client: Arc<RestClient>) -> ApproveUpgradeBuilder {
		ApproveUpgradeBuilder { client, request: ApproveUpgradeRequest::default() }
	}

	pub async fn send(self) -> Result<()> {
		self.client.post_ignore(self.request).await
	}
}

impl Jira {
	pub fn approve_upgrade(&self) -> ApproveUpgradeBuilder {
		ApproveUpgradeBuilder::new(Arc::clone(&self.client))
	}
}