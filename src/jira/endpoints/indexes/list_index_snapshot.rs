use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct ListIndexSnapshotBuilder {
	client: Arc<RestClient>,
	request: ListIndexSnapshotRequest,
}

#[derive(Debug, Clone, Default)]
struct ListIndexSnapshotRequest {}

impl Endpoint for ListIndexSnapshotRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"index-snapshot".into()
	}
}

impl ListIndexSnapshotBuilder {
	fn new(client: Arc<RestClient>) -> ListIndexSnapshotBuilder {
		ListIndexSnapshotBuilder { client, request: ListIndexSnapshotRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Lists available index snapshots absolute paths with timestamps.
	///
	/// Only System Administrator can request listing index snapshots.
	pub fn list_index_snapshot(&self) -> ListIndexSnapshotBuilder {
		ListIndexSnapshotBuilder::new(Arc::clone(&self.client))
	}
}