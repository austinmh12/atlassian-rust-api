use std::sync::Arc;

use crate::{Jira, Result, rest_client::RestClient, web::{Endpoint, JsonFormParams}};

#[derive(Debug, Clone)]
pub struct SetPropertyBuilder {
	client: Arc<RestClient>,
	request: SetPropertyRequest,
}

#[derive(Debug, Clone, Default)]
struct SetPropertyRequest {
	id: String,
	value: String,
}

impl Endpoint for SetPropertyRequest {
	fn endpoint(&self) -> std::borrow::Cow<'static, str> {
		format!("application-properties/{}", self.id.clone()).into()
	}

	fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>> {
		let mut body = JsonFormParams::default();
		body
			.push("id", &self.id)
			.push("value", &self.value);
		body.into_body()
	}
}

impl SetPropertyBuilder {
	fn new(client: Arc<RestClient>) -> SetPropertyBuilder {
		SetPropertyBuilder { client, request: SetPropertyRequest::default() }
	}

	fn id(mut self, id: impl Into<String>) -> SetPropertyBuilder {
		self.request.id = id.into();
		self
	}

	fn value(mut self, value: impl Into<String>) -> SetPropertyBuilder {
		self.request.value = value.into();
		self
	}

	async fn send(self) -> Result<()> {
		self.client.put_ignore(self.request).await
	}
}

crate::macros::futurize!(SetPropertyBuilder);

impl Jira {
	/// Modify an application property via PUT. The "value" field present in the PUT will override the
	/// existing value.
	pub async fn set_property(&self, id: impl Into<String>, value: impl Into<String>) -> Result<()> {
		SetPropertyBuilder::new(Arc::clone(&self.client)).id(id).value(value).await
	}
}