use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct CancelUpgradeBuilder {
	client: Arc<RestClient>,
	request: CancelUpgradeRequest,
}

#[derive(Debug, Clone, Default)]
struct CancelUpgradeRequest;

impl Endpoint for CancelUpgradeRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"cluster/zdu/cancel".into()
	}
}

impl CancelUpgradeBuilder {
	fn new(client: Arc<RestClient>) -> CancelUpgradeBuilder {
		CancelUpgradeBuilder { client, request: CancelUpgradeRequest::default() }
	}

	pub async fn send(self) -> Result<()> {
		self.client.post_ignore(self.request).await
	}
}

impl Jira {
	pub fn cancel_upgrade(&self) -> CancelUpgradeBuilder {
		CancelUpgradeBuilder::new(Arc::clone(&self.client))
	}
}