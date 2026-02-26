use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::Endpoint};

#[derive(Debug, Clone)]
pub struct CreateCustomFieldBuilder {
	client: Arc<RestClient>,
	request: CreateCustomFieldRequest,
}

#[derive(Debug, Clone, Default)]
struct CreateCustomFieldRequest {
	name: String,
	description: Option<String>,
	field_type: String,
	searcher_key: Option<String>,
}

impl Endpoint for CreateCustomFieldRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		"field".into()
	}
}

impl CreateCustomFieldBuilder {
	fn new(client: Arc<RestClient>) -> CreateCustomFieldBuilder {
		CreateCustomFieldBuilder { client, request: CreateCustomFieldRequest::default() }
	}
 
	fn name(mut self, name: impl Into<String>) -> CreateCustomFieldBuilder {
		self.request.name = name.into();
		self
	}

	/// Add a description to the custom field.
	pub fn description(mut self, description: impl Into<String>) -> CreateCustomFieldBuilder {
		self.request.description = Some(description.into());
		self
	}

	fn field_type(mut self, field_type: impl Into<String>) -> CreateCustomFieldBuilder {
		self.request.field_type = field_type.into();
		self
	}

	/// Select a search key for the custom field. Needed for User/Group picker fields.
	pub fn searcher_key(mut self, searcher_key: impl Into<String>) -> CreateCustomFieldBuilder {
		self.request.searcher_key = Some(searcher_key.into());
		self
	}

	pub async fn send(self) -> Result<()> {
		self.client.post(self.request).await
	}
}

impl Jira {
	/// Creates a custom field using a definition.
	pub fn create_custom_field(&self, name: impl Into<String>, field_type: impl Into<String>) -> CreateCustomFieldBuilder {
		CreateCustomFieldBuilder::new(Arc::clone(&self.client)).name(name).field_type(field_type)
	}
}