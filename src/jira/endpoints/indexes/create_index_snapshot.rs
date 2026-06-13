use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct CreateIndexSnapshotBuilder {
	client: Arc<RestClient>,
	request: CreateIndexSnapshotRequest,
}

#[derive(Debug, Clone, Default)]
struct CreateIndexSnapshotRequest {}

impl Endpoint for CreateIndexSnapshotRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"index-snapshot".into()
	}
}

impl CreateIndexSnapshotBuilder {
	fn new(client: Arc<RestClient>) -> CreateIndexSnapshotBuilder {
		CreateIndexSnapshotBuilder { client, request: CreateIndexSnapshotRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.post(self.request).await
	}
}

impl Jira {
	/// Tries to start taking an index snapshot if no other snapshot creation process is in progress. Performs a 
	/// cleanup of index snapshots directory so only a limited number of most recent snapshots are persisted. If 
	/// another snapshot creation process is in progress, returns `409` without waiting for the other process to complete.
	///
	/// Only System Administrator can request creation of snapshot. There is no guarantee as to the time after which 
	/// the snapshot will be available.
	pub fn create_index_snapshot(&self) -> CreateIndexSnapshotBuilder {
		CreateIndexSnapshotBuilder::new(Arc::clone(&self.client))
	}
}