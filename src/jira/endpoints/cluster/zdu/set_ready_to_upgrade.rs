use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct SetReadyToUpgradeBuilder {
	client: Arc<RestClient>,
	request: SetReadyToUpgradeRequest,
}

#[derive(Debug, Clone, Default)]
struct SetReadyToUpgradeRequest;

impl Endpoint for SetReadyToUpgradeRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"cluster/zdu/start".into()
	}
}

impl SetReadyToUpgradeBuilder {
	fn new(client: Arc<RestClient>) -> SetReadyToUpgradeBuilder {
		SetReadyToUpgradeBuilder { client, request: SetReadyToUpgradeRequest::default() }
	}

	pub async fn send(self) -> Result<()> {
		self.client.post_ignore(self.request).await
	}
}

impl Jira {
	pub fn set_ready_to_upgrade(&self) -> SetReadyToUpgradeBuilder {
		SetReadyToUpgradeBuilder::new(Arc::clone(&self.client))
	}
}