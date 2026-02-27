use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct GetFilterBuilder {
	client: Arc<RestClient>,
	request: GetFilterRequest,
}

#[derive(Debug, Clone, Default)]
struct GetFilterRequest {
	id: u64,
	expand: Option<Vec<String>>,
}

impl Endpoint for GetFilterRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}", self.id).into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		let expand = match &self.expand {
			Some(e) => Some(e.join(",")),
			None => None
		};
		params.push_opt("expand", expand);
		params
	}
}

impl GetFilterBuilder {
	fn new(client: Arc<RestClient>) -> GetFilterBuilder {
		GetFilterBuilder { client, request: GetFilterRequest::default() }
	}

	fn id(mut self, id: u64) -> GetFilterBuilder {
		self.request.id = id;
		self
	}

	/// Add an expand field to the returned data.
	pub fn expand(mut self, expand: impl Into<String>) -> GetFilterBuilder {
		match self.request.expand {
			Some(ref mut f) => f.push(expand.into()),
			None => self.request.expand = Some(vec![expand.into()]),
		};
		self
	}

	/// Add multiple expand fields to the returned data.
	pub fn expands<S>(mut self, expands: impl IntoIterator<Item = S>) -> GetFilterBuilder 
	where
		S: Into<String>,
	{
		let expands = expands.into_iter().map(|f| f.into()).collect::<Vec<String>>();
		match self.request.expand {
			Some(ref mut f) => f.extend(expands),
			None => self.request.expand = Some(expands),
		};
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.get(self.request).await
	}
}

impl Jira {
	/// Returns a filter with a given ID.
	pub fn get_filter(&self, id: u64) -> GetFilterBuilder {
		GetFilterBuilder::new(Arc::clone(&self.client)).id(id)
	}
}