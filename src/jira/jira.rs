use std::sync::Arc;

use crate::rest_client::{RestClient, RestClientBuilder};
use crate::Result;

#[derive(Debug, Clone)]
pub struct Jira {
	pub(crate) client: Arc<RestClient>,
}

impl Jira {
	pub fn builder() -> JiraBuilder {
		JiraBuilder::default()
	}
}

#[derive(Debug, Default)]
pub struct JiraBuilder {
	client: RestClientBuilder
}

impl JiraBuilder {
	pub fn url(mut self, url: impl Into<String>) -> JiraBuilder {
		self.client = self.client.url(url.into());
		self
	}

	pub fn username(mut self, username: impl Into<String>) -> JiraBuilder {
		self.client = self.client.username(username.into());
		self
	}

	pub fn password(mut self, password: impl Into<String>) -> JiraBuilder {
		self.client = self.client.password(password.into());
		self
	}

	pub fn timeout(mut self, timeout: u32) -> JiraBuilder {
		self.client = self.client.timeout(timeout);
		self
	}

	pub fn api_root(mut self, api_root: impl Into<String>) -> JiraBuilder {
		self.client = self.client.api_root(api_root.into());
		self
	}

	pub fn api_version(mut self, api_version: impl Into<String>) -> JiraBuilder {
		self.client = self.client.api_version(api_version.into());
		self
	}

	pub fn verify_ssl(mut self, verify_ssl: bool) -> JiraBuilder {
		self.client = self.client.verify_ssl(verify_ssl);
		self
	}

	pub fn session(mut self, session: reqwest::Client) -> JiraBuilder {
		self.client = self.client.session(session);
		self
	}

	pub fn build(self) -> Result<Jira> {
		let client = self.client.build()?;
		Ok(Jira {
			client: Arc::new(client)
		})
	}
}