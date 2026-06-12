use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct GetFavoriteFiltersBuilder {
	client: Arc<RestClient>,
	request: GetFavoriteFiltersRequest,
}

#[derive(Debug, Clone, Default)]
struct GetFavoriteFiltersRequest {}

impl Endpoint for GetFavoriteFiltersRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"filter/favourite".into()
	}
}

impl GetFavoriteFiltersBuilder {
	fn new(client: Arc<RestClient>) -> GetFavoriteFiltersBuilder {
		GetFavoriteFiltersBuilder { client, request: GetFavoriteFiltersRequest::default() }
	}

	pub async fn send(self) -> Result<serde_json::Value> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns the favorite filters of the logged in user.
	pub fn get_favorite_filters(&self) -> GetFavoriteFiltersBuilder {
		GetFavoriteFiltersBuilder::new(Arc::clone(&self.client))
	}

	/// Returns the favourite filters of the logged in users but in Australian.
	/// 
	/// An alias for [Jira::get_favorite_filters].
	pub fn get_favourite_filters(&self) -> GetFavoriteFiltersBuilder {
		self.get_favorite_filters()
	}
}