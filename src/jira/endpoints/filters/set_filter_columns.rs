use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct SetFilterColumnsBuilder {
	client: Arc<RestClient>,
	request: SetFilterColumnsRequest,
}

#[derive(Debug, Clone, Default)]
struct SetFilterColumnsRequest {
	id: u64,
	columns: Vec<String>,
}

impl Endpoint for SetFilterColumnsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("filter/{}/columns", self.id).into()
	}

	// TODO: Body
}

impl SetFilterColumnsBuilder {
	fn new(client: Arc<RestClient>) -> SetFilterColumnsBuilder {
		SetFilterColumnsBuilder { client, request: SetFilterColumnsRequest::default() }
	}

	fn id(mut self, id: u64) -> SetFilterColumnsBuilder {
		self.request.id = id;
		self
	}

	/// Name of the columns to set on the filter.
	pub fn columns(mut self, columns: Vec<String>) -> SetFilterColumnsBuilder {
		// TODO: Determine the columns input
		self.request.columns = columns;
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.put_ignore(self.request).await
	}
}

impl Jira {
	/// Sets the default columns for the given filter
	pub fn set_filter_columns(&self, id: u64) -> SetFilterColumnsBuilder {
		SetFilterColumnsBuilder::new(Arc::clone(&self.client)).id(id)
	}
}