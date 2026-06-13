use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct IsIndexSnapshotRunningBuilder {
	client: Arc<RestClient>,
	request: IsIndexSnapshotRunningRequest,
}

#[derive(Debug, Clone, Default)]
struct IsIndexSnapshotRunningRequest {}

impl Endpoint for IsIndexSnapshotRunningRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"index-snapshot/isRunning".into()
	}
}

impl IsIndexSnapshotRunningBuilder {
	fn new(client: Arc<RestClient>) -> IsIndexSnapshotRunningBuilder {
		IsIndexSnapshotRunningBuilder { client, request: IsIndexSnapshotRunningRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Answers true if index snapshot creation is currently running.
	///
	/// Only System Administrator can request current snapshot creation status.
	pub fn is_index_snapshot_running(&self) -> IsIndexSnapshotRunningBuilder {
		IsIndexSnapshotRunningBuilder::new(Arc::clone(&self.client))
	}
}