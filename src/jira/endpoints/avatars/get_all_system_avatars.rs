use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetAllSystemAvatarsBuilder {
	client: Arc<RestClient>,
	request: GetAllSystemAvatarsRequest,
}

#[derive(Debug, Clone, Default)]
struct GetAllSystemAvatarsRequest {
	avatar_type: String,
}

impl Endpoint for GetAllSystemAvatarsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("avatar/{}/system", &self.avatar_type).into()
	}
}

impl GetAllSystemAvatarsBuilder {
	fn new(client: Arc<RestClient>) -> GetAllSystemAvatarsBuilder {
		GetAllSystemAvatarsBuilder { client, request: GetAllSystemAvatarsRequest::default() }
	}

	fn avatar_type(mut self, avatar_type: impl Into<String>) -> GetAllSystemAvatarsBuilder {
		self.request.avatar_type = avatar_type.into();
		self
	}

	async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

crate::macros::futurize!(GetAllSystemAvatarsBuilder, serde_json::Value);

impl Jira {
	/// Returns all system avatars of the given type
	pub async fn get_all_system_avatars(&self, avatar_type: impl Into<String>) -> Result<serde_json::Value> {
		GetAllSystemAvatarsBuilder::new(Arc::clone(&self.client)).avatar_type(avatar_type).await
	}
}