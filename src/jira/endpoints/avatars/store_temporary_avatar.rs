use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct StoreTemporaryAvatarBuilder {
	client: Arc<RestClient>,
	request: StoreTemporaryAvatarRequest,
}

#[derive(Debug, Clone, Default)]
struct StoreTemporaryAvatarRequest {
	avatar_type: String,
	filename: String,
	size: u64,
}

impl Endpoint for StoreTemporaryAvatarRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("avatar/{}/temporary", &self.avatar_type).into()
	}

	fn parameters(&self) -> QueryParams<'_> {
		let mut params = QueryParams::default();
		params
			.push("filename", &self.filename)
			.push("size", self.size);
		params
	}
}

impl StoreTemporaryAvatarBuilder {
	fn new(client: Arc<RestClient>) -> StoreTemporaryAvatarBuilder {
		StoreTemporaryAvatarBuilder { client, request: StoreTemporaryAvatarRequest::default() }
	}

	fn avatar_type(mut self, avatar_type: impl Into<String>) -> StoreTemporaryAvatarBuilder {
		self.request.avatar_type = avatar_type.into();
		self
	}

	fn filename(mut self, filename: impl Into<String>) -> StoreTemporaryAvatarBuilder {
		self.request.filename = filename.into();
		self
	}

	fn size(mut self, size: u64) -> StoreTemporaryAvatarBuilder {
		self.request.size = size.into();
		self
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.post(self.request).await
	}
}

impl Jira {
	/// Creates a temporary avatar and returns cropping instructions
	pub fn store_temporary_avatar(
		&self,
		avatar_type: impl Into<String>,
		filename: impl Into<String>,
		size: u64,
	) -> StoreTemporaryAvatarBuilder {
		StoreTemporaryAvatarBuilder::new(Arc::clone(&self.client))
			.avatar_type(avatar_type)
			.filename(filename)
			.size(size)
	}
}