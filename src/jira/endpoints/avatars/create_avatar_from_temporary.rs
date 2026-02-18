use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct CreateAvatarFromTemporaryBuilder {
	client: Arc<RestClient>,
	request: CreateAvatarFromTemporaryRequest,
}

#[derive(Debug, Clone, Default)]
struct CreateAvatarFromTemporaryRequest {
	avatar_type: String,
	width: u64,
	offset_x: u64,
	offset_y: u64,
	needs_cropping: bool,
}

impl Endpoint for CreateAvatarFromTemporaryRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("avatar/{}/temporaryCrop", &self.avatar_type).into()
	}
}

impl CreateAvatarFromTemporaryBuilder {
	fn new(client: Arc<RestClient>) -> CreateAvatarFromTemporaryBuilder {
		CreateAvatarFromTemporaryBuilder { client, request: CreateAvatarFromTemporaryRequest::default() }
	}
	
	fn avatar_type(mut self, avatar_type: impl Into<String>) -> CreateAvatarFromTemporaryBuilder {
		self.request.avatar_type = avatar_type.into();
		self
	}

	pub fn width(mut self, width: u64) -> CreateAvatarFromTemporaryBuilder {
		self.request.width = width;
		self
	}

	pub fn offset_x(mut self, offset_x: u64) -> CreateAvatarFromTemporaryBuilder {
		self.request.offset_x = offset_x;
		self
	}

	pub fn offset_y(mut self, offset_y: u64) -> CreateAvatarFromTemporaryBuilder {
		self.request.offset_y = offset_y;
		self
	}

	pub fn needs_cropping(mut self, needs_cropping: bool) -> CreateAvatarFromTemporaryBuilder {
		self.request.needs_cropping = needs_cropping;
		self
	}

	async fn send(self) -> Result<()> {
		self.client.post_ignore(self.request).await
	}
}

crate::macros::futurize!(CreateAvatarFromTemporaryBuilder);

impl Jira {
	/// Updates the cropping instructions of the temporary avatar.
	pub fn create_avatar_from_temporary(&self, avatar_type: impl Into<String>) -> CreateAvatarFromTemporaryBuilder {
		CreateAvatarFromTemporaryBuilder::new(Arc::clone(&self.client)).avatar_type(avatar_type)
	}
}