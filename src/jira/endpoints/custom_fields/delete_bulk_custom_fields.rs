use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, QueryParams}};

#[derive(Debug, Clone)]
pub struct DeleteBulkCustomFieldsBuilder {
	client: Arc<RestClient>,
	request: DeleteBulkCustomFieldsRequest,
}

#[derive(Debug, Clone, Default)]
struct DeleteBulkCustomFieldsRequest {
	ids: Vec<String>
}

impl Endpoint for DeleteBulkCustomFieldsRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"customFields".into()
	}

	fn parameters(&self) -> crate::web::QueryParams<'_> {
		let mut params = QueryParams::default();
		params.push("ids", self.ids.join(","));
		params
	}
}

impl DeleteBulkCustomFieldsBuilder {
	fn new(client: Arc<RestClient>) -> DeleteBulkCustomFieldsBuilder {
		DeleteBulkCustomFieldsBuilder { client, request: DeleteBulkCustomFieldsRequest::default() }
	}

	/// Add a custom field ID to delete.
	pub fn id(mut self, id: impl Into<String>) -> DeleteBulkCustomFieldsBuilder {
		self.request.ids.push(id.into());
		self
	}

	/// Add multiple custom field IDs to delete.
	pub fn ids<S>(mut self, ids: impl IntoIterator<Item = S>) -> DeleteBulkCustomFieldsBuilder 
	where
		S: Into<String>,
	{
		self.request.ids.extend(ids.into_iter().map(|i| i.into()));
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.delete_ignore(self.request).await
	}
}

impl Jira {
	/// Delete multiple custom fields from the system.
	pub fn delete_bulk_custom_fields(&self) -> DeleteBulkCustomFieldsBuilder {
		DeleteBulkCustomFieldsBuilder::new(Arc::clone(&self.client))
	}
}